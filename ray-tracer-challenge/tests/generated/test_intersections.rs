
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_intersections_00_an_intersection_encapsulates_t_and_object() {
    // An intersection encapsulates t and object
	let mut s = sphere();
	let i = intersection(3.5_f64, &s);
	assert!(equal(i.t, 3.5_f64));
	assert_eq!(i.object, &s);
}

#[test]
fn test_intersections_01_precomputing_the_state_of_an_intersection() {
    // Precomputing the state of an intersection
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = sphere();
	let i = intersection(4_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	assert_eq!(comps.t, i.t);
	assert_eq!(comps.object, i.object);
	assert_eq!(comps.point, point(0_f64, 0_f64, -1_f64));
	assert_eq!(comps.eyev, vector(0_f64, 0_f64, -1_f64));
	assert_eq!(comps.normalv, vector(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_intersections_02_precomputing_the_reflection_vector() {
    // Precomputing the reflection vector
	let mut shape = plane();
	let r = ray(&point(0_f64, 1_f64, -1_f64), &vector(0_f64, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	let i = intersection(2_f64.sqrt(), &shape);
	let comps = prepare_computations2(&i, &r);
	assert_eq!(comps.reflectv, vector(0_f64, 2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
}

#[test]
fn test_intersections_03_the_hit_when_an_intersection_occurs_on_the_outside() {
    // The hit, when an intersection occurs on the outside
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = sphere();
	let i = intersection(4_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	assert_eq!(comps.inside, false);
}

#[test]
fn test_intersections_04_the_hit_when_an_intersection_occurs_on_the_inside() {
    // The hit, when an intersection occurs on the inside
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = sphere();
	let i = intersection(1_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	assert_eq!(comps.point, point(0_f64, 0_f64, 1_f64));
	assert_eq!(comps.eyev, vector(0_f64, 0_f64, -1_f64));
	assert_eq!(comps.inside, true);
	assert_eq!(comps.normalv, vector(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_intersections_05_the_hit_should_offset_the_point() {
    // The hit should offset the point
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = sphere();
	shape.set_transform(&(translation(0_f64, 0_f64, 1_f64)));
	let i = intersection(5_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	assert!(comps.over_point.z < -EPSILON / 2_f64);
	assert!(comps.point.z > comps.over_point.z);
}

#[test]
fn test_intersections_06_the_under_point_is_offset_below_the_surface() {
    // The under point is offset below the surface
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = glass_sphere();
	shape.set_transform(&(translation(0_f64, 0_f64, 1_f64)));
	let i = intersection(5_f64, &shape);
	let xs = intersections1(i);
	let comps = prepare_computations3(&i, &r, &xs);
	assert!(comps.under_point.z > EPSILON / 2_f64);
	assert!(comps.point.z < comps.under_point.z);
}

#[test]
fn test_intersections_07_aggregating_intersections() {
    // Aggregating intersections
	let mut s = sphere();
	let i1 = intersection(1_f64, &s);
	let i2 = intersection(2_f64, &s);
	let xs = intersections2(i1, i2);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 1_f64));
	assert!(equal(xs[1].t, 2_f64));
}

#[test]
fn test_intersections_08_the_hit_when_all_intersections_have_positive_t() {
    // The hit, when all intersections have positive t
	let mut s = sphere();
	let i1 = intersection(1_f64, &s);
	let i2 = intersection(2_f64, &s);
	let xs = intersections2(i2, i1);
	let i = hit(&xs);
	assert_eq!(i.unwrap(), i1);
}

#[test]
fn test_intersections_09_the_hit_when_some_intersections_have_negative_t() {
    // The hit, when some intersections have negative t
	let mut s = sphere();
	let i1 = intersection(-1_f64, &s);
	let i2 = intersection(1_f64, &s);
	let xs = intersections2(i2, i1);
	let i = hit(&xs);
	assert_eq!(i.unwrap(), i2);
}

#[test]
fn test_intersections_10_the_hit_when_all_intersections_have_negative_t() {
    // The hit, when all intersections have negative t
	let mut s = sphere();
	let i1 = intersection(-2_f64, &s);
	let i2 = intersection(-1_f64, &s);
	let xs = intersections2(i2, i1);
	let i = hit(&xs);
	assert!(i.is_none());
}

#[test]
fn test_intersections_11_the_hit_is_always_the_lowest_nonnegative_intersection() {
    // The hit is always the lowest nonnegative intersection
	let mut s = sphere();
	let i1 = intersection(5_f64, &s);
	let i2 = intersection(7_f64, &s);
	let i3 = intersection(-3_f64, &s);
	let i4 = intersection(2_f64, &s);
	let xs = intersections4(i1, i2, i3, i4);
	let i = hit(&xs);
	assert_eq!(i.unwrap(), i4);
}

#[test]
fn test_intersections_12_finding_n1_and_n2_at_various_intersections_ex00() {
    // Finding n1 and n2 at various intersections
	let mut A = glass_sphere();
	A.set_transform(&(scaling(2_f64, 2_f64, 2_f64)));
	A.material.refractive_index = 1.5_f64;
	let mut B = glass_sphere();
	B.set_transform(&(translation(0_f64, 0_f64, -0.25_f64)));
	B.material.refractive_index = 2.0_f64;
	let mut C = glass_sphere();
	C.set_transform(&(translation(0_f64, 0_f64, 0.25_f64)));
	C.material.refractive_index = 2.5_f64;
	let r = ray(&point(0_f64, 0_f64, -4_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections6(intersection(2_f64, &A), intersection(2.75_f64, &B), intersection(3.25_f64, &C), intersection(4.75_f64, &B), intersection(5.25_f64, &C), intersection(6_f64, &A));
	let comps = prepare_computations3(&xs[0], &r, &xs);
	assert!(equal(comps.n1(), 1.0_f64));
	assert!(equal(comps.n2(), 1.5_f64));
}

#[test]
fn test_intersections_12_finding_n1_and_n2_at_various_intersections_ex01() {
    // Finding n1 and n2 at various intersections
	let mut A = glass_sphere();
	A.set_transform(&(scaling(2_f64, 2_f64, 2_f64)));
	A.material.refractive_index = 1.5_f64;
	let mut B = glass_sphere();
	B.set_transform(&(translation(0_f64, 0_f64, -0.25_f64)));
	B.material.refractive_index = 2.0_f64;
	let mut C = glass_sphere();
	C.set_transform(&(translation(0_f64, 0_f64, 0.25_f64)));
	C.material.refractive_index = 2.5_f64;
	let r = ray(&point(0_f64, 0_f64, -4_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections6(intersection(2_f64, &A), intersection(2.75_f64, &B), intersection(3.25_f64, &C), intersection(4.75_f64, &B), intersection(5.25_f64, &C), intersection(6_f64, &A));
	let comps = prepare_computations3(&xs[1], &r, &xs);
	assert!(equal(comps.n1(), 1.5_f64));
	assert!(equal(comps.n2(), 2.0_f64));
}

#[test]
fn test_intersections_12_finding_n1_and_n2_at_various_intersections_ex02() {
    // Finding n1 and n2 at various intersections
	let mut A = glass_sphere();
	A.set_transform(&(scaling(2_f64, 2_f64, 2_f64)));
	A.material.refractive_index = 1.5_f64;
	let mut B = glass_sphere();
	B.set_transform(&(translation(0_f64, 0_f64, -0.25_f64)));
	B.material.refractive_index = 2.0_f64;
	let mut C = glass_sphere();
	C.set_transform(&(translation(0_f64, 0_f64, 0.25_f64)));
	C.material.refractive_index = 2.5_f64;
	let r = ray(&point(0_f64, 0_f64, -4_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections6(intersection(2_f64, &A), intersection(2.75_f64, &B), intersection(3.25_f64, &C), intersection(4.75_f64, &B), intersection(5.25_f64, &C), intersection(6_f64, &A));
	let comps = prepare_computations3(&xs[2], &r, &xs);
	assert!(equal(comps.n1(), 2.0_f64));
	assert!(equal(comps.n2(), 2.5_f64));
}

#[test]
fn test_intersections_12_finding_n1_and_n2_at_various_intersections_ex03() {
    // Finding n1 and n2 at various intersections
	let mut A = glass_sphere();
	A.set_transform(&(scaling(2_f64, 2_f64, 2_f64)));
	A.material.refractive_index = 1.5_f64;
	let mut B = glass_sphere();
	B.set_transform(&(translation(0_f64, 0_f64, -0.25_f64)));
	B.material.refractive_index = 2.0_f64;
	let mut C = glass_sphere();
	C.set_transform(&(translation(0_f64, 0_f64, 0.25_f64)));
	C.material.refractive_index = 2.5_f64;
	let r = ray(&point(0_f64, 0_f64, -4_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections6(intersection(2_f64, &A), intersection(2.75_f64, &B), intersection(3.25_f64, &C), intersection(4.75_f64, &B), intersection(5.25_f64, &C), intersection(6_f64, &A));
	let comps = prepare_computations3(&xs[3], &r, &xs);
	assert!(equal(comps.n1(), 2.5_f64));
	assert!(equal(comps.n2(), 2.5_f64));
}

#[test]
fn test_intersections_12_finding_n1_and_n2_at_various_intersections_ex04() {
    // Finding n1 and n2 at various intersections
	let mut A = glass_sphere();
	A.set_transform(&(scaling(2_f64, 2_f64, 2_f64)));
	A.material.refractive_index = 1.5_f64;
	let mut B = glass_sphere();
	B.set_transform(&(translation(0_f64, 0_f64, -0.25_f64)));
	B.material.refractive_index = 2.0_f64;
	let mut C = glass_sphere();
	C.set_transform(&(translation(0_f64, 0_f64, 0.25_f64)));
	C.material.refractive_index = 2.5_f64;
	let r = ray(&point(0_f64, 0_f64, -4_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections6(intersection(2_f64, &A), intersection(2.75_f64, &B), intersection(3.25_f64, &C), intersection(4.75_f64, &B), intersection(5.25_f64, &C), intersection(6_f64, &A));
	let comps = prepare_computations3(&xs[4], &r, &xs);
	assert!(equal(comps.n1(), 2.5_f64));
	assert!(equal(comps.n2(), 1.5_f64));
}

#[test]
fn test_intersections_12_finding_n1_and_n2_at_various_intersections_ex05() {
    // Finding n1 and n2 at various intersections
	let mut A = glass_sphere();
	A.set_transform(&(scaling(2_f64, 2_f64, 2_f64)));
	A.material.refractive_index = 1.5_f64;
	let mut B = glass_sphere();
	B.set_transform(&(translation(0_f64, 0_f64, -0.25_f64)));
	B.material.refractive_index = 2.0_f64;
	let mut C = glass_sphere();
	C.set_transform(&(translation(0_f64, 0_f64, 0.25_f64)));
	C.material.refractive_index = 2.5_f64;
	let r = ray(&point(0_f64, 0_f64, -4_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections6(intersection(2_f64, &A), intersection(2.75_f64, &B), intersection(3.25_f64, &C), intersection(4.75_f64, &B), intersection(5.25_f64, &C), intersection(6_f64, &A));
	let comps = prepare_computations3(&xs[5], &r, &xs);
	assert!(equal(comps.n1(), 1.5_f64));
	assert!(equal(comps.n2(), 1.0_f64));
}

#[test]
fn test_intersections_13_the_schlick_approximation_under_total_internal_reflection() {
    // The Schlick approximation under total internal reflection
	let mut shape = glass_sphere();
	let r = ray(&point(0_f64, 0_f64, 2_f64.sqrt() / 2_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = intersections2(intersection(-2_f64.sqrt() / 2_f64, &shape), intersection(2_f64.sqrt() / 2_f64, &shape));
	let comps = prepare_computations3(&xs[1], &r, &xs);
	let reflectance = schlick(&comps);
	assert!(equal(reflectance, 1.0_f64));
}

#[test]
fn test_intersections_14_the_schlick_approximation_with_a_perpendicular_viewing_angle() {
    // The Schlick approximation with a perpendicular viewing angle
	let mut shape = glass_sphere();
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = intersections2(intersection(-1_f64, &shape), intersection(1_f64, &shape));
	let comps = prepare_computations3(&xs[1], &r, &xs);
	let reflectance = schlick(&comps);
	assert!(equal(reflectance, 0.04_f64));
}

#[test]
fn test_intersections_15_the_schlick_approximation_with_small_angle_and_n2_n1() {
    // The Schlick approximation with small angle and n2 > n1
	let mut shape = glass_sphere();
	let r = ray(&point(0_f64, 0.99_f64, -2_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections1(intersection(1.8589_f64, &shape));
	let comps = prepare_computations3(&xs[0], &r, &xs);
	let reflectance = schlick(&comps);
	assert!(equal(reflectance, 0.48873_f64));
}

#[test]
fn test_intersections_16_an_intersection_can_encapsulate_u_and_v_() {
    // An intersection can encapsulate `u` and `v`
	let mut s = triangle(&point(0_f64, 1_f64, 0_f64), &point(-1_f64, 0_f64, 0_f64), &point(1_f64, 0_f64, 0_f64));
	let i = intersection_with_uv(3.5_f64, &s, 0.2_f64, 0.4_f64);
	assert!(equal(i.u, 0.2_f64));
	assert!(equal(i.v, 0.4_f64));
}
