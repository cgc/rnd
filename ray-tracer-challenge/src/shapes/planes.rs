use std::{vec, f64::{NEG_INFINITY, INFINITY}};

use crate::{Shape, ShapeType, test_shape, Tuple, Ray, EPSILON, vector, LocalShape, Intersection, BoundingBox, point, f64_for_bound};

#[derive(PartialEq, Debug, Clone)]
pub struct Plane {}

impl LocalShape for Plane {
    fn local_normal_at(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        vector(0., 1., 0.)
    }
    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }
        vec![-ray.origin.y / ray.direction.y]
    }
    fn local_bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            point(f64_for_bound(NEG_INFINITY), 0., f64_for_bound(NEG_INFINITY)),
            point(f64_for_bound(INFINITY), 0., f64_for_bound(INFINITY)),
        )
    }
}


pub fn plane() -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::Plane(Plane {});
    s
}
