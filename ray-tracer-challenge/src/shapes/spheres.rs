use crate::BoundingBox;
use crate::Intersection;
use crate::LocalShape;
use crate::Ray;
use crate::Shape;
use crate::ShapeType;
use crate::Tuple;
use crate::dot;
use crate::material;
use crate::point;
use crate::test_shape;

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {}

impl LocalShape for Sphere {
    fn local_normal_at(&self, object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        *object_point - point(0., 0., 0.)
    }
    fn local_intersect_ts(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - point(0., 0., 0.);
        let a = dot(&ray.direction, &ray.direction);
        let b = 2. * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2. * a),
                (-b + discriminant.sqrt()) / (2. * a),
            ]
        }
    }
    fn local_bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            point(-1., -1., -1.),
            point(1., 1., 1.),
        )
    }
}

pub fn sphere() -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::Sphere(Sphere {});
    s
}

pub fn glass_sphere() -> Shape {
    let mut material = material();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    let mut s = sphere();
    s.material = material;
    s
}
