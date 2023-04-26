use std::{str::FromStr};

use crate::{Shape, LocalShape, test_shape, ShapeType, Intersections, intersections, BoundingBox, transform};

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Union,
    Intersection,
    Difference,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "union" { Ok(Operation::Union) }
        else if s == "intersection" { Ok(Operation::Intersection) }
        else if s == "difference" { Ok(Operation::Difference) }
        else { Err(()) }
    }
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Union => "union",
            Operation::Intersection => "intersection",
            Operation::Difference => "difference",
        }.to_owned()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CSG {
    op: Operation,
    pub children: Vec<Shape>,
}

impl LocalShape for CSG {
    fn local_normal_at(&self, _object_point: &crate::Tuple, _intersection: &crate::Intersection) -> crate::Tuple {
        panic!("Should not be called")
    }

    fn local_bounding_box(&self) -> crate::BoundingBox {
        match self.op {
            // Needs to be conservative!
            // In particular, we can't be sure that the entirety of the right side's bounding
            // box can be excluded, because the bounding box itself is conservative and we're
            // working with its negation here.
            Operation::Difference => self.children[0].as_local_shape().local_bounding_box(),
            _ => {
                // TODO: Could be more conservarive for intersection.
                let mut bb = BoundingBox::new_empty();
                for s in &self.children {
                    bb.include_bb(&s.as_local_shape().local_bounding_box());
                }
                bb
            },
        }
    }

    fn local_intersect<'a>(&'a self, shape: &'a Shape, object_ray: &crate::Ray) -> Vec<crate::Intersection> {
        assert!(self.children.len() == 2); // Just asserting this somewhere
        let mut xs = vec![];
        for child in &self.children {
            // HACK: this loop is basically copy/pasted from group's intersect.
            let t = child.local_inverse();
            let object_ray = transform(object_ray, &t);
            let mut ts = child.local_intersect(&object_ray);
            xs.append(&mut ts);
        }
        // Doing this to sort by time.
        let is = intersections(xs);
        filter_intersections(shape, &is).data
    }
}

impl Shape {
    pub fn as_csg(&self) -> Option<&CSG> {
        if let ShapeType::CSG(c) = &self.shape_type { Some(c) } else { None }
    }

    pub fn left(&self) -> Shape {
        self.as_csg().unwrap().children[0].clone()
    }

    pub fn right(&self) -> Shape {
        self.as_csg().unwrap().children[1].clone()
    }

    pub fn operation(&self) -> String {
        self.as_csg().unwrap().op.to_string()
    }
}

pub fn csg(op: &str, s1: &Shape, s2: &Shape) -> Shape {
    let mut s = test_shape();
    s.shape_type = ShapeType::CSG(CSG {
        op: op.parse().unwrap(),
        children: vec![s1.clone(), s2.clone()],
    });
    s
}

pub fn intersection_allowed(op: &str, lhit: &bool, inl: &bool, inr: &bool) -> bool {
    let lhit = *lhit;
    let inl = *inl;
    let inr = *inr;
    let op: Operation = op.parse().unwrap();
    match op {
        Operation::Union => {
            (lhit && !inr) || (!lhit && !inl)
        },
        Operation::Intersection => {
            (lhit && inr) || (!lhit && inl)
        }
        Operation::Difference => {
            (lhit && !inr) || (!lhit && inl)
        }
    }
}

pub fn filter_intersections<'a>(shape: &'a Shape, xs: &Intersections<'a>) -> Intersections<'a> {
    let csg = shape.as_csg().unwrap();
    let mut inl = false;
    let mut inr = false;
    let mut result = vec![];
    for i in &xs.data {
        let lhit = csg.children[0].includes(&i.object);
        if intersection_allowed(&csg.op.to_string(), &lhit, &inl, &inr) {
            result.push(i.clone());
        }
        if lhit {
            inl = !inl;
        } else {
            inr = !inr;
        }
    }
    intersections(result)
}
