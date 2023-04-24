use crate::{Tuple, Matrix};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub fn ray(origin: &Tuple, direction: &Tuple) -> Ray {
    assert!(!origin.is_vector()); // Just making sure it's non-zero
    assert!(direction.is_vector());
    Ray { origin: *origin, direction: *direction }
}

pub fn position(ray: &Ray, t: f64) -> Tuple {
    ray.origin + ray.direction * t
}

pub fn transform(r: &Ray, transform: &Matrix) -> Ray {
    ray(&(*transform * r.origin), &(*transform * r.direction))
}
