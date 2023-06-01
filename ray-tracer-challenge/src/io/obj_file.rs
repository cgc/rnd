use std::{f64::NAN, collections::HashMap};

use crate::{Tuple, point, group, Shape, triangle, add_child, vector, smooth_triangle};

pub struct Parser<'a> {
    pub ignored: usize,
    pub vertices: Vec<Tuple>,
    pub normals: Vec<Tuple>,
    pub default_group: Shape,
    pub current_group: Option<&'a str>,
    pub groups: HashMap<&'a str, Shape>,
}

impl Parser<'_> {
    fn new() -> Parser<'static> {
        // Dummy point to permit 1-indexing
        let dummy = point(NAN, NAN, NAN);
        Parser {
            ignored: 0,
            vertices: vec![dummy],
            normals: vec![dummy],
            default_group: group(),
            groups: HashMap::new(),
            current_group: None,
        }
    }

    pub fn default_group(&self) -> Shape {
        self.default_group.clone()
    }

    pub fn named_group(&self, name: &str) -> Option<&Shape> {
        self.groups.get(name)
    }
}

pub fn parse_obj_file(bytes: &[u8]) -> Parser {
    let s = std::str::from_utf8(bytes).unwrap();
    let mut p = Parser::new();
    for line in s.lines() {
        let bits: Vec<&str> = line.split_whitespace().collect();
        if bits.len() == 0 {
            continue
        }
        let cmd = bits[0];
        let args = &bits[1..];
        match cmd {
            "v" => {
                assert!(args.len() == 3);
                p.vertices.push(point(
                    args[0].parse().unwrap(),
                    args[1].parse().unwrap(),
                    args[2].parse().unwrap(),
                ));
            }
            "vn" => {
                assert!(args.len() == 3);
                p.normals.push(vector(
                    args[0].parse().unwrap(),
                    args[1].parse().unwrap(),
                    args[2].parse().unwrap(),
                ));
            }
            "f" => {
                let has_normals = args[0].contains("/");
                let parsed: Vec<(usize, Option<usize>, Option<usize>)> = args.into_iter().map(
                    |x| {
                        let els: Vec<&str> = x.split("/").collect();
                        if has_normals {
                            assert!(els.len() == 3);
                            (els[0].parse().unwrap(), els[1].parse().ok(), els[2].parse().ok())
                        } else {
                            assert!(els.len() == 1);
                            (els[0].parse().unwrap(), None, None)
                        }
                    }
                ).collect();
                let g = if let Some(name) = p.current_group {
                    p.groups.get_mut(name).unwrap()
                } else {
                    &mut p.default_group
                };
                for i in 2..args.len() {
                    let idx = (0, i-1, i);
                    let mut t = if has_normals {
                        smooth_triangle(
                            &p.vertices[parsed[idx.0].0],
                            &p.vertices[parsed[idx.1].0],
                            &p.vertices[parsed[idx.2].0],
                            &p.normals[parsed[idx.0].2.unwrap()],
                            &p.normals[parsed[idx.1].2.unwrap()],
                            &p.normals[parsed[idx.2].2.unwrap()],
                        )
                    } else {
                        triangle(
                            &p.vertices[parsed[idx.0].0],
                            &p.vertices[parsed[idx.1].0],
                            &p.vertices[parsed[idx.2].0],
                        )
                    };
                    add_child(g, &mut t);
                }
            }
            "g" => {
                let mut namesplit = line.splitn(2, char::is_whitespace);
                assert!(namesplit.next().unwrap() == cmd);
                let name = namesplit.next().unwrap();
                p.groups.insert(name, group());
                p.current_group = Some(name);
            }
            _ => {
                p.ignored += 1;
            }
        }
    }
    p
}

pub fn obj_to_group(parser: &Parser) -> Shape {
    let mut g = group();
    add_child(&mut g, & parser.default_group);
    for child in parser.groups.values() {
        add_child(&mut g, child);
    }
    g
}
