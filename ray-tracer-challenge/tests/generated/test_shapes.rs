
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_shapes_00_the_default_transformation() {
    // The default transformation
	let mut s = test_shape();
	assert_eq!(s.transform(), identity_matrix);
}

#[test]
fn test_shapes_01_assigning_a_transformation() {
    // Assigning a transformation
	let mut s = test_shape();
	set_transform(&mut s, &translation(2_f64, 3_f64, 4_f64));
	assert_eq!(s.transform(), translation(2_f64, 3_f64, 4_f64));
}

#[test]
fn test_shapes_02_the_default_material() {
    // The default material
	let mut s = test_shape();
	let mut m = s.material;
	assert_eq!(m, material());
}

#[test]
fn test_shapes_03_assigning_a_material() {
    // Assigning a material
	let mut s = test_shape();
	let mut m = material();
	m.ambient = 1_f64;
	s.material = m;
	assert_eq!(s.material, m);
}

#[test]
fn test_shapes_04_intersecting_a_scaled_shape_with_a_ray() {
    // Intersecting a scaled shape with a ray
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = test_shape();
	set_transform(&mut s, &scaling(2_f64, 2_f64, 2_f64));
	let xs = intersect(&s, &r);
	let saved_ray = unpack_saved_ray(&xs);
	assert_eq!(saved_ray.origin, point(0_f64, 0_f64, -2.5_f64));
	assert_eq!(saved_ray.direction, vector(0_f64, 0_f64, 0.5_f64));
}

#[test]
fn test_shapes_05_intersecting_a_translated_shape_with_a_ray() {
    // Intersecting a translated shape with a ray
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = test_shape();
	set_transform(&mut s, &translation(5_f64, 0_f64, 0_f64));
	let xs = intersect(&s, &r);
	let saved_ray = unpack_saved_ray(&xs);
	assert_eq!(saved_ray.origin, point(-5_f64, 0_f64, -5_f64));
	assert_eq!(saved_ray.direction, vector(0_f64, 0_f64, 1_f64));
}

#[test]
fn test_shapes_06_computing_the_normal_on_a_translated_shape() {
    // Computing the normal on a translated shape
	let mut s = test_shape();
	set_transform(&mut s, &translation(0_f64, 1_f64, 0_f64));
	let n = normal_at2(&s, &point(0_f64, 1.70711_f64, -0.70711_f64));
	assert_eq!(n, vector(0_f64, 0.70711_f64, -0.70711_f64));
}

#[test]
fn test_shapes_07_computing_the_normal_on_a_transformed_shape() {
    // Computing the normal on a transformed shape
	let mut s = test_shape();
	let mut m = scaling(1_f64, 0.5_f64, 1_f64) * rotation_z(PI / 5_f64);
	set_transform(&mut s, &m);
	let n = normal_at2(&s, &point(0_f64, 2_f64.sqrt() / 2_f64, -2_f64.sqrt() / 2_f64));
	assert_eq!(n, vector(0_f64, 0.97014_f64, -0.24254_f64));
}

#[test]
fn test_shapes_08_a_shape_has_a_parent_attribute() {
    // A shape has a parent attribute
	let mut s = test_shape();
}

#[test]
fn test_shapes_09_converting_a_point_from_world_to_object_space() {
    // Converting a point from world to object space
	let mut g1 = group();
	set_transform(&mut g1, &rotation_y(PI / 2_f64));
	let mut g2 = group();
	set_transform(&mut g2, &scaling(2_f64, 2_f64, 2_f64));
	let mut s = sphere();
	set_transform(&mut s, &translation(5_f64, 0_f64, 0_f64));
	add_child(&mut g2, &s);
	add_child(&mut g1, &g2);
	let mut s = &g1.children()[0].children()[0];
	let p = world_to_object(&s, &point(-2_f64, 0_f64, -10_f64));
	assert_eq!(p, point(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_shapes_10_converting_a_normal_from_object_to_world_space() {
    // Converting a normal from object to world space
	let mut g1 = group();
	set_transform(&mut g1, &rotation_y(PI / 2_f64));
	let mut g2 = group();
	set_transform(&mut g2, &scaling(1_f64, 2_f64, 3_f64));
	let mut s = sphere();
	set_transform(&mut s, &translation(5_f64, 0_f64, 0_f64));
	add_child(&mut g2, &s);
	add_child(&mut g1, &g2);
	let mut s = &g1.children()[0].children()[0];
	let n = normal_to_world(&s, &vector(3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64));
	assert_eq!(n, vector(0.2857_f64, 0.4286_f64, -0.8571_f64));
}

#[test]
fn test_shapes_11_finding_the_normal_on_a_child_object() {
    // Finding the normal on a child object
	let mut g1 = group();
	set_transform(&mut g1, &rotation_y(PI / 2_f64));
	let mut g2 = group();
	set_transform(&mut g2, &scaling(1_f64, 2_f64, 3_f64));
	let mut s = sphere();
	set_transform(&mut s, &translation(5_f64, 0_f64, 0_f64));
	add_child(&mut g2, &s);
	add_child(&mut g1, &g2);
	let mut s = &g1.children()[0].children()[0];
	let n = normal_at2(&s, &point(1.7321_f64, 1.1547_f64, -5.5774_f64));
	assert_eq!(n, vector(0.2857_f64, 0.4286_f64, -0.8571_f64));
}
