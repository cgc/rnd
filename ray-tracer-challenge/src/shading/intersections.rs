use std::cmp;
use std::ops;
use crate::EPSILON;
use crate::Ray;
use crate::Shape;
use crate::Tuple;
use crate::dot;
use crate::normal_at3;
use crate::position;
use crate::reflect;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
    // Only used by triangles.
    pub u: f64,
    pub v: f64,
}

impl cmp::PartialEq<f64> for Intersection<'_> {
    fn eq(&self, other: &f64) -> bool {
        self.t == *other
    }
}

pub fn intersection(t: f64, object: &Shape) -> Intersection {
    intersection_with_uv(t, object, 0., 0.)
}

pub fn intersection_with_uv(t: f64, object: &Shape, u: f64, v: f64) -> Intersection {
    Intersection { t, object, u, v }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersections<'a> {
    pub count: usize,
    pub data: Vec<Intersection<'a>>,
}

impl Intersections<'_> {
    pub fn hit(&self) -> Option<Intersection> {
        self.data.iter().find(|i| i.t > 0.).cloned()
    }
    pub fn hit_for_shadow(&self) -> Option<Intersection> {
        self.data.iter().find(|i| i.t > 0. && i.object.shadow).cloned()
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<'a> ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub fn intersections(is: Vec<Intersection>) -> Intersections {
    let mut is = is.clone();
    is.sort_by(|a, b|
        a.t.partial_cmp(&b.t).unwrap_or(cmp::Ordering::Equal));
    Intersections {
        count: is.len(),
        data: is,
    }
}

pub fn intersections1<'a>(i1: Intersection<'a>) -> Intersections<'a> {
    intersections(vec![i1])
}
pub fn intersections2<'a>(i1: Intersection<'a>, i2: Intersection<'a>) -> Intersections<'a> {
    intersections(vec![i1, i2])
}
pub fn intersections3<'a>(i1: Intersection<'a>, i2: Intersection<'a>, i3: Intersection<'a>) -> Intersections<'a> {
    intersections(vec![i1, i2, i3])
}
pub fn intersections4<'a>(i1: Intersection<'a>, i2: Intersection<'a>, i3: Intersection<'a>, i4: Intersection<'a>) -> Intersections<'a> {
    intersections(vec![i1, i2, i3, i4])
}
pub fn intersections6<'a>(i1: Intersection<'a>, i2: Intersection<'a>, i3: Intersection<'a>, i4: Intersection<'a>, i5: Intersection<'a>, i6: Intersection<'a>) -> Intersections<'a> {
    intersections(vec![i1, i2, i3, i4, i5, i6])
}

pub fn hit<'a>(is: &'a Intersections) -> Option<Intersection<'a>> {
    is.hit()
}

#[derive(Debug)]
pub struct C<'a> {
    pub t: f64,
    pub object: &'a Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl C<'_> {
    pub fn n1(&self) -> f64 { self.n1 }
    pub fn n2(&self) -> f64 { self.n2 }
}

pub fn prepare_computations2<'a>(intersection: &'a Intersection, ray: &Ray) -> C<'a> {
    prepare_computations3(intersection, ray, &intersections(vec![intersection.clone()]))
}

pub fn prepare_computations3<'a>(intersection: &'a Intersection, ray: &Ray, intersections: &Intersections) -> C<'a> {
    let point = position(ray, intersection.t);
    let mut normalv = normal_at3(&intersection.object, &point, intersection);
    let eyev = -ray.direction;
    let inside;
    if dot(&eyev, &normalv) < 0. {
        inside = true;
        normalv = -normalv;
    } else {
        inside = false;
    }
    let over_point = point + normalv * EPSILON;
    let under_point = point - normalv * EPSILON;
    let reflectv = reflect(&ray.direction, &normalv);

    let mut n1 = 1.;
    let mut n2 = 1.;
    let mut containers: Vec<Shape> = Vec::new();
    for i in &intersections.data {
        // Optimization: First check t as a quick test.
        let matching = i.t == intersection.t && i == intersection;
        if matching {
            if let Some(last) = containers.last() {
                n1 = last.material.refractive_index;
            }
        }
        let o = i.object.clone();
        if let Some(index) = containers.iter().position(|value| *value == o) {
            containers.swap_remove(index);
        } else {
            containers.push(o);
        }
        if matching {
            if let Some(last) = containers.last() {
                n2 = last.material.refractive_index;
            }
            break;
        }
    }

    C {
        t: intersection.t,
        object: &intersection.object,
        point,
        eyev,
        normalv,
        inside,
        reflectv,
        over_point,
        under_point,
        n1,
        n2,
    }
}

pub fn schlick(comps: &C) -> f64 {
    let mut cos = dot(&comps.eyev, &comps.normalv);
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n * n * (1. - cos * cos);
        if sin2_t > 1. {
            return 1.;
        }
        let cos_t = (1. - sin2_t).sqrt();
        cos = cos_t;
    }
    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    return r0 + (1. - r0) * (1. - cos).powi(5);
}

