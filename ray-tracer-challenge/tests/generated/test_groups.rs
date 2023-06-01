
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_groups_00_creating_a_new_group() {
    // Creating a new group
	let mut g = group();
	assert_eq!(g.transform(), identity_matrix);
	assert!(g.is_empty());
}

#[test]
fn test_groups_01_adding_a_child_to_a_group() {
    // Adding a child to a group
	let mut g = group();
	let mut s = test_shape();
	add_child(&mut g, &s);
	assert!(!g.is_empty());
	assert!(g.includes(&s));
}

#[test]
fn test_groups_02_intersecting_a_ray_with_an_empty_group() {
    // Intersecting a ray with an empty group
	let mut g = group();
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&g, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_groups_03_intersecting_a_ray_with_a_nonempty_group() {
    // Intersecting a ray with a nonempty group
	let mut g = group();
	let mut s1 = sphere();
	let mut s2 = sphere();
	set_transform(&mut s2, &translation(0_f64, 0_f64, -3_f64));
	let mut s3 = sphere();
	set_transform(&mut s3, &translation(5_f64, 0_f64, 0_f64));
	add_child(&mut g, &s1);
	add_child(&mut g, &s2);
	add_child(&mut g, &s3);
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&g, &r);
	assert_eq!(xs.count, 4);
	assert_eq!(xs[0].object, &s2);
	assert_eq!(xs[1].object, &s2);
	assert_eq!(xs[2].object, &s1);
	assert_eq!(xs[3].object, &s1);
}

#[test]
fn test_groups_04_intersecting_a_transformed_group() {
    // Intersecting a transformed group
	let mut g = group();
	set_transform(&mut g, &scaling(2_f64, 2_f64, 2_f64));
	let mut s = sphere();
	set_transform(&mut s, &translation(5_f64, 0_f64, 0_f64));
	add_child(&mut g, &s);
	let r = ray(&point(10_f64, 0_f64, -10_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersect(&g, &r);
	assert_eq!(xs.count, 2);
}
