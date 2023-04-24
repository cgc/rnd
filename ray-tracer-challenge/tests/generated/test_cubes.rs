
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex00() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(5_f64, 0.5_f64, 0_f64), &vector(-1_f64, 0_f64, 0_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex01() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(-5_f64, 0.5_f64, 0_f64), &vector(1_f64, 0_f64, 0_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex02() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(0.5_f64, 5_f64, 0_f64), &vector(0_f64, -1_f64, 0_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex03() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(0.5_f64, -5_f64, 0_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex04() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(0.5_f64, 0_f64, 5_f64), &vector(0_f64, 0_f64, -1_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex05() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(0.5_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cubes_00_a_ray_intersects_a_cube_ex06() {
    // A ray intersects a cube
	let mut c = cube();
	let r = ray(&point(0_f64, 0.5_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert_eq!(xs[0].t, -1_f64);
	assert!(equal(xs[1].t, 1_f64));
}

#[test]
fn test_cubes_01_a_ray_misses_a_cube_ex00() {
    // A ray misses a cube
	let mut c = cube();
	let r = ray(&point(-2_f64, 0_f64, 0_f64), &vector(0.2673_f64, 0.5345_f64, 0.8018_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cubes_01_a_ray_misses_a_cube_ex01() {
    // A ray misses a cube
	let mut c = cube();
	let r = ray(&point(0_f64, -2_f64, 0_f64), &vector(0.8018_f64, 0.2673_f64, 0.5345_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cubes_01_a_ray_misses_a_cube_ex02() {
    // A ray misses a cube
	let mut c = cube();
	let r = ray(&point(0_f64, 0_f64, -2_f64), &vector(0.5345_f64, 0.8018_f64, 0.2673_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cubes_01_a_ray_misses_a_cube_ex03() {
    // A ray misses a cube
	let mut c = cube();
	let r = ray(&point(2_f64, 0_f64, 2_f64), &vector(0_f64, 0_f64, -1_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cubes_01_a_ray_misses_a_cube_ex04() {
    // A ray misses a cube
	let mut c = cube();
	let r = ray(&point(0_f64, 2_f64, 2_f64), &vector(0_f64, -1_f64, 0_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cubes_01_a_ray_misses_a_cube_ex05() {
    // A ray misses a cube
	let mut c = cube();
	let r = ray(&point(2_f64, 2_f64, 0_f64), &vector(-1_f64, 0_f64, 0_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex00() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(1_f64, 0.5_f64, -0.8_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(1_f64, 0_f64, 0_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex01() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(-1_f64, -0.2_f64, 0.9_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(-1_f64, 0_f64, 0_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex02() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(-0.4_f64, 1_f64, -0.1_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(0_f64, 1_f64, 0_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex03() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(0.3_f64, -1_f64, -0.7_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(0_f64, -1_f64, 0_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex04() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(-0.6_f64, 0.3_f64, 1_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(0_f64, 0_f64, 1_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex05() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(0.4_f64, 0.4_f64, -1_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex06() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(1_f64, 1_f64, 1_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(1_f64, 0_f64, 0_f64));
}

#[test]
fn test_cubes_02_the_normal_on_the_surface_of_a_cube_ex07() {
    // The normal on the surface of a cube
	let mut c = cube();
	let p = point(-1_f64, -1_f64, -1_f64);
	let normal = local_normal_at(&c, &p);
	assert_eq!(normal, vector(-1_f64, 0_f64, 0_f64));
}
