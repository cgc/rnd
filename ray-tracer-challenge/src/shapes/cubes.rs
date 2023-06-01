use std::{f64::INFINITY};

use crate::{Shape, ShapeType, test_shape, Tuple, Ray, EPSILON, vector, LocalShape, point, BoundingBox, BaseBoundingBox};

#[derive(PartialEq, Debug, Clone)]
pub struct Cube {}

impl BaseBoundingBox for Cube {
    fn bounds(&self) -> (Tuple, Tuple) {
        (
            point(-1., -1., -1.),
            point(1., 1., 1.),
        )
    }
}

impl LocalShape for Cube {
    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        (self as &dyn BaseBoundingBox).local_intersect_ts(ray)
    }

    fn local_normal_at(&self, object_point: &Tuple, _intersection: &crate::Intersection) -> Tuple {
        let max = object_point.x.abs().max(object_point.y.abs()).max(object_point.z.abs());
        if max == object_point.x.abs() {
            vector(object_point.x, 0., 0.)
        } else if max == object_point.y.abs() {
            vector(0., object_point.y, 0.)
        } else {
            vector(0., 0., object_point.z)
        }
    }

    fn local_bounding_box(&self) -> BoundingBox {
        let (min, max) = self.bounds();
        BoundingBox::new(min, max)
    }
}

pub fn cube() -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::Cube(Cube {});
    s
}

pub fn check_axis(origin: f64, direction: f64, bounds: (f64, f64)) -> (f64, f64) {
    let (low, high) = bounds;
    let tmin_numerator = low - origin;
    let tmax_numerator = high - origin;

    let mut tmin;
    let mut tmax;
    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * INFINITY;
        tmax = tmax_numerator * INFINITY;
    }

    if tmin > tmax {
        (tmin, tmax) = (tmax, tmin);
    }

    (tmin, tmax)
}
