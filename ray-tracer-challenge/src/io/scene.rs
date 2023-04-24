use core::panic;
use std::{fs, error::Error, collections::HashMap};

use serde::{Serialize, Deserialize};

use crate::{Tuple, Color, world, point_light, point, vector, color, camera, view_transform, render, Canvas, identity_matrix, Matrix, translation, rotation_x, plane, Shape, DEFAULT_MATERIAL, Material, scaling, cube, sphere, group, add_child, checkers_pattern, rotation_y, rotation_z, stripe_pattern, cylinder, cone};

type Tup = [f64; 3];

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Pat {
    #[serde(alias = "type")]
    type_: String,
    colors: Vec<Tup>,
    transform: Option<Vec<TransformEntry>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Mat {
    pattern: Option<Pat>,
    color: Option<Tup>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    reflective: Option<f64>,
    transparency: Option<f64>,
    #[serde(alias = "refractive-index")]
    refractive_index: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum MatEntry {
    Name(String),
    Mat(Mat),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum TransformEntryItem {
    Op(String),
    Arg(f64),
}

impl TransformEntryItem {
    fn as_op(&self) -> Option<&str> {
        if let TransformEntryItem::Op(s) = self { Some(s) } else { None }
    }
    fn as_arg(&self) -> Option<f64> {
        if let TransformEntryItem::Arg(s) = self { Some(*s) } else { None }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum TransformEntry {
    Name(String),
    List(Vec<TransformEntryItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ShapeEntry {
    add: String,
    material: Option<MatEntry>,
    transform: Option<Vec<TransformEntry>>,
    min: Option<f64>,
    max: Option<f64>,
    closed: Option<bool>,
    children: Option<Vec<ShapeEntry>>,
    shadow: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(tag = "add")]
pub enum AddEntry {
    #[serde(alias = "light")]
    Light {
        at: Tup,
        intensity: Tup,
    },
    #[serde(alias = "camera")]
    Camera {
        width: usize,
        height: usize,
        #[serde(alias = "field-of-view")]
        field_of_view: f64,
        from: Tup,
        to: Tup,
        up: Tup,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum DefineEntryValue {
    Mat(Mat),
    Transform(Vec<TransformEntry>),
    Shape(ShapeEntry)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DefineEntry {
    define: String,
    extend: Option<String>,
    value: DefineEntryValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Entry {
    AddEntry(AddEntry),
    DefineEntry(DefineEntry),
    ShapeEntry(ShapeEntry),
}

fn to_point(p: &Tup) -> Tuple { point(p[0], p[1], p[2]) }
fn to_vector(p: &Tup) -> Tuple { vector(p[0], p[1], p[2]) }
fn to_color(p: &Tup) -> Color { color(p[0], p[1], p[2]) }
fn to_material(m: &MatEntry, base: &Material, mdef: &HashMap<&str, Material>, tdef: &HashMap<&str, Matrix>) -> Material {
    match m {
        MatEntry::Name(name) => *mdef.get(name.as_str()).unwrap(),
        MatEntry::Mat(mat) => {
            let mut m = *base;
            if let Some(x) = mat.color { m.color = to_color(&x); };
            if let Some(x) = mat.ambient { m.ambient = x; };
            if let Some(x) = mat.diffuse { m.diffuse = x; };
            if let Some(x) = mat.specular { m.specular = x; };
            if let Some(x) = mat.shininess { m.shininess = x; };
            if let Some(x) = mat.reflective { m.reflective = x; };
            if let Some(x) = mat.transparency { m.transparency = x; };
            if let Some(x) = mat.refractive_index { m.refractive_index = x; };
            if let Some(p) = &mat.pattern {
                let func = match p.type_.as_str() {
                    "checkers" => checkers_pattern,
                    "stripes" => stripe_pattern,
                    _ => panic!(),
                };
                let mut pattern = func(&to_color(&p.colors[0]), &to_color(&p.colors[1]));
                if let Some(ts) = &p.transform {
                    pattern.set_transform(&to_transform(&ts, tdef));
                }
                m.pattern = Some(pattern);
            }
            m
        }
    }
}

fn to_transform(ts: &Vec<TransformEntry>, tdef: &HashMap<&str, Matrix>) -> Matrix {
    let mut m = identity_matrix;
    for t in ts {
        let tm = match t {
            TransformEntry::Name(name) => *tdef.get(name.as_str()).unwrap(),
            TransformEntry::List(te) => {
                let args: Vec<f64> = te[1..].into_iter().map(|a| a.as_arg().unwrap()).collect();
                match te[0].as_op().unwrap() {
                    "translate" => translation(args[0], args[1], args[2]),
                    "scale" => scaling(args[0], args[1], args[2]),
                    "rotate-x" => rotation_x(args[0]),
                    "rotate-y" => rotation_y(args[0]),
                    "rotate-z" => rotation_z(args[0]),
                    _ => panic!(),
                }
            },
        };
        m = tm * m;
    }
    m
}

fn to_shape(se: &ShapeEntry, tdef: &HashMap<&str, Matrix>, mdef: &HashMap<&str, Material>, sdef: &HashMap<&str, Shape>) -> Shape {
    let mut p = match se.add.as_str() {
        "plane" => plane(),
        "cube" => cube(),
        "sphere" => sphere(),
        "cylinder" => {
            let mut p = cylinder();
            p.set_closed(&se.closed.unwrap());
            p.set_minimum(&se.min.unwrap());
            p.set_maximum(&se.max.unwrap());
            p
        }
        "cone" => {
            let mut p = cone();
            p.set_closed(&se.closed.unwrap());
            p.set_minimum(&se.min.unwrap());
            p.set_maximum(&se.max.unwrap());
            p
        }
        "group" => {
            let mut g = group();
            if let Some(children) = &se.children {
                for child_se in children {
                    let mut s = to_shape(&child_se, tdef, mdef, sdef);
                    add_child(&mut g, &mut s)
                }
            }
            g
        }
        _ => sdef.get(se.add.as_str()).unwrap().clone(),
    };
    fill_shape(&mut p, se, &tdef, &mdef);
    p
}

fn fill_shape(shape: &mut Shape, se: &ShapeEntry, tdef: &HashMap<&str, Matrix>, mdef: &HashMap<&str, Material>) {
    if let Some(t) = &se.transform {
        shape.set_transform(&to_transform(t, tdef));
    }
    if let Some(m) = &se.material {
        shape.material = to_material(m, &DEFAULT_MATERIAL, mdef, tdef);
    }
    if let Some(s) = se.shadow {
        shape.shadow = s;
    }
}

pub fn load(path: &str) -> Result<Canvas, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let yaml = std::str::from_utf8(&bytes)?;
    let res: Vec<Entry> = serde_yaml::from_str(&yaml)?;

    let mut w = world();
    let mut c = None;
    let mut g = group();

    let mut tdef = HashMap::new();
    let mut mdef: HashMap<&str, Material> = HashMap::new();
    let mut sdef: HashMap<&str, Shape> = HashMap::new();

    for e in &res {
        match &e {
            Entry::AddEntry(ae) => {
                match ae {
                    AddEntry::Light { at, intensity } => {
                        w.add_light(&point_light(&to_point(&at), &to_color(&intensity)))
                    }
                    AddEntry::Camera { width, height, field_of_view, from, to, up } => {
                        let mut camera = camera(*width as f64, *height as f64, *field_of_view);
                        let t = view_transform(
                            &to_point(from),
                            &to_point(to),
                            &to_vector(up),
                        );
                        camera.set_transform(&t);
                        c = Some(camera);
                    }
                }
            }
            Entry::ShapeEntry(se) => {
                let mut p = to_shape(se, &tdef, &mdef, &sdef);
                add_child(&mut g, &mut p);
            }
            Entry::DefineEntry(de) => {
                match &de.value {
                    DefineEntryValue::Mat(mat) => {
                        let base = if let Some(e) = &de.extend { mdef.get(e.as_str()).unwrap() } else { &DEFAULT_MATERIAL };
                        let m = to_material(&MatEntry::Mat(mat.clone()), base, &mdef, &tdef);
                        mdef.insert(de.define.as_str(), m);
                    },
                    DefineEntryValue::Transform(ts) => {
                        assert!(de.extend.is_none());
                        tdef.insert(de.define.as_str(), to_transform(ts, &tdef));
                    },
                    DefineEntryValue::Shape(se) => {
                        assert!(de.extend.is_none());
                        sdef.insert(de.define.as_str(), to_shape(se, &tdef, &mdef, &sdef));
                    },
                }
            }
        }
    }

    // Doesn't help too much here.
    g.freeze_and_optimize();
    w.add(&g);

    Ok(render(&c.unwrap(), &w))
}
