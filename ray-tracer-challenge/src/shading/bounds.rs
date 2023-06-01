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

pub enum BVHTraversalPolicy<'i> {
    ClosestHit { x: Option<Intersection<'i>> },
    // TODO to make this useful for shadowing, we need to take a maximum value here.
    AnyHit { x: Option<Intersection<'i>> },
    AllHits { xs: Vec<Intersection<'i>> },
}

impl<'i> BVHTraversalPolicy<'i> {
    pub fn new_any_hit() -> BVHTraversalPolicy<'i> {
        BVHTraversalPolicy::AnyHit { x: None }
    }
    pub fn new_all_hits() -> BVHTraversalPolicy<'i> {
        BVHTraversalPolicy::AllHits { xs: vec![] }
    }
    pub fn new_closest_hit() -> BVHTraversalPolicy<'i> {
        BVHTraversalPolicy::ClosestHit { x: None }
    }

    pub fn should_traverse(&self, bb: &BoundingBox, ray: &Ray) -> (bool, f64) {
        match self {
            BVHTraversalPolicy::ClosestHit { x } => {
                let (bbhit, tmin, tmax) = bb.local_intersect_tmin_tmax(ray);
                let current_t = x.map_or(INFINITY, |x| x.t);
                return (bbhit && tmin <= current_t && 0. <= tmax, tmin);
            },
            BVHTraversalPolicy::AnyHit { x } => {
                if x.is_some() {
                    return (false, NEG_INFINITY);
                }
            },
            BVHTraversalPolicy::AllHits { xs: _ } => {},
        }
        let (bbhit, tmin, _tmax) = bb.local_intersect_tmin_tmax(ray);
        (bbhit, tmin)
    }

    pub fn add_intersections<'a>(&'a mut self, new_xs: &mut Vec<Intersection<'i>>) {
        match self {
            BVHTraversalPolicy::ClosestHit { x } => {
                let mut current_t = x.map_or(INFINITY, |x| x.t);
                for new_x in new_xs {
                    if 0. <= new_x.t && new_x.t < current_t {
                        *x = Some(*new_x);
                        current_t = new_x.t;
                    }
                }
            },
            BVHTraversalPolicy::AnyHit { x } => {
                for new_x in new_xs {
                    if 0. <= new_x.t {
                        *x = Some(*new_x);
                    }
                }
            }
            BVHTraversalPolicy::AllHits { xs } => {
                xs.append(new_xs);
            }
        }
    }

    pub fn intersections<'a>(&'a self) -> Vec<Intersection<'i>> {
        match self {
            BVHTraversalPolicy::ClosestHit { x } => {
                if let Some(i) = x { vec![*i] } else { vec![] }
            }
            BVHTraversalPolicy::AnyHit { x } => {
                if let Some(i) = x { vec![*i] } else { vec![] }
            }
            BVHTraversalPolicy::AllHits { xs } => {
                xs.clone()
            }
        }
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

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut p = BVHTraversalPolicy::new_all_hits();
        self._do_intersect(ray, &mut p)
    }

    pub fn intersect_closest<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut p = BVHTraversalPolicy::new_closest_hit();
        self._do_intersect(ray, &mut p)
    }

    pub fn intersect_any<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut p = BVHTraversalPolicy::new_any_hit();
        self._do_intersect(ray, &mut p)
    }

    pub fn _do_intersect<'a>(&'a self, ray: &Ray, p: &mut BVHTraversalPolicy<'a>) -> Vec<Intersection<'a>> {
        // Since we check node bbox right before recursive traversal in _intersect, we have this special
        // case for the root node, to make sure we check the root node's bbox before starting traversal.
        // We check node bbox in the Internal arm because it lets us reorder arms for ClosestHit
        if p.should_traverse(&self.bb, ray).0 {
            self._intersect(ray, p);
        }
        p.intersections()
    }

    pub fn _intersect<'a>(&'a self, ray: &Ray, p: &mut BVHTraversalPolicy<'a>) {
        match &self.ntype {
            BVHNodeType::Leaf(shapes) => {
                for child in shapes {
                    // HACK: this loop was originally copy/pasted from group's intersect.
                    let t = child.local_inverse();
                    let object_ray = transform(ray, &t);
                    // TODO Can optimize this further for AnyHit by bailing out if there's a match here. Potentially via should_traverse
                    let mut ts = match p {
                        BVHTraversalPolicy::ClosestHit { x: _ } => child.local_intersect_closest_hit(&object_ray),
                        BVHTraversalPolicy::AnyHit { x: _ } => child.local_intersect_any_hit(&object_ray),
                        BVHTraversalPolicy::AllHits { xs: _ } => child.local_intersect(&object_ray),
                    };
                    p.add_intersections(&mut ts);
                }
            }
            BVHNodeType::Internal(left, right) => {
                let (l_bbhit, l_tmin) = p.should_traverse(&left.bb, ray);
                let (r_bbhit, r_tmin) = p.should_traverse(&right.bb, ray);

                // We handle this case separately for simplicity. If we want all hits,
                // then we should recurse into all viable subtrees.
                if let BVHTraversalPolicy::AllHits { xs: _ } = p {
                    if l_bbhit { left._intersect(ray, p) }
                    if r_bbhit { right._intersect(ray, p) }
                    return
                }

                // We try to optimize the order we visit nodes here.
                if l_bbhit && r_bbhit {
                    let (mut left, mut right) = (left, right);
                    // Reorder to traverse closer box first.
                    if l_tmin > r_tmin {
                        (left, right) = (right, left)
                    }
                    left._intersect(ray, p);
                    // Now that we've checked the left tree, we double check to make sure the right tree is still relevant.
                    // If we had a collision in the closer tree, that may rule out the need to check the other.
                    if p.should_traverse(&right.bb, ray).0 {
                        right._intersect(ray, p);
                    }
                } else if l_bbhit {
                    // If we only have a bbox hit on one side or the other, we just visit those here.
                    left._intersect(ray, p);
                } else if r_bbhit {
                    right._intersect(ray, p);
                }
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
                let c = s.local_to_parent_transform * centroid;
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

    fn local_intersect_tmin_tmax(&self, ray: &Ray) -> (bool, f64, f64) {
        let (bmin, bmax) = self.bounds();
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x, (bmin.x, bmax.x));
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y, (bmin.y, bmax.y));
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z, (bmin.z, bmax.z));

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        (!(tmin > tmax), tmin, tmax)
    }

    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        let (hit, tmin, tmax) = self.local_intersect_tmin_tmax(ray);
        if hit {
            vec![tmin, tmax]
        } else {
            vec![]
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
        let transform = &s.local_to_parent_transform;
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
