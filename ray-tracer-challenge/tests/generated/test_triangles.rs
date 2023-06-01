
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_triangles_00_constructing_a_triangle() {
    // Constructing a triangle
	let p1 = point(0_f64, 1_f64, 0_f64);
	let p2 = point(-1_f64, 0_f64, 0_f64);
	let p3 = point(1_f64, 0_f64, 0_f64);
	let t = triangle(&p1, &p2, &p3);
	assert_eq!(t.p1(), p1);
	assert_eq!(t.p2(), p2);
	assert_eq!(t.p3(), p3);
	assert_eq!(t.e1(), vector(-1_f64, -1_f64, 0_f64));
	assert_eq!(t.e2(), vector(1_f64, -1_f64, 0_f64));
	assert_eq!(t.normal(), vector(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_triangles_01_intersecting_a_ray_parallel_to_the_triangle() {
    // Intersecting a ray parallel to the triangle
	let t = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let r = ray(&point(0_f64, -1_f64, -2_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = local_intersect(&t, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_triangles_02_a_ray_misses_the_p1_p3_edge() {
    // A ray misses the p1-p3 edge
	let t = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let r = ray(&point(1_f64, 1_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&t, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_triangles_03_a_ray_misses_the_p1_p2_edge() {
    // A ray misses the p1-p2 edge
	let t = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let r = ray(&point(-1_f64, 1_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&t, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_triangles_04_a_ray_misses_the_p2_p3_edge() {
    // A ray misses the p2-p3 edge
	let t = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let r = ray(&point(0_f64, -1_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&t, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_triangles_05_a_ray_strikes_a_triangle() {
    // A ray strikes a triangle
	let t = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let r = ray(&point(0_f64, 0.5_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&t, &r);
	assert_eq!(xs.count, 1);
	assert!(equal(xs[0].t, 2_f64));
}

#[test]
fn test_triangles_06_finding_the_normal_on_a_triangle() {
    // Finding the normal on a triangle
	let t = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let n1 = local_normal_at(&t, &point(0_f64, 0.5_f64, 0_f64));
	let n2 = local_normal_at(&t, &point(-0.5_f64, 0.75_f64, 0_f64));
	let n3 = local_normal_at(&t, &point(0.5_f64, 0.25_f64, 0_f64));
	assert_eq!(n1, t.normal());
	assert_eq!(n2, t.normal());
	assert_eq!(n3, t.normal());
}
