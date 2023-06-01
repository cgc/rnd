use std::{vec, f64::INFINITY};

use crate::{Shape, ShapeType, test_shape, Tuple, Ray, equal, vector, EPSILON, LocalShape, BoundingBox, point, f64_for_bound};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cone {
    pub maximum: f64,
    pub minimum: f64,
    pub closed: bool,
}

impl Shape {
    pub fn as_cone(&self) -> Option<&Cone> {
        if let ShapeType::Cone(c) = &self.shape_type { Some(c) } else { None }
    }
    pub fn as_cone_mut(&mut self) -> Option<&mut Cone> {
        if let ShapeType::Cone(c) = &mut self.shape_type { Some(c) } else { None }
    }
}

pub fn cone() -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::Cone(Cone { maximum: INFINITY, minimum: -INFINITY, closed: false });
    s
}

fn intersect_caps(cyl: &Cone, ray: &Ray, xs: &mut Vec<f64>) {
    if !cyl.closed || equal(ray.direction.y, 0.) {
        return;
    }
    for bound in [cyl.minimum, cyl.maximum] {
        let t = (bound - ray.origin.y) / ray.direction.y;
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x * x + z * z < bound.abs() {
            xs.push(t);
        }
    }
}

impl LocalShape for Cone {
    fn local_normal_at(&self, object_point: &Tuple, _intersection: &crate::Intersection) -> Tuple {
        let dist = object_point.x * object_point.x + object_point.z * object_point.z;
        if dist < 1. && object_point.y >= self.maximum - EPSILON {
            vector(0., 1., 0.)
        } else if dist < 1. && object_point.y <= self.minimum + EPSILON {
            vector(0., -1., 0.)
        } else {
            let mut y = dist.sqrt();
            if object_point.y > 0. {
                y = -y;
            }
            vector(object_point.x, y, object_point.z)
        }
    }

    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        let mut xs = vec![];
        let o = ray.origin;
        let d = ray.direction;
        let a = d.x * d.x - d.y * d.y + d.z * d.z;
        let b = 2. * (o.x * d.x - o.y * d.y + o.z * d.z);
        let c = o.x * o.x - o.y * o.y + o.z * o.z;

        if equal(a, 0.) {
            if !equal(b, 0.) {
                xs.push(-c / (2. * b));
            }
            intersect_caps(self, ray, &mut xs);
            return xs;
        }

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

    fn local_bounding_box(&self) -> BoundingBox {
        let extent = f64_for_bound(self.minimum.abs().max(self.maximum));
        BoundingBox::new(
            point(-extent, f64_for_bound(self.minimum), -extent),
            point(extent, f64_for_bound(self.maximum), extent),
        )
    }
}
