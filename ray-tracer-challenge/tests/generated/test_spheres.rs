
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_spheres_00_a_ray_intersects_a_sphere_at_two_points() {
    // A ray intersects a sphere at two points
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 2);
	assert_eq!(xs[0], 4.0_f64);
	assert_eq!(xs[1], 6.0_f64);
}

#[test]
fn test_spheres_01_a_ray_intersects_a_sphere_at_a_tangent() {
    // A ray intersects a sphere at a tangent
	let r = ray(&point(0_f64, 1_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 2);
	assert_eq!(xs[0], 5.0_f64);
	assert_eq!(xs[1], 5.0_f64);
}

#[test]
fn test_spheres_02_a_ray_misses_a_sphere() {
    // A ray misses a sphere
	let r = ray(&point(0_f64, 2_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_spheres_03_a_ray_originates_inside_a_sphere() {
    // A ray originates inside a sphere
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 2);
	assert_eq!(xs[0], -1.0_f64);
	assert_eq!(xs[1], 1.0_f64);
}

#[test]
fn test_spheres_04_a_sphere_is_behind_a_ray() {
    // A sphere is behind a ray
	let r = ray(&point(0_f64, 0_f64, 5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 2);
	assert_eq!(xs[0], -6.0_f64);
	assert_eq!(xs[1], -4.0_f64);
}

#[test]
fn test_spheres_05_intersect_sets_the_object_on_the_intersection() {
    // Intersect sets the object on the intersection
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 2);
	assert_eq!(xs[0].object, &s);
	assert_eq!(xs[1].object, &s);
}

#[test]
fn test_spheres_06_a_sphere_s_default_transformation() {
    // A sphere's default transformation
	let mut s = sphere();
	assert_eq!(s.transform(), identity_matrix);
}

#[test]
fn test_spheres_07_changing_a_sphere_s_transformation() {
    // Changing a sphere's transformation
	let mut s = sphere();
	let t = translation(2_f64, 3_f64, 4_f64);
	set_transform(&mut s, &t);
	assert_eq!(s.transform(), t);
}

#[test]
fn test_spheres_08_intersecting_a_scaled_sphere_with_a_ray() {
    // Intersecting a scaled sphere with a ray
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	set_transform(&mut s, &scaling(2_f64, 2_f64, 2_f64));
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 3_f64));
	assert!(equal(xs[1].t, 7_f64));
}

#[test]
fn test_spheres_09_intersecting_a_translated_sphere_with_a_ray() {
    // Intersecting a translated sphere with a ray
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut s = sphere();
	set_transform(&mut s, &translation(5_f64, 0_f64, 0_f64));
	let xs = intersect(&s, &r);
	assert_eq!(xs.count, 0);
}

#[test]
fn test_spheres_10_the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    // The normal on a sphere at a point on the x axis
	let mut s = sphere();
	let n = normal_at2(&s, &point(1_f64, 0_f64, 0_f64));
	assert_eq!(n, vector(1_f64, 0_f64, 0_f64));
}

#[test]
fn test_spheres_11_the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    // The normal on a sphere at a point on the y axis
	let mut s = sphere();
	let n = normal_at2(&s, &point(0_f64, 1_f64, 0_f64));
	assert_eq!(n, vector(0_f64, 1_f64, 0_f64));
}

#[test]
fn test_spheres_12_the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    // The normal on a sphere at a point on the z axis
	let mut s = sphere();
	let n = normal_at2(&s, &point(0_f64, 0_f64, 1_f64));
	assert_eq!(n, vector(0_f64, 0_f64, 1_f64));
}

#[test]
fn test_spheres_13_the_normal_on_a_sphere_at_a_nonaxial_point() {
    // The normal on a sphere at a nonaxial point
	let mut s = sphere();
	let n = normal_at2(&s, &point(3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64));
	assert_eq!(n, vector(3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64));
}

#[test]
fn test_spheres_14_the_normal_is_a_normalized_vector() {
    // The normal is a normalized vector
	let mut s = sphere();
	let n = normal_at2(&s, &point(3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64, 3_f64.sqrt() / 3_f64));
	assert_eq!(n, normalize(&n));
}

#[test]
fn test_spheres_15_computing_the_normal_on_a_translated_sphere() {
    // Computing the normal on a translated sphere
	let mut s = sphere();
	set_transform(&mut s, &translation(0_f64, 1_f64, 0_f64));
	let n = normal_at2(&s, &point(0_f64, 1.70711_f64, -0.70711_f64));
	assert_eq!(n, vector(0_f64, 0.70711_f64, -0.70711_f64));
}

#[test]
fn test_spheres_16_computing_the_normal_on_a_transformed_sphere() {
    // Computing the normal on a transformed sphere
	let mut s = sphere();
	let mut m = scaling(1_f64, 0.5_f64, 1_f64) * rotation_z(PI / 5_f64);
	set_transform(&mut s, &m);
	let n = normal_at2(&s, &point(0_f64, 2_f64.sqrt() / 2_f64, -2_f64.sqrt() / 2_f64));
	assert_eq!(n, vector(0_f64, 0.97014_f64, -0.24254_f64));
}

#[test]
fn test_spheres_17_a_sphere_has_a_default_material() {
    // A sphere has a default material
	let mut s = sphere();
	let mut m = s.material;
	assert_eq!(m, material());
}

#[test]
fn test_spheres_18_a_sphere_may_be_assigned_a_material() {
    // A sphere may be assigned a material
	let mut s = sphere();
	let mut m = material();
	m.ambient = 1_f64;
	s.material = m;
	assert_eq!(s.material, m);
}

#[test]
fn test_spheres_19_a_helper_for_producing_a_sphere_with_a_glassy_material() {
    // A helper for producing a sphere with a glassy material
	let mut s = glass_sphere();
	assert_eq!(s.transform(), identity_matrix);
	assert!(equal(s.material.transparency, 1.0_f64));
	assert!(equal(s.material.refractive_index, 1.5_f64));
}
