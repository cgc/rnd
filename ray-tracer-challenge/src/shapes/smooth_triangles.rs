use std::vec;

use crate::BoundingBox;
use crate::Intersection;
use crate::LocalShape;
use crate::Ray;
use crate::Shape;
use crate::ShapeType;
use crate::Tuple;
use crate::equal;
use crate::intersection_with_uv;
use crate::test_shape;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SmoothTriangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,

    pub n1: Tuple,
    pub n2: Tuple,
    pub n3: Tuple,
}

impl LocalShape for SmoothTriangle {
    fn local_intersect<'a>(&'a self, shape: &'a Shape, ray: &Ray) -> Vec<Intersection> {
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);
        if equal(det, 0.) {
            return vec![];
        }
        let f = 1. / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if !(0. <= u && u <= 1.) {
            return vec![];
        }
        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * ray.direction.dot(&origin_cross_e1);
        if !(0. <= v && u + v <= 1.) {
            return vec![];
        }
        let t = f * self.e2.dot(&origin_cross_e1);
        vec![intersection_with_uv(t, shape, u, v)]
    }

    fn local_normal_at(&self, _object_point: &Tuple, intersection: &Intersection) -> Tuple {
        self.n2 * intersection.u +
        self.n3 * intersection.v +
        self.n1 * (1. - intersection.u - intersection.v)
    }

    fn local_bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }
}

impl Shape {
    // HACK: for tests

    pub fn as_smooth_triangle(&self) -> Option<SmoothTriangle> {
        match &self.shape_type {
            ShapeType::SmoothTriangle(t) => Some(*t),
            _ => None,
        }
    }

    pub fn n1(&self) -> Tuple { self.as_smooth_triangle().unwrap().n1 }
    pub fn n2(&self) -> Tuple { self.as_smooth_triangle().unwrap().n2 }
    pub fn n3(&self) -> Tuple { self.as_smooth_triangle().unwrap().n3 }
}

pub fn smooth_triangle(
    p1: &Tuple,
    p2: &Tuple,
    p3: &Tuple,
    n1: &Tuple,
    n2: &Tuple,
    n3: &Tuple,
) -> Shape {
    let mut s = test_shape();
    let e1 = *p2 - *p1;
    let e2 = *p3 - *p1;
    s.shape_type = ShapeType::SmoothTriangle(SmoothTriangle {
        p1: *p1,
        p2: *p2,
        p3: *p3,
        e1,
        e2,
        n1: *n1,
        n2: *n2,
        n3: *n3,
    });
    s
}
