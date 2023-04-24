use std::vec;

use crate::BoundingBox;
use crate::Intersection;
use crate::LocalShape;
use crate::Ray;
use crate::Shape;
use crate::ShapeType;
use crate::Tuple;
use crate::equal;
use crate::test_shape;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,
    pub e1: Tuple,
    pub e2: Tuple,
    pub normal: Tuple,
}

impl LocalShape for Triangle {
    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
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
        vec![t]
    }

    fn local_normal_at(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        self.normal
    }

    fn local_bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(&[self.p1, self.p2, self.p3])
    }
}

impl Shape {
    // HACK: for tests

    fn as_triangle(&self) -> Option<Triangle> {
        match &self.shape_type {
            ShapeType::Triangle(t) => Some(*t),
            _ => None,
        }
    }

    pub fn p1(&self) -> Tuple {
        if let Some(t) = self.as_triangle() { t.p1 }
        else { self.as_smooth_triangle().unwrap().p1 }
    }
    pub fn p2(&self) -> Tuple {
        if let Some(t) = self.as_triangle() { t.p2 }
        else { self.as_smooth_triangle().unwrap().p2 }
    }
    pub fn p3(&self) -> Tuple {
        if let Some(t) = self.as_triangle() { t.p3 }
        else { self.as_smooth_triangle().unwrap().p3 }
    }

    pub fn e1(&self) -> Tuple { self.as_triangle().unwrap().e1 }
    pub fn e2(&self) -> Tuple { self.as_triangle().unwrap().e2 }
    pub fn normal(&self) -> Tuple { self.as_triangle().unwrap().normal }
}

pub fn triangle(
    p1: &Tuple,
    p2: &Tuple,
    p3: &Tuple,
) -> Shape {
    let mut s = test_shape();
    let e1 = *p2 - *p1;
    let e2 = *p3 - *p1;
    s.shape_type = ShapeType::Triangle(Triangle {
        p1: *p1,
        p2: *p2,
        p3: *p3,
        e1,
        e2,
        normal: e2.cross(&e1).normalized(),
    });
    s
}
