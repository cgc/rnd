
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_cylinders_00_a_ray_misses_a_cylinder_ex00() {
    // A ray misses a cylinder
	let mut cyl = cylinder();
	let direction = normalize(&vector(0_f64, 1_f64, 0_f64));
	let r = ray(&point(1_f64, 0_f64, 0_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_00_a_ray_misses_a_cylinder_ex01() {
    // A ray misses a cylinder
	let mut cyl = cylinder();
	let direction = normalize(&vector(0_f64, 1_f64, 0_f64));
	let r = ray(&point(0_f64, 0_f64, 0_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_00_a_ray_misses_a_cylinder_ex02() {
    // A ray misses a cylinder
	let mut cyl = cylinder();
	let direction = normalize(&vector(1_f64, 1_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_01_a_ray_strikes_a_cylinder_ex00() {
    // A ray strikes a cylinder
	let mut cyl = cylinder();
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(1_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 5_f64));
	assert!(equal(xs[1].t, 5_f64));
}

#[test]
fn test_cylinders_01_a_ray_strikes_a_cylinder_ex01() {
    // A ray strikes a cylinder
	let mut cyl = cylinder();
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 6_f64));
}

#[test]
fn test_cylinders_01_a_ray_strikes_a_cylinder_ex02() {
    // A ray strikes a cylinder
	let mut cyl = cylinder();
	let direction = normalize(&vector(0.1_f64, 1_f64, 1_f64));
	let r = ray(&point(0.5_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 6.80798_f64));
	assert!(equal(xs[1].t, 7.08872_f64));
}

#[test]
fn test_cylinders_02_normal_vector_on_a_cylinder_ex00() {
    // Normal vector on a cylinder
	let mut cyl = cylinder();
	let n = local_normal_at(&cyl, &point(1_f64, 0_f64, 0_f64));
	assert_eq!(n, vector(1_f64, 0_f64, 0_f64));
}

#[test]
fn test_cylinders_02_normal_vector_on_a_cylinder_ex01() {
    // Normal vector on a cylinder
	let mut cyl = cylinder();
	let n = local_normal_at(&cyl, &point(0_f64, 5_f64, -1_f64));
	assert_eq!(n, vector(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_cylinders_02_normal_vector_on_a_cylinder_ex02() {
    // Normal vector on a cylinder
	let mut cyl = cylinder();
	let n = local_normal_at(&cyl, &point(0_f64, -2_f64, 1_f64));
	assert_eq!(n, vector(0_f64, 0_f64, 1_f64));
}

#[test]
fn test_cylinders_02_normal_vector_on_a_cylinder_ex03() {
    // Normal vector on a cylinder
	let mut cyl = cylinder();
	let n = local_normal_at(&cyl, &point(-1_f64, 1_f64, 0_f64));
	assert_eq!(n, vector(-1_f64, 0_f64, 0_f64));
}

#[test]
fn test_cylinders_03_the_default_minimum_and_maximum_for_a_cylinder() {
    // The default minimum and maximum for a cylinder
	let mut cyl = cylinder();
	assert_eq!(cyl.minimum(), -infinity);
	assert_eq!(cyl.maximum(), infinity);
}

#[test]
fn test_cylinders_04_intersecting_a_constrained_cylinder_ex00() {
    // Intersecting a constrained cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	let direction = normalize(&vector(0.1_f64, 1_f64, 0_f64));
	let r = ray(&point(0_f64, 1.5_f64, 0_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_04_intersecting_a_constrained_cylinder_ex01() {
    // Intersecting a constrained cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 3_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_04_intersecting_a_constrained_cylinder_ex02() {
    // Intersecting a constrained cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 0_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_04_intersecting_a_constrained_cylinder_ex03() {
    // Intersecting a constrained cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 2_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_04_intersecting_a_constrained_cylinder_ex04() {
    // Intersecting a constrained cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 1_f64, -5_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_cylinders_04_intersecting_a_constrained_cylinder_ex05() {
    // Intersecting a constrained cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	let direction = normalize(&vector(0_f64, 0_f64, 1_f64));
	let r = ray(&point(0_f64, 1.5_f64, -2_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cylinders_05_the_default_closed_value_for_a_cylinder() {
    // The default closed value for a cylinder
	let mut cyl = cylinder();
	assert_eq!(cyl.closed(), false);
}

#[test]
fn test_cylinders_06_intersecting_the_caps_of_a_closed_cylinder_ex00() {
    // Intersecting the caps of a closed cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let direction = normalize(&vector(0_f64, -1_f64, 0_f64));
	let r = ray(&point(0_f64, 3_f64, 0_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cylinders_06_intersecting_the_caps_of_a_closed_cylinder_ex01() {
    // Intersecting the caps of a closed cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let direction = normalize(&vector(0_f64, -1_f64, 2_f64));
	let r = ray(&point(0_f64, 3_f64, -2_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cylinders_06_intersecting_the_caps_of_a_closed_cylinder_ex02() {
    // Intersecting the caps of a closed cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let direction = normalize(&vector(0_f64, -1_f64, 1_f64));
	let r = ray(&point(0_f64, 4_f64, -2_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cylinders_06_intersecting_the_caps_of_a_closed_cylinder_ex03() {
    // Intersecting the caps of a closed cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let direction = normalize(&vector(0_f64, 1_f64, 2_f64));
	let r = ray(&point(0_f64, 0_f64, -2_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cylinders_06_intersecting_the_caps_of_a_closed_cylinder_ex04() {
    // Intersecting the caps of a closed cylinder
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let direction = normalize(&vector(0_f64, 1_f64, 1_f64));
	let r = ray(&point(0_f64, -1_f64, -2_f64), &direction);
	let xs = local_intersect(&cyl, &r);
	assert_eq!(xs.count, 2);
}

#[test]
fn test_cylinders_07_the_normal_vector_on_a_cylinder_s_end_caps_ex00() {
    // The normal vector on a cylinder's end caps
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let n = local_normal_at(&cyl, &point(0_f64, 1_f64, 0_f64));
	assert_eq!(n, vector(0_f64, -1_f64, 0_f64));
}

#[test]
fn test_cylinders_07_the_normal_vector_on_a_cylinder_s_end_caps_ex01() {
    // The normal vector on a cylinder's end caps
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let n = local_normal_at(&cyl, &point(0.5_f64, 1_f64, 0_f64));
	assert_eq!(n, vector(0_f64, -1_f64, 0_f64));
}

#[test]
fn test_cylinders_07_the_normal_vector_on_a_cylinder_s_end_caps_ex02() {
    // The normal vector on a cylinder's end caps
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let n = local_normal_at(&cyl, &point(0_f64, 1_f64, 0.5_f64));
	assert_eq!(n, vector(0_f64, -1_f64, 0_f64));
}

#[test]
fn test_cylinders_07_the_normal_vector_on_a_cylinder_s_end_caps_ex03() {
    // The normal vector on a cylinder's end caps
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let n = local_normal_at(&cyl, &point(0_f64, 2_f64, 0_f64));
	assert_eq!(n, vector(0_f64, 1_f64, 0_f64));
}

#[test]
fn test_cylinders_07_the_normal_vector_on_a_cylinder_s_end_caps_ex04() {
    // The normal vector on a cylinder's end caps
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let n = local_normal_at(&cyl, &point(0.5_f64, 2_f64, 0_f64));
	assert_eq!(n, vector(0_f64, 1_f64, 0_f64));
}

#[test]
fn test_cylinders_07_the_normal_vector_on_a_cylinder_s_end_caps_ex05() {
    // The normal vector on a cylinder's end caps
	let mut cyl = cylinder();
	cyl.set_minimum(&(1_f64));
	cyl.set_maximum(&(2_f64));
	cyl.set_closed(&(true));
	let n = local_normal_at(&cyl, &point(0_f64, 2_f64, 0.5_f64));
	assert_eq!(n, vector(0_f64, 1_f64, 0_f64));
}
