use std::{vec, f64::INFINITY};

use crate::{Shape, ShapeType, test_shape, Tuple, Ray, equal, vector, EPSILON, LocalShape, BoundingBox, point, f64_for_bound};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cylinder {
    pub maximum: f64,
    pub minimum: f64,
    pub closed: bool,
}

impl Shape {
    fn as_cylinder(&self) -> Option<&Cylinder> {
        if let ShapeType::Cylinder(c) = &self.shape_type { Some(c) } else { None }
    }
    fn as_cylinder_mut(&mut self) -> Option<&mut Cylinder> {
        if let ShapeType::Cylinder(c) = &mut self.shape_type { Some(c) } else { None }
    }

    pub fn closed(&self) -> bool {
        if let Some(c) = self.as_cylinder() { c.closed }
        else if let Some(c) = self.as_cone() { c.closed }
        else { panic!() }
    }
    pub fn set_closed(&mut self, closed: &bool) {
        if let Some(c) = self.as_cylinder_mut() { c.closed = *closed; }
        else if let Some(c) = self.as_cone_mut() { c.closed = *closed; }
        else { panic!() }
    }

    pub fn maximum(&self) -> f64 {
        if let Some(c) = self.as_cylinder() { c.maximum }
        else if let Some(c) = self.as_cone() { c.maximum }
        else { panic!() }
    }
    pub fn set_maximum(&mut self, maximum: &f64) {
        if let Some(c) = self.as_cylinder_mut() { c.maximum = *maximum; }
        else if let Some(c) = self.as_cone_mut() { c.maximum = *maximum; }
        else { panic!() }
    }

    pub fn minimum(&self) -> f64 {
        if let Some(c) = self.as_cylinder() { c.minimum }
        else if let Some(c) = self.as_cone() { c.minimum }
        else { panic!() }
    }
    pub fn set_minimum(&mut self, minimum: &f64) {
        if let Some(c) = self.as_cylinder_mut() { c.minimum = *minimum; }
        else if let Some(c) = self.as_cone_mut() { c.minimum = *minimum; }
        else { panic!() }
    }
}

pub fn cylinder() -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::Cylinder(Cylinder { maximum: INFINITY, minimum: -INFINITY, closed: false });
    s
}

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    (x * x + z * z) <= 1.
}

fn intersect_caps(cyl: &Cylinder, ray: &Ray, xs: &mut Vec<f64>) {
    if !cyl.closed || equal(ray.direction.y, 0.) {
        return;
    }
    for bound in [cyl.minimum, cyl.maximum] {
        let t = (bound - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t) {
            xs.push(t);
        }
    }
}

impl LocalShape for Cylinder {
    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        let mut xs = vec![];
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        if equal(a, 0.) {
            intersect_caps(self, ray, &mut xs);
            return xs;
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.;
        let disc = b * b - 4. * a * c;
        if disc < 0. {
            return vec![];
        }
        let mut t0 = (-b - disc.sqrt()) / (2. * a);
        let mut t1 = (-b + disc.sqrt()) / (2. * a);
        if t0 > t1 {
            (t0, t1) = (t1, t0);
        }
        for t in [t0, t1] {
            let y = ray.origin.y + t * ray.direction.y;
            if self.minimum < y && y < self.maximum {
                xs.push(t);
            }
        }
        intersect_caps(self, ray, &mut xs);
        xs
    }

    fn local_normal_at(&self, object_point: &Tuple, _intersection: &crate::Intersection) -> Tuple {
        let dist = object_point.x * object_point.x + object_point.z * object_point.z;
        if dist < 1. && object_point.y >= self.maximum - EPSILON {
            vector(0., 1., 0.)
        } else if dist < 1. && object_point.y <= self.minimum + EPSILON {
            vector(0., -1., 0.)
        } else {
            vector(object_point.x, 0., object_point.z)
        }
    }

    fn local_bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            point(-1., f64_for_bound(self.minimum), -1.),
            point(1., f64_for_bound(self.maximum), 1.),
        )
    }
}
