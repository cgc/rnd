
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_tuples_00_a_tuple_with_w_1_0_is_a_point() {
    // A tuple with w=1.0 is a point
	let a = tuple(4.3_f64, -4.2_f64, 3.1_f64, 1.0_f64);
	assert!(equal(a.x, 4.3_f64));
	assert_eq!(a.y, -4.2_f64);
	assert!(equal(a.z, 3.1_f64));
	assert!(equal(a.w, 1.0_f64));
	assert!(a.is_point());
	assert!(!a.is_vector());
}

#[test]
fn test_tuples_01_a_tuple_with_w_0_is_a_vector() {
    // A tuple with w=0 is a vector
	let a = tuple(4.3_f64, -4.2_f64, 3.1_f64, 0.0_f64);
	assert!(equal(a.x, 4.3_f64));
	assert_eq!(a.y, -4.2_f64);
	assert!(equal(a.z, 3.1_f64));
	assert!(equal(a.w, 0.0_f64));
	assert!(!a.is_point());
	assert!(a.is_vector());
}

#[test]
fn test_tuples_02_point_creates_tuples_with_w_1() {
    // point() creates tuples with w=1
	let p = point(4_f64, -4_f64, 3_f64);
	assert_eq!(p, tuple(4_f64, -4_f64, 3_f64, 1_f64));
}

#[test]
fn test_tuples_03_vector_creates_tuples_with_w_0() {
    // vector() creates tuples with w=0
	let v = vector(4_f64, -4_f64, 3_f64);
	assert_eq!(v, tuple(4_f64, -4_f64, 3_f64, 0_f64));
}

#[test]
fn test_tuples_04_adding_two_tuples() {
    // Adding two tuples
	let a1 = tuple(3_f64, -2_f64, 5_f64, 1_f64);
	let a2 = tuple(-2_f64, 3_f64, 1_f64, 0_f64);
	assert_eq!(a1 + a2, tuple(1_f64, 1_f64, 6_f64, 1_f64));
}

#[test]
fn test_tuples_05_subtracting_two_points() {
    // Subtracting two points
	let p1 = point(3_f64, 2_f64, 1_f64);
	let p2 = point(5_f64, 6_f64, 7_f64);
	assert_eq!(p1 - p2, vector(-2_f64, -4_f64, -6_f64));
}

#[test]
fn test_tuples_06_subtracting_a_vector_from_a_point() {
    // Subtracting a vector from a point
	let p = point(3_f64, 2_f64, 1_f64);
	let v = vector(5_f64, 6_f64, 7_f64);
	assert_eq!(p - v, point(-2_f64, -4_f64, -6_f64));
}

#[test]
fn test_tuples_07_subtracting_two_vectors() {
    // Subtracting two vectors
	let v1 = vector(3_f64, 2_f64, 1_f64);
	let v2 = vector(5_f64, 6_f64, 7_f64);
	assert_eq!(v1 - v2, vector(-2_f64, -4_f64, -6_f64));
}

#[test]
fn test_tuples_08_subtracting_a_vector_from_the_zero_vector() {
    // Subtracting a vector from the zero vector
	let zero = vector(0_f64, 0_f64, 0_f64);
	let v = vector(1_f64, -2_f64, 3_f64);
	assert_eq!(zero - v, vector(-1_f64, 2_f64, -3_f64));
}

#[test]
fn test_tuples_09_negating_a_tuple() {
    // Negating a tuple
	let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
	assert_eq!(-a, tuple(-1_f64, 2_f64, -3_f64, 4_f64));
}

#[test]
fn test_tuples_10_multiplying_a_tuple_by_a_scalar() {
    // Multiplying a tuple by a scalar
	let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
	assert_eq!(a * 3.5_f64, tuple(3.5_f64, -7_f64, 10.5_f64, -14_f64));
}

#[test]
fn test_tuples_11_multiplying_a_tuple_by_a_fraction() {
    // Multiplying a tuple by a fraction
	let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
	assert_eq!(a * 0.5_f64, tuple(0.5_f64, -1_f64, 1.5_f64, -2_f64));
}

#[test]
fn test_tuples_12_dividing_a_tuple_by_a_scalar() {
    // Dividing a tuple by a scalar
	let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
	assert_eq!(a / 2_f64, tuple(0.5_f64, -1_f64, 1.5_f64, -2_f64));
}

#[test]
fn test_tuples_13_computing_the_magnitude_of_vector_1_0_0_() {
    // Computing the magnitude of vector(1, 0, 0)
	let v = vector(1_f64, 0_f64, 0_f64);
	assert!(equal(magnitude(&v), 1_f64));
}

#[test]
fn test_tuples_14_computing_the_magnitude_of_vector_0_1_0_() {
    // Computing the magnitude of vector(0, 1, 0)
	let v = vector(0_f64, 1_f64, 0_f64);
	assert!(equal(magnitude(&v), 1_f64));
}

#[test]
fn test_tuples_15_computing_the_magnitude_of_vector_0_0_1_() {
    // Computing the magnitude of vector(0, 0, 1)
	let v = vector(0_f64, 0_f64, 1_f64);
	assert!(equal(magnitude(&v), 1_f64));
}

#[test]
fn test_tuples_16_computing_the_magnitude_of_vector_1_2_3_() {
    // Computing the magnitude of vector(1, 2, 3)
	let v = vector(1_f64, 2_f64, 3_f64);
	assert_eq!(magnitude(&v), 14_f64.sqrt());
}

#[test]
fn test_tuples_17_computing_the_magnitude_of_vector_1_2_3_() {
    // Computing the magnitude of vector(-1, -2, -3)
	let v = vector(-1_f64, -2_f64, -3_f64);
	assert_eq!(magnitude(&v), 14_f64.sqrt());
}

#[test]
fn test_tuples_18_normalizing_vector_4_0_0_gives_1_0_0_() {
    // Normalizing vector(4, 0, 0) gives (1, 0, 0)
	let v = vector(4_f64, 0_f64, 0_f64);
	assert_eq!(normalize(&v), vector(1_f64, 0_f64, 0_f64));
}

#[test]
fn test_tuples_19_normalizing_vector_1_2_3_() {
    // Normalizing vector(1, 2, 3)
	let v = vector(1_f64, 2_f64, 3_f64);
	assert_eq!(normalize(&v), vector(0.26726_f64, 0.53452_f64, 0.80178_f64));
}

#[test]
fn test_tuples_20_the_magnitude_of_a_normalized_vector() {
    // The magnitude of a normalized vector
	let v = vector(1_f64, 2_f64, 3_f64);
	let norm = normalize(&v);
	assert!(equal(magnitude(&norm), 1_f64));
}

#[test]
fn test_tuples_21_the_dot_product_of_two_tuples() {
    // The dot product of two tuples
	let a = vector(1_f64, 2_f64, 3_f64);
	let b = vector(2_f64, 3_f64, 4_f64);
	assert!(equal(dot(&a, &b), 20_f64));
}

#[test]
fn test_tuples_22_the_cross_product_of_two_vectors() {
    // The cross product of two vectors
	let a = vector(1_f64, 2_f64, 3_f64);
	let b = vector(2_f64, 3_f64, 4_f64);
	assert_eq!(cross(&a, &b), vector(-1_f64, 2_f64, -1_f64));
	assert_eq!(cross(&b, &a), vector(1_f64, -2_f64, 1_f64));
}

#[test]
fn test_tuples_23_colors_are_red_green_blue_tuples() {
    // Colors are (red, green, blue) tuples
	let mut c = color(-0.5_f64, 0.4_f64, 1.7_f64);
	assert_eq!(c.red, -0.5_f64);
	assert!(equal(c.green, 0.4_f64));
	assert!(equal(c.blue, 1.7_f64));
}

#[test]
fn test_tuples_24_adding_colors() {
    // Adding colors
	let c1 = color(0.9_f64, 0.6_f64, 0.75_f64);
	let c2 = color(0.7_f64, 0.1_f64, 0.25_f64);
	assert_eq!(c1 + c2, color(1.6_f64, 0.7_f64, 1.0_f64));
}

#[test]
fn test_tuples_25_subtracting_colors() {
    // Subtracting colors
	let c1 = color(0.9_f64, 0.6_f64, 0.75_f64);
	let c2 = color(0.7_f64, 0.1_f64, 0.25_f64);
	assert_eq!(c1 - c2, color(0.2_f64, 0.5_f64, 0.5_f64));
}

#[test]
fn test_tuples_26_multiplying_a_color_by_a_scalar() {
    // Multiplying a color by a scalar
	let mut c = color(0.2_f64, 0.3_f64, 0.4_f64);
	assert_eq!(c * 2_f64, color(0.4_f64, 0.6_f64, 0.8_f64));
}

#[test]
fn test_tuples_27_multiplying_colors() {
    // Multiplying colors
	let c1 = color(1_f64, 0.2_f64, 0.4_f64);
	let c2 = color(0.9_f64, 1_f64, 0.1_f64);
	assert_eq!(c1 * c2, color(0.9_f64, 0.2_f64, 0.04_f64));
}

#[test]
fn test_tuples_28_reflecting_a_vector_approaching_at_45_() {
    // Reflecting a vector approaching at 45°
	let v = vector(1_f64, -1_f64, 0_f64);
	let n = vector(0_f64, 1_f64, 0_f64);
	let r = reflect(&v, &n);
	assert_eq!(r, vector(1_f64, 1_f64, 0_f64));
}

#[test]
fn test_tuples_29_reflecting_a_vector_off_a_slanted_surface() {
    // Reflecting a vector off a slanted surface
	let v = vector(0_f64, -1_f64, 0_f64);
	let n = vector(2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64, 0_f64);
	let r = reflect(&v, &n);
	assert_eq!(r, vector(1_f64, 0_f64, 0_f64));
}
