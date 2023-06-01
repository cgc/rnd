
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_cones_00_intersecting_a_cone_with_a_ray_ex00() {
    // Intersecting a cone with a ray
	let mut shape = cone();
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 5_f64));
	assert!(equal(xs[1].t, 5_f64));
}

#[test]
fn test_cones_00_intersecting_a_cone_with_a_ray_ex01() {
    // Intersecting a cone with a ray
	let mut shape = cone();
	let direction = normalize(&vector(1_f64, 1_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 8.66025_f64));
	assert!(equal(xs[1].t, 8.66025_f64));
}

#[test]
fn test_cones_00_intersecting_a_cone_with_a_ray_ex02() {
    // Intersecting a cone with a ray
	let mut shape = cone();
	let direction = normalize(&vector(-0.5_f64, -1_f64, 1_f64));
	let r = ray(&point(1_f64, 1_f64, -5_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4.55006_f64));
	assert!(equal(xs[1].t, 49.44994_f64));
}

#[test]
fn test_cones_01_intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
    // Intersecting a cone with a ray parallel to one of its halves
	let mut shape = cone();
	let direction = normalize(&vector(0_f64, 1_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -1_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 1);
	assert!(equal(xs[0].t, 0.35355_f64));
}

#[test]
fn test_cones_02_intersecting_a_cone_s_end_caps_ex00() {
    // Intersecting a cone's end caps
	let mut shape = cone();
	shape.set_minimum(&(-0.5_f64));
	shape.set_maximum(&(0.5_f64));
	shape.set_closed(&(true));
	let direction = normalize(&vector(0_f64, 1_f64, 0_f64));
	let r = ray(&point(0_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cones_02_intersecting_a_cone_s_end_caps_ex01() {
    // Intersecting a cone's end caps
	let mut shape = cone();
	shape.set_minimum(&(-0.5_f64));
	shape.set_maximum(&(0.5_f64));
	shape.set_closed(&(true));
	let direction = normalize(&vector(0_f64, 1_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -0.25_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cones_02_intersecting_a_cone_s_end_caps_ex02() {
    // Intersecting a cone's end caps
	let mut shape = cone();
	shape.set_minimum(&(-0.5_f64));
	shape.set_maximum(&(0.5_f64));
	shape.set_closed(&(true));
	let direction = normalize(&vector(0_f64, 1_f64, 0_f64));
	let r = ray(&point(0_f64, 0_f64, -0.25_f64), &direction);
	let xs = local_intersect(&shape, &r);
	assert_eq!(xs.count, 4);
}

#[test]
fn test_cones_03_computing_the_normal_vector_on_a_cone_ex00() {
    // Computing the normal vector on a cone
	let mut shape = cone();
	let n = local_normal_at(&shape, &point(0_f64, 0_f64, 0_f64));
	assert_eq!(n, vector(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_cones_03_computing_the_normal_vector_on_a_cone_ex01() {
    // Computing the normal vector on a cone
	let mut shape = cone();
	let n = local_normal_at(&shape, &point(1_f64, 1_f64, 1_f64));
	assert_eq!(n, vector(1_f64, -2_f64.sqrt(), 1_f64));
}

#[test]
fn test_cones_03_computing_the_normal_vector_on_a_cone_ex02() {
    // Computing the normal vector on a cone
	let mut shape = cone();
	let n = local_normal_at(&shape, &point(-1_f64, -1_f64, 0_f64));
	assert_eq!(n, vector(-1_f64, 1_f64, 0_f64));
}
