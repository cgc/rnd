
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_rays_00_creating_and_querying_a_ray() {
    // Creating and querying a ray
	let origin = point(1_f64, 2_f64, 3_f64);
	let direction = vector(4_f64, 5_f64, 6_f64);
	let r = ray(&origin, &direction);
	assert_eq!(r.origin, origin);
	assert_eq!(r.direction, direction);
}

#[test]
fn test_rays_01_computing_a_point_from_a_distance() {
    // Computing a point from a distance
	let r = ray(&point(2_f64, 3_f64, 4_f64), &vector(1_f64, 0_f64, 0_f64));
	assert_eq!(position(&r, 0_f64), point(2_f64, 3_f64, 4_f64));
	assert_eq!(position(&r, 1_f64), point(3_f64, 3_f64, 4_f64));
	assert_eq!(position(&r, -1_f64), point(1_f64, 3_f64, 4_f64));
	assert_eq!(position(&r, 2.5_f64), point(4.5_f64, 3_f64, 4_f64));
}

#[test]
fn test_rays_02_translating_a_ray() {
    // Translating a ray
	let r = ray(&point(1_f64, 2_f64, 3_f64), &vector(0_f64, 1_f64, 0_f64));
	let mut m = translation(3_f64, 4_f64, 5_f64);
	let r2 = transform(&r, &m);
	assert_eq!(r2.origin, point(4_f64, 6_f64, 8_f64));
	assert_eq!(r2.direction, vector(0_f64, 1_f64, 0_f64));
}

#[test]
fn test_rays_03_scaling_a_ray() {
    // Scaling a ray
	let r = ray(&point(1_f64, 2_f64, 3_f64), &vector(0_f64, 1_f64, 0_f64));
	let mut m = scaling(2_f64, 3_f64, 4_f64);
	let r2 = transform(&r, &m);
	assert_eq!(r2.origin, point(2_f64, 6_f64, 12_f64));
	assert_eq!(r2.direction, vector(0_f64, 3_f64, 0_f64));
}
