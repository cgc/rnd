
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_smooth_triangles_00_constructing_a_smooth_triangle() {
    // Constructing a smooth triangle
	let p1 = point(0_f64, 1_f64, 0_f64);
	let p2 = point(-1_f64, 0_f64, 0_f64);
	let p3 = point(1_f64, 0_f64, 0_f64);
	let n1 = vector(0_f64, 1_f64, 0_f64);
	let n2 = vector(-1_f64, 0_f64, 0_f64);
	let n3 = vector(1_f64, 0_f64, 0_f64);
	let tri = smooth_triangle(&p1, &p2, &p3, &n1, &n2, &n3);
	assert_eq!(tri.p1(), p1);
	assert_eq!(tri.p2(), p2);
	assert_eq!(tri.p3(), p3);
	assert_eq!(tri.n1(), n1);
	assert_eq!(tri.n2(), n2);
	assert_eq!(tri.n3(), n3);
}

#[test]
fn test_smooth_triangles_01_an_intersection_with_a_smooth_triangle_stores_u_v() {
    // An intersection with a smooth triangle stores u/v
	let p1 = point(0_f64, 1_f64, 0_f64);
	let p2 = point(-1_f64, 0_f64, 0_f64);
	let p3 = point(1_f64, 0_f64, 0_f64);
	let n1 = vector(0_f64, 1_f64, 0_f64);
	let n2 = vector(-1_f64, 0_f64, 0_f64);
	let n3 = vector(1_f64, 0_f64, 0_f64);
	let tri = smooth_triangle(&p1, &p2, &p3, &n1, &n2, &n3);
	let r = ray(&point(-0.2_f64, 0.3_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&tri, &r);
	assert!(equal(xs[0].u, 0.45_f64));
	assert!(equal(xs[0].v, 0.25_f64));
}

#[test]
fn test_smooth_triangles_02_a_smooth_triangle_uses_u_v_to_interpolate_the_normal() {
    // A smooth triangle uses u/v to interpolate the normal
	let p1 = point(0_f64, 1_f64, 0_f64);
	let p2 = point(-1_f64, 0_f64, 0_f64);
	let p3 = point(1_f64, 0_f64, 0_f64);
	let n1 = vector(0_f64, 1_f64, 0_f64);
	let n2 = vector(-1_f64, 0_f64, 0_f64);
	let n3 = vector(1_f64, 0_f64, 0_f64);
	let tri = smooth_triangle(&p1, &p2, &p3, &n1, &n2, &n3);
	let i = intersection_with_uv(1_f64, &tri, 0.45_f64, 0.25_f64);
	let n = normal_at3(&tri, &point(0_f64, 0_f64, 0_f64), &i);
	assert_eq!(n, vector(-0.5547_f64, 0.83205_f64, 0_f64));
}

#[test]
fn test_smooth_triangles_03_preparing_the_normal_on_a_smooth_triangle() {
    // Preparing the normal on a smooth triangle
	let p1 = point(0_f64, 1_f64, 0_f64);
	let p2 = point(-1_f64, 0_f64, 0_f64);
	let p3 = point(1_f64, 0_f64, 0_f64);
	let n1 = vector(0_f64, 1_f64, 0_f64);
	let n2 = vector(-1_f64, 0_f64, 0_f64);
	let n3 = vector(1_f64, 0_f64, 0_f64);
	let tri = smooth_triangle(&p1, &p2, &p3, &n1, &n2, &n3);
	let i = intersection_with_uv(1_f64, &tri, 0.45_f64, 0.25_f64);
	let r = ray(&point(-0.2_f64, 0.3_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections1(i);
	let comps = prepare_computations3(&i, &r, &xs);
	assert_eq!(comps.normalv, vector(-0.5547_f64, 0.83205_f64, 0_f64));
}
