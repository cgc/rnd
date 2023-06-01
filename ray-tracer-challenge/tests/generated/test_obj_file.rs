
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_obj_file_00_ignoring_unrecognized_lines() {
    // Ignoring unrecognized lines
	let gibberish = "There was a young lady named Bright\nwho traveled much faster than light.\nShe set out one day\nin a relative way,\nand came back the previous night.".as_bytes();
	let mut parser = parse_obj_file(&gibberish);
	assert_eq!(parser.ignored, 5);
}

#[test]
fn test_obj_file_01_vertex_records() {
    // Vertex records
	let file = "v -1 1 0\nv -1.0000 0.5000 0.0000\nv 1 0 0\nv 1 1 0".as_bytes();
	let mut parser = parse_obj_file(&file);
	assert_eq!(parser.vertices[1], point(-1_f64, 1_f64, 0_f64));
	assert_eq!(parser.vertices[2], point(-1_f64, 0.5_f64, 0_f64));
	assert_eq!(parser.vertices[3], point(1_f64, 0_f64, 0_f64));
	assert_eq!(parser.vertices[4], point(1_f64, 1_f64, 0_f64));
}

#[test]
fn test_obj_file_02_parsing_triangle_faces() {
    // Parsing triangle faces
	let file = "v -1 1 0\nv -1 0 0\nv 1 0 0\nv 1 1 0\n\nf 1 2 3\nf 1 3 4".as_bytes();
	let mut parser = parse_obj_file(&file);
	let mut g = parser.default_group();
	let t1 = &g.children()[0];
	let t2 = &g.children()[1];
	assert_eq!(t1.p1(), parser.vertices[1]);
	assert_eq!(t1.p2(), parser.vertices[2]);
	assert_eq!(t1.p3(), parser.vertices[3]);
	assert_eq!(t2.p1(), parser.vertices[1]);
	assert_eq!(t2.p2(), parser.vertices[3]);
	assert_eq!(t2.p3(), parser.vertices[4]);
}

#[test]
fn test_obj_file_03_triangulating_polygons() {
    // Triangulating polygons
	let file = "v -1 1 0\nv -1 0 0\nv 1 0 0\nv 1 1 0\nv 0 2 0\n\nf 1 2 3 4 5".as_bytes();
	let mut parser = parse_obj_file(&file);
	let mut g = parser.default_group();
	let t1 = &g.children()[0];
	let t2 = &g.children()[1];
	let t3 = &g.children()[2];
	assert_eq!(t1.p1(), parser.vertices[1]);
	assert_eq!(t1.p2(), parser.vertices[2]);
	assert_eq!(t1.p3(), parser.vertices[3]);
	assert_eq!(t2.p1(), parser.vertices[1]);
	assert_eq!(t2.p2(), parser.vertices[3]);
	assert_eq!(t2.p3(), parser.vertices[4]);
	assert_eq!(t3.p1(), parser.vertices[1]);
	assert_eq!(t3.p2(), parser.vertices[4]);
	assert_eq!(t3.p3(), parser.vertices[5]);
}

#[test]
fn test_obj_file_04_triangles_in_groups() {
    // Triangles in groups
	let file = read("book-code/files/triangles.obj").unwrap();
	let mut parser = parse_obj_file(&file);
	let mut g1 = &parser.named_group("FirstGroup").unwrap();
	let mut g2 = &parser.named_group("SecondGroup").unwrap();
	let t1 = &g1.children()[0];
	let t2 = &g2.children()[0];
	assert_eq!(t1.p1(), parser.vertices[1]);
	assert_eq!(t1.p2(), parser.vertices[2]);
	assert_eq!(t1.p3(), parser.vertices[3]);
	assert_eq!(t2.p1(), parser.vertices[1]);
	assert_eq!(t2.p2(), parser.vertices[3]);
	assert_eq!(t2.p3(), parser.vertices[4]);
}

#[test]
fn test_obj_file_05_converting_an_obj_file_to_a_group() {
    // Converting an OBJ file to a group
	let file = read("book-code/files/triangles.obj").unwrap();
	let mut parser = parse_obj_file(&file);
	let mut g = obj_to_group(&parser);
	assert!(g.includes(&parser.named_group("FirstGroup").unwrap()));
	assert!(g.includes(&parser.named_group("SecondGroup").unwrap()));
}

#[test]
fn test_obj_file_06_vertex_normal_records() {
    // Vertex normal records
	let file = "vn 0 0 1\nvn 0.707 0 -0.707\nvn 1 2 3".as_bytes();
	let mut parser = parse_obj_file(&file);
	assert_eq!(parser.normals[1], vector(0_f64, 0_f64, 1_f64));
	assert_eq!(parser.normals[2], vector(0.707_f64, 0_f64, -0.707_f64));
	assert_eq!(parser.normals[3], vector(1_f64, 2_f64, 3_f64));
}

#[test]
fn test_obj_file_07_faces_with_normals() {
    // Faces with normals
	let file = "v 0 1 0\nv -1 0 0\nv 1 0 0\n\nvn -1 0 0\nvn 1 0 0\nvn 0 1 0\n\nf 1//3 2//1 3//2\nf 1/0/3 2/102/1 3/14/2".as_bytes();
	let mut parser = parse_obj_file(&file);
	let mut g = parser.default_group();
	let t1 = &g.children()[0];
	let t2 = &g.children()[1];
	assert_eq!(t1.p1(), parser.vertices[1]);
	assert_eq!(t1.p2(), parser.vertices[2]);
	assert_eq!(t1.p3(), parser.vertices[3]);
	assert_eq!(t1.n1(), parser.normals[3]);
	assert_eq!(t1.n2(), parser.normals[1]);
	assert_eq!(t1.n3(), parser.normals[2]);
	assert_eq!(t2, t1);
}
