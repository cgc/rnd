use std::f64::{INFINITY, NEG_INFINITY};

use crate::{Shape, Ray, Tuple, transform, Intersection, point, check_axis};

// https://forum.raytracerchallenge.com/post/401/thread
pub const INFINITY_FOR_BOUNDS: f64 = f64::MAX / 2.;

pub fn f64_for_bound(a: f64) -> f64 {
    if a == INFINITY {
        INFINITY_FOR_BOUNDS
    } else if a == NEG_INFINITY {
        -INFINITY_FOR_BOUNDS
    } else {
        a
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum BVHNodeType {
    Leaf(Vec<Shape>),
    Internal(BVHLink, BVHLink),
}

type BVHLink = Box<BVHNode>;

#[derive(PartialEq, Debug, Clone)]
pub struct BVHNode {
    pub bb: BoundingBox,
    pub ntype: BVHNodeType,
}

impl BVHNode {
    pub fn child_nodes(&self) -> Option<(&BVHNode, &BVHNode)> {
        match &self.ntype {
            BVHNodeType::Leaf(_) => None,
            BVHNodeType::Internal(left, right) => Some((left, right)),
        }
    }

    pub fn shapes(&self) -> Option<&Vec<Shape>> {
        match &self.ntype {
            BVHNodeType::Leaf(shapes) => Some(shapes),
            BVHNodeType::Internal(_, _) => None,
        }
    }

    pub fn max_depth(&self) -> usize {
        match &self.ntype {
            BVHNodeType::Leaf(_) => 1,
            BVHNodeType::Internal(left, right) => left.max_depth().max(right.max_depth()) + 1,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut rv = vec![];
        self._intersect(ray, &mut rv);
        rv
    }

    pub fn _intersect<'a>(&'a self, ray: &Ray, xs: &mut Vec<Intersection<'a>>) {
        // TODO Implement early exit; inspect ts to see if we can rule this out.
        // NOTE Need to be careful with refraction / CSG with early exit, ensuring it's under a flag.
        if self.bb.local_intersect_ts(ray).len() == 0 {
            return;
        }
        match &self.ntype {
            BVHNodeType::Leaf(shapes) => {
                for child in shapes {
                    // HACK: this loop is basically copy/pasted from group's intersect.
                    let t = child.inverse() * child.parent_transform;
                    let object_ray = transform(ray, &t);
                    let mut ts = child.local_intersect(&object_ray);
                    xs.append(&mut ts);
                }
            }
            BVHNodeType::Internal(left, right) => {
                // TODO early exit; Order based on closest, only recurse in each if viable intersection with bound
                left._intersect(ray, xs);
                right._intersect(ray, xs);
            }
        }
    }

    pub fn build(bb: BoundingBox, shapes: &Vec<Shape>) -> BVHNode {
        // Implementation closely follows PBR 3rd Ed
        // https://pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#TheSurfaceAreaHeuristic
        const DIVISIONS: usize = 12;
        const RELATIVE_COST_TRAVERSAL: f64 = 0.125;

        if shapes.len() < 4 {
            return BVHNode {
                bb,
                ntype: BVHNodeType::Leaf(shapes.clone())
            }
        }

        // The case where we don't split
        // This cost is a simplified version of below equation for cost assuming no traversal; surface area of bb with itself cancels out.
        let mut mincost = shapes.len() as f64;
        let mut minlists = (0, 0, bb, BoundingBox::new_empty());

        // Init buckets
        let mut allbuckets: [[(BoundingBox, Vec<&Shape>); DIVISIONS]; 3] = core::array::from_fn(|_dim|
            core::array::from_fn(|_bucket| (BoundingBox::new_empty(), vec![])));

        for (dim, buckets) in allbuckets.iter_mut().enumerate() {
            // Fill buckets
            let (min, max) = (bb.min[dim], bb.max[dim]);
            let extent = max - min;
            for s in shapes {
                let centroid = s.as_local_shape().local_bounding_box().centroid();
                // This is ok b/c of linearity?
                let c = s.original_transform * centroid;
                let idx = ((c[dim] - min) / extent * DIVISIONS as f64).floor() as usize;
                let (bb, v) = &mut buckets[idx];
                v.push(s);
                bb.include_transformed_shape(s);
            }

            // Try all partitions of buckets.
            // We start at index 1 to avoid an empty one.
            for split_idx in 1..DIVISIONS {
                let mut lbb = BoundingBox::new_empty();
                let mut rbb = BoundingBox::new_empty();
                let mut lcount = 0;
                let mut rcount = 0;
                for (b, ct, rng) in [
                    (&mut lbb, &mut lcount, 0..split_idx),
                    (&mut rbb, &mut rcount, split_idx..DIVISIONS),
                ] {
                    for idx in rng {
                        let (bb, v) = &buckets[idx];
                        b.include_bb(bb);
                        *ct += v.len();
                    }
                }
                let cost = RELATIVE_COST_TRAVERSAL + (
                    lcount as f64 * lbb.surface_area() +
                    rcount as f64 * rbb.surface_area()) / bb.surface_area();
                if cost < mincost {
                    mincost = cost;
                    minlists = (dim, split_idx, lbb, rbb);
                }
            }
        }

        let (dim, split_idx, lbb, rbb) = minlists;

        if split_idx == 0 {
            // Not worth recursing.
            return BVHNode {
                bb,
                ntype: BVHNodeType::Leaf(shapes.clone()),
            }
        } else {
            let mut left: Vec<Shape> = vec![];
            let mut right = vec![];
            for (v, rng) in [
                (&mut left, 0..split_idx),
                (&mut right, split_idx..DIVISIONS),
            ] {
                for idx in rng {
                    v.extend(allbuckets[dim][idx].1.iter().map(|s| (*s).clone()));
                }
            }
            // Recursively build tree.
            let left = BVHNode::build(lbb, &left);
            let right = BVHNode::build(rbb, &right);

            BVHNode {
                bb,
                ntype: BVHNodeType::Internal(
                    Box::new(left),
                    Box::new(right),
                ),
            }
        }
    }
}


pub trait BaseBoundingBox {
    fn bounds(&self) -> (Tuple, Tuple);

    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        let (bmin, bmax) = self.bounds();
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x, (bmin.x, bmax.x));
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y, (bmin.y, bmax.y));
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z, (bmin.z, bmax.z));

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Tuple,
    pub max: Tuple,
}
impl BoundingBox {
    pub fn new(min: Tuple, max: Tuple) -> BoundingBox {
        assert!(min.is_point());
        assert!(max.is_point());
        BoundingBox { min, max }
    }

    pub fn new_empty() -> BoundingBox {
        let min = point(INFINITY, INFINITY, INFINITY);
        let max = point(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);
        BoundingBox::new(min, max)
    }

    pub fn add_point(&mut self, point: &Tuple) {
        for dim in 0..3 {
            let el = point[dim];
            if el < self.min[dim] {
                self.min[dim] = el;
            }
            if el > self.max[dim] {
                self.max[dim] = el;
            }
        }
    }

    pub fn from_points(points: &[Tuple]) -> BoundingBox {
        let mut bb = BoundingBox::new_empty();
        for p in points {
            bb.add_point(p);
        }
        bb
    }

    pub fn from_transformed_shapes(shapes: &[&Shape]) -> BoundingBox {
        let mut bb = BoundingBox::new_empty();
        for s in shapes {
            bb.include_transformed_shape(s);
        }
        bb
    }

    pub fn corners(&self) -> [Tuple; 8] {
        let mut corners = [point(0., 0., 0.); 8];
        let bounds = [self.min, self.max];
        for i in 0..8 {
            for dim in 0..3 {
                let mask = 1 << dim;
                let idx = if i & mask == 0 { 0 } else { 1 };
                corners[i][dim] = bounds[idx][dim];
            }
        }
        corners
    }

    pub fn include_transformed_shape(&mut self, s: &Shape) {
        let bb = s.as_local_shape().local_bounding_box();
        let transform = &s.original_transform;
        for p in bb.corners() {
            let p2 = *transform * p;
            self.add_point(&p2);
        }
    }

    pub fn include_bb(&mut self, bb: &BoundingBox) {
        for dim in 0..3 {
            if bb.min[dim] < self.min[dim] {
                self.min[dim] = bb.min[dim];
            }
            if bb.max[dim] > self.max[dim] {
                self.max[dim] = bb.max[dim];
            }
        }
    }

    pub fn centroid(&self) -> Tuple {
        point(
            (self.min.x + self.max.x) / 2.,
            (self.min.y + self.max.y) / 2.,
            (self.min.z + self.max.z) / 2.,
        )
    }

    pub fn surface_area(&self) -> f64 {
        let dx = self.max.x - self.min.x;
        let dy = self.max.y - self.min.y;
        let dz = self.max.z - self.min.z;
        if dx < 0. || dy < 0. || dz < 0. {
            return 0.
        }
        2. * (dx * dy + dy * dz + dz * dx)
    }
}

impl BaseBoundingBox for BoundingBox {
    fn bounds(&self) -> (Tuple, Tuple) {
        (self.min, self.max)
    }
}
