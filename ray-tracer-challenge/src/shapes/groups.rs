use core::panic;
use std::{time::Instant};

use crate::{Shape, test_shape, ShapeType, Ray, Tuple, transform, Intersection, LocalShape, BaseBoundingBox, BoundingBox, BVHNode};

#[derive(PartialEq, Debug, Clone)]
pub struct Group {
    pub children: Vec<Shape>,
    pub bb: BoundingBox,
    pub bvh: Option<BVHNode>,
}

impl Shape {
    // HACK: for tests

    pub fn as_group(&self) -> Option<&Group> {
        match &self.shape_type {
            ShapeType::Group(g) => Some(g),
            _ => None,
        }
    }

    fn as_group_mut(&mut self) -> Option<&mut Group> {
        match &mut self.shape_type {
            ShapeType::Group(g) => Some(g),
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children().len() == 0
    }

    pub fn freeze_and_optimize(&mut self) {
        if let Some(g) = self.as_group_mut() {
            // NOTE We should freeze children before adding, since we copy when adding.
            for c in &mut g.children {
                c.freeze_and_optimize();
            }
            let now = Instant::now();
            let bvh = BVHNode::build(g.bb, &g.children);
            let elapsed_time = now.elapsed();
            println!(
                "Built BVH. {} children. Max depth {}. {} seconds.",
                g.children.len(),
                bvh.max_depth(),
                (elapsed_time.as_millis() as f64)/1000.,
            );
            g.bvh = Some(bvh);
        }
    }
}

impl LocalShape for Group {
    fn local_normal_at(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        panic!("Should not be called")
    }

    fn local_intersect(&self, group: &Shape, object_ray: &Ray) -> Vec<Intersection> {
        if let Some(bvh) = &self.bvh {
            return bvh.intersect(object_ray);
        }

        if self.bb.local_intersect_ts(object_ray).len() == 0 {
            return vec![];
        }

        let mut xs = vec![];
        for child in &self.children {
            let t = child.inverse() * group.transform();
            let object_ray = transform(object_ray, &t);
            let mut ts = child.local_intersect(&object_ray);
            xs.append(&mut ts);
        }
        xs
    }

    fn local_bounding_box(&self) -> BoundingBox {
        self.bb
        // let mut gbb = BoundingBox::new_empty();
        // for c in &self.children {
        //     let bb = c.as_local_shape().local_bounding_box();
        //     for p in bb.corners() {
        //         let p2 = c.original_transform * p;
        //         gbb.add_point(&p2);
        //     }
        // }
        // gbb
    }
}

pub fn group() -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::Group(Group {
        children: vec![], bb: BoundingBox::new_empty(), bvh: None });
    s
}

pub fn add_child(shape: &mut Shape, s: &Shape) {
    // Copy our transform here, to avoid ownership issues.
    let t = shape.transform().clone();

    let g = shape.as_group_mut().unwrap();
    assert!(g.bvh.is_none()); // Avoid changing when bvh built.
    let mut s = s.clone();

    // Set parent transform and recompute transform
    s.parent_transform = t;
    s.recompute_transform();

    // Add to group's bounding box. We keep the bounding box in the local space of the group.
    g.bb.include_transformed_shape(&s);

    // Move our local copy to group.
    g.children.push(s);
}

mod test {
    pub use crate::{sphere, add_child, group};
    #[test]
    fn test_group_equality() {
        /*
        Wrote this to debug another unit test, but this turned out to not be the issue.
        */
        let mut s = sphere();
        let mut g1 = group();
        let mut g2 = group();

        add_child(&mut g1, &mut s);
        assert!(s == g1.children()[0]);

        add_child(&mut g2, &mut s);
        assert!(s == g2.children()[0]);

        assert!(g1.children()[0] == g2.children()[0]);

        assert!(g1 == g2);
    }
}
