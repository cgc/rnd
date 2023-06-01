use core::panic;
use std::{f64::INFINITY};

use crate::{Matrix, Material, Tuple, Intersections, Ray, inverse, transpose, normalize, point, transform, intersection, Intersection, ray, identity_matrix, material, vector, Cylinder, Cone, Group, intersections, Triangle, SmoothTriangle, Cube, Sphere, Plane, BoundingBox, CSG};

pub trait LocalShape {
    fn local_normal_at(&self, object_point: &Tuple, intersection: &Intersection) -> Tuple;
    fn local_intersect_ts(&self, _ray: &Ray) -> Vec<f64> {
        panic!("Must either implement local_intersect_ts or local_intersect")
    }
    fn local_intersect<'a>(&'a self, shape: &'a Shape, ray: &Ray) -> Vec<Intersection> {
        let ts = self.local_intersect_ts(ray);
        ts.into_iter().map(|t| intersection(t, shape)).collect()
    }
    fn local_bounding_box(&self) -> BoundingBox;

    // These are special cases, meant to make it easy to use different BVH traversal.
    // Only implemented by group, which manages BVH
    fn local_intersect_closest_hit<'a>(&'a self, shape: &'a Shape, ray: &Ray) -> Vec<Intersection> { self.local_intersect(shape, ray) }
    fn local_intersect_any_hit<'a>(&'a self, shape: &'a Shape, ray: &Ray) -> Vec<Intersection> { self.local_intersect(shape, ray) }
}

#[derive(PartialEq, Debug, Clone)]
pub struct TestShape {}
impl LocalShape for TestShape {
    fn local_normal_at(&self, object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        vector(object_point.x, object_point.y, object_point.z)
    }
    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        vec![ray.origin.x, ray.origin.y, ray.origin.z, ray.direction.x, ray.direction.y, ray.direction.z]
    }
    fn local_bounding_box(&self) -> BoundingBox {
        BoundingBox::new_empty()
    }
}

pub fn unpack_saved_ray(is: &Intersections) -> Ray {
    ray(
        &point(is.data[0].t, is.data[1].t, is.data[2].t),
        &vector(is.data[3].t, is.data[4].t, is.data[5].t),
    )
}

#[derive(PartialEq, Debug, Clone)]
pub enum ShapeType {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
    Triangle(Triangle),
    SmoothTriangle(SmoothTriangle),
    CSG(CSG),
    TestShape(TestShape),
}

impl ShapeType {
    pub fn as_local_shape(&self) -> &dyn LocalShape {
        match &self {
            ShapeType::Sphere(s) => s,
            ShapeType::TestShape(s) => s,
            ShapeType::Plane(s) => s,
            ShapeType::Cube(s) => s,
            ShapeType::Group(s) => s,
            ShapeType::Cone(s) => s,
            ShapeType::Cylinder(s) => s,
            ShapeType::Triangle(s) => s,
            ShapeType::SmoothTriangle(s) => s,
            ShapeType::CSG(s) => s,
        }
    }

    pub fn local_normal_at(&self, object_point: &Tuple, intersection: &Intersection) -> Tuple {
        self.as_local_shape().local_normal_at(object_point, intersection)
    }

}

#[derive(Debug, Clone)]
pub struct Shape {
    pub material: Material,
    pub shape_type: ShapeType,
    pub shadow: bool,
    pub local_to_parent_transform: Matrix,
    pub parent_to_global_transform: Matrix,
    // These values are cached / recomputed
    cached_transform: Matrix,
    cached_inverse: Matrix,
    cached_local_inverse: Matrix,
}

impl Shape {
    pub fn transform(&self) -> Matrix { self.cached_transform }
    pub fn inverse(&self) -> Matrix { self.cached_inverse }
    pub fn local_transform(&self) -> Matrix { self.local_to_parent_transform }
    pub fn local_inverse(&self) -> Matrix { self.cached_local_inverse }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.local_to_parent_transform = *m;
        self.recompute_transform();
    }
    pub fn recompute_transform(&mut self) {
        self.cached_local_inverse = inverse(&self.local_to_parent_transform);
        self.cached_transform = self.parent_to_global_transform * self.local_to_parent_transform;
        self.cached_inverse = inverse(&self.cached_transform);
        if let ShapeType::Group(group) = &mut self.shape_type {
            for mut c in &mut group.children {
                c.parent_to_global_transform = self.cached_transform;
                c.recompute_transform();
            }
        }
    }

    pub fn local_from_parent_ray(&self, parent_ray: &Ray) -> Ray {
        transform(parent_ray, &self.local_inverse())
    }

    pub fn local_intersect(&self, object_ray: &Ray) -> Vec<Intersection> {
        self.shape_type.as_local_shape().local_intersect(self, object_ray)
    }
    pub fn intersect(&self, parent_ray: &Ray) -> Vec<Intersection> {
        self.local_intersect(&self.local_from_parent_ray(parent_ray))
    }

    pub fn local_intersect_closest_hit(&self, object_ray: &Ray) -> Vec<Intersection> {
        self.shape_type.as_local_shape().local_intersect_closest_hit(self, object_ray)
    }
    pub fn intersect_closest_hit(&self, parent_ray: &Ray) -> Vec<Intersection> {
        self.local_intersect_closest_hit(&self.local_from_parent_ray(parent_ray))
    }

    pub fn local_intersect_any_hit(&self, object_ray: &Ray) -> Vec<Intersection> {
        self.shape_type.as_local_shape().local_intersect_any_hit(self, object_ray)
    }
    pub fn intersect_any_hit(&self, parent_ray: &Ray) -> Vec<Intersection> {
        self.local_intersect_any_hit(&self.local_from_parent_ray(parent_ray))
    }

    pub fn as_local_shape(&self) -> &dyn LocalShape {
        self.shape_type.as_local_shape()
    }

    pub fn includes(&self, query: &Shape) -> bool {
        if self == query {
            return true;
        }

        if let Some(c) = self.children_option() {
            for s in c {
                if s.includes(query) {
                    return true;
                }
            }
        }

        false
    }

    pub fn children(&self) -> &[Shape] {
        self.children_option().unwrap()
    }

    pub fn children_option(&self) -> Option<&[Shape]> {
        if let Some(g) = self.as_group() {
            Some(&g.children)
        } else if let Some(csg) = self.as_csg() {
            Some(&csg.children)
        } else {
            None
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        // NOTE We avoid checking transform or inverse, since those might be updated by a parent.
        self.material == other.material &&
        self.shape_type == other.shape_type &&
        self.local_to_parent_transform == other.local_to_parent_transform
    }
}

pub fn world_to_object(shape: &Shape, point: &Tuple) -> Tuple {
    shape.inverse() * *point
}
pub fn normal_to_world(shape: &Shape, vector: &Tuple) -> Tuple {
    let mut world_normal = transpose(&shape.inverse()) * *vector;
    world_normal.w = 0.;
    normalize(&world_normal)
}

pub fn test_shape() -> Shape {
    Shape {
        parent_to_global_transform: identity_matrix,
        local_to_parent_transform: identity_matrix,
        cached_transform: identity_matrix,
        cached_inverse: identity_matrix,
        cached_local_inverse: identity_matrix,
        shadow: true,
        material: material(),
        shape_type: ShapeType::TestShape(TestShape { }),
    }
}

pub fn set_transform(s: &mut Shape, t: &Matrix) {
    s.set_transform(t);
}

pub fn local_normal_at(shape: &Shape, object_point: &Tuple) -> Tuple {
    // only used by tests
    let fake = intersection(INFINITY, shape);
    shape.shape_type.local_normal_at(object_point, &fake)
}

pub fn local_intersect<'a>(shape: &'a Shape, ray: &Ray) -> Intersections<'a> {
    // Basically only used by test routines.
    let xs = shape.local_intersect(ray);
    intersections(xs) // Call this to ensure sorted for tests.
}

pub fn normal_at2(shape: &Shape, world_point: &Tuple) -> Tuple {
    // only used by tests
    let fake = intersection(INFINITY, shape);
    normal_at3(shape, world_point, &fake)
}

pub fn normal_at3(shape: &Shape, world_point: &Tuple, intersection: &Intersection) -> Tuple {
    let object_point = world_to_object(shape, world_point);
    let object_normal = shape.shape_type.local_normal_at(&object_point, intersection);
    normal_to_world(shape, &object_normal)
}

pub fn intersect<'a>(shape: &'a Shape, world_ray: &Ray) -> Intersections<'a> {
    let xs = shape.intersect(world_ray);
    // NOTE We construct explicitly here to avoid a needless sort.
    Intersections { count: xs.len(), data: xs }
}
