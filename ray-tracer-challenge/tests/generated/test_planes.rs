
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_planes_00_the_normal_of_a_plane_is_constant_everywhere() {
    // The normal of a plane is constant everywhere
	let p = plane();
	let n1 = local_normal_at(&p, &point(0_f64, 0_f64, 0_f64));
	let n2 = local_normal_at(&p, &point(10_f64, 0_f64, -10_f64));
	let n3 = local_normal_at(&p, &point(-5_f64, 0_f64, 150_f64));
	assert_eq!(n1, vector(0_f64, 1_f64, 0_f64));
	assert_eq!(n2, vector(0_f64, 1_f64, 0_f64));
	assert_eq!(n3, vector(0_f64, 1_f64, 0_f64));
}

#[test]
fn test_planes_01_intersect_with_a_ray_parallel_to_the_plane() {
    // Intersect with a ray parallel to the plane
	let p = plane();
	let r = ray(&point(0_f64, 10_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&p, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_planes_02_intersect_with_a_coplanar_ray() {
    // Intersect with a coplanar ray
	let p = plane();
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&p, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_planes_03_a_ray_intersecting_a_plane_from_above() {
    // A ray intersecting a plane from above
	let p = plane();
	let r = ray(&point(0_f64, 1_f64, 0_f64), &vector(0_f64, -1_f64, 0_f64));
	let xs = local_intersect(&p, &r);
	assert_eq!(xs.count, 1);
	assert!(equal(xs[0].t, 1_f64));
	assert_eq!(xs[0].object, &p);
}

#[test]
fn test_planes_04_a_ray_intersecting_a_plane_from_below() {
    // A ray intersecting a plane from below
	let p = plane();
	let r = ray(&point(0_f64, -1_f64, 0_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = local_intersect(&p, &r);
	assert_eq!(xs.count, 1);
	assert!(equal(xs[0].t, 1_f64));
	assert_eq!(xs[0].object, &p);
}
