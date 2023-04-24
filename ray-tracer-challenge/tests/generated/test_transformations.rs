
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_transformations_00_multiplying_by_a_translation_matrix() {
    // Multiplying by a translation matrix
	let transform = translation(5_f64, -3_f64, 2_f64);
	let p = point(-3_f64, 4_f64, 5_f64);
	assert_eq!(transform * p, point(2_f64, 1_f64, 7_f64));
}

#[test]
fn test_transformations_01_multiplying_by_the_inverse_of_a_translation_matrix() {
    // Multiplying by the inverse of a translation matrix
	let transform = translation(5_f64, -3_f64, 2_f64);
	let inv = inverse(&transform);
	let p = point(-3_f64, 4_f64, 5_f64);
	assert_eq!(inv * p, point(-8_f64, 7_f64, 3_f64));
}

#[test]
fn test_transformations_02_translation_does_not_affect_vectors() {
    // Translation does not affect vectors
	let transform = translation(5_f64, -3_f64, 2_f64);
	let v = vector(-3_f64, 4_f64, 5_f64);
	assert_eq!(transform * v, v);
}

#[test]
fn test_transformations_03_a_scaling_matrix_applied_to_a_point() {
    // A scaling matrix applied to a point
	let transform = scaling(2_f64, 3_f64, 4_f64);
	let p = point(-4_f64, 6_f64, 8_f64);
	assert_eq!(transform * p, point(-8_f64, 18_f64, 32_f64));
}

#[test]
fn test_transformations_04_a_scaling_matrix_applied_to_a_vector() {
    // A scaling matrix applied to a vector
	let transform = scaling(2_f64, 3_f64, 4_f64);
	let v = vector(-4_f64, 6_f64, 8_f64);
	assert_eq!(transform * v, vector(-8_f64, 18_f64, 32_f64));
}

#[test]
fn test_transformations_05_multiplying_by_the_inverse_of_a_scaling_matrix() {
    // Multiplying by the inverse of a scaling matrix
	let transform = scaling(2_f64, 3_f64, 4_f64);
	let inv = inverse(&transform);
	let v = vector(-4_f64, 6_f64, 8_f64);
	assert_eq!(inv * v, vector(-2_f64, 2_f64, 2_f64));
}

#[test]
fn test_transformations_06_reflection_is_scaling_by_a_negative_value() {
    // Reflection is scaling by a negative value
	let transform = scaling(-1_f64, 1_f64, 1_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(-2_f64, 3_f64, 4_f64));
}

#[test]
fn test_transformations_07_rotating_a_point_around_the_x_axis() {
    // Rotating a point around the x axis
	let p = point(0_f64, 1_f64, 0_f64);
	let half_quarter = rotation_x(PI / 4_f64);
	let full_quarter = rotation_x(PI / 2_f64);
	assert_eq!(half_quarter * p, point(0_f64, 2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	assert_eq!(full_quarter * p, point(0_f64, 0_f64, 1_f64));
}

#[test]
fn test_transformations_08_the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    // The inverse of an x-rotation rotates in the opposite direction
	let p = point(0_f64, 1_f64, 0_f64);
	let half_quarter = rotation_x(PI / 4_f64);
	let inv = inverse(&half_quarter);
	assert_eq!(inv * p, point(0_f64, 2_f64.sqrt() / 2_f64, -2_f64.sqrt() / 2_f64));
}

#[test]
fn test_transformations_09_rotating_a_point_around_the_y_axis() {
    // Rotating a point around the y axis
	let p = point(0_f64, 0_f64, 1_f64);
	let half_quarter = rotation_y(PI / 4_f64);
	let full_quarter = rotation_y(PI / 2_f64);
	assert_eq!(half_quarter * p, point(2_f64.sqrt() / 2_f64, 0_f64, 2_f64.sqrt() / 2_f64));
	assert_eq!(full_quarter * p, point(1_f64, 0_f64, 0_f64));
}

#[test]
fn test_transformations_10_rotating_a_point_around_the_z_axis() {
    // Rotating a point around the z axis
	let p = point(0_f64, 1_f64, 0_f64);
	let half_quarter = rotation_z(PI / 4_f64);
	let full_quarter = rotation_z(PI / 2_f64);
	assert_eq!(half_quarter * p, point(-2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64, 0_f64));
	assert_eq!(full_quarter * p, point(-1_f64, 0_f64, 0_f64));
}

#[test]
fn test_transformations_11_a_shearing_transformation_moves_x_in_proportion_to_y() {
    // A shearing transformation moves x in proportion to y
	let transform = shearing(1_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(5_f64, 3_f64, 4_f64));
}

#[test]
fn test_transformations_12_a_shearing_transformation_moves_x_in_proportion_to_z() {
    // A shearing transformation moves x in proportion to z
	let transform = shearing(0_f64, 1_f64, 0_f64, 0_f64, 0_f64, 0_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(6_f64, 3_f64, 4_f64));
}

#[test]
fn test_transformations_13_a_shearing_transformation_moves_y_in_proportion_to_x() {
    // A shearing transformation moves y in proportion to x
	let transform = shearing(0_f64, 0_f64, 1_f64, 0_f64, 0_f64, 0_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(2_f64, 5_f64, 4_f64));
}

#[test]
fn test_transformations_14_a_shearing_transformation_moves_y_in_proportion_to_z() {
    // A shearing transformation moves y in proportion to z
	let transform = shearing(0_f64, 0_f64, 0_f64, 1_f64, 0_f64, 0_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(2_f64, 7_f64, 4_f64));
}

#[test]
fn test_transformations_15_a_shearing_transformation_moves_z_in_proportion_to_x() {
    // A shearing transformation moves z in proportion to x
	let transform = shearing(0_f64, 0_f64, 0_f64, 0_f64, 1_f64, 0_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(2_f64, 3_f64, 6_f64));
}

#[test]
fn test_transformations_16_a_shearing_transformation_moves_z_in_proportion_to_y() {
    // A shearing transformation moves z in proportion to y
	let transform = shearing(0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 1_f64);
	let p = point(2_f64, 3_f64, 4_f64);
	assert_eq!(transform * p, point(2_f64, 3_f64, 7_f64));
}

#[test]
fn test_transformations_17_individual_transformations_are_applied_in_sequence() {
    // Individual transformations are applied in sequence
	let p = point(1_f64, 0_f64, 1_f64);
	let mut A = rotation_x(PI / 2_f64);
	let mut B = scaling(5_f64, 5_f64, 5_f64);
	let mut C = translation(10_f64, 5_f64, 7_f64);
	let p2 = A * p;
	assert_eq!(p2, point(1_f64, -1_f64, 0_f64));
	let p3 = B * p2;
	assert_eq!(p3, point(5_f64, -5_f64, 0_f64));
	let p4 = C * p3;
	assert_eq!(p4, point(15_f64, 0_f64, 7_f64));
}

#[test]
fn test_transformations_18_chained_transformations_must_be_applied_in_reverse_order() {
    // Chained transformations must be applied in reverse order
	let p = point(1_f64, 0_f64, 1_f64);
	let mut A = rotation_x(PI / 2_f64);
	let mut B = scaling(5_f64, 5_f64, 5_f64);
	let mut C = translation(10_f64, 5_f64, 7_f64);
	let T = C * B * A;
	assert_eq!(T * p, point(15_f64, 0_f64, 7_f64));
}

#[test]
fn test_transformations_19_the_transformation_matrix_for_the_default_orientation() {
    // The transformation matrix for the default orientation
	let from_ = point(0_f64, 0_f64, 0_f64);
	let to = point(0_f64, 0_f64, -1_f64);
	let up = vector(0_f64, 1_f64, 0_f64);
	let t = view_transform(&from_, &to, &up);
	assert_eq!(t, identity_matrix);
}

#[test]
fn test_transformations_20_a_view_transformation_matrix_looking_in_positive_z_direction() {
    // A view transformation matrix looking in positive z direction
	let from_ = point(0_f64, 0_f64, 0_f64);
	let to = point(0_f64, 0_f64, 1_f64);
	let up = vector(0_f64, 1_f64, 0_f64);
	let t = view_transform(&from_, &to, &up);
	assert_eq!(t, scaling(-1_f64, 1_f64, -1_f64));
}

#[test]
fn test_transformations_21_the_view_transformation_moves_the_world() {
    // The view transformation moves the world
	let from_ = point(0_f64, 0_f64, 8_f64);
	let to = point(0_f64, 0_f64, 0_f64);
	let up = vector(0_f64, 1_f64, 0_f64);
	let t = view_transform(&from_, &to, &up);
	assert_eq!(t, translation(0_f64, 0_f64, -8_f64));
}

#[test]
fn test_transformations_22_an_arbitrary_view_transformation() {
    // An arbitrary view transformation
	let from_ = point(1_f64, 3_f64, 2_f64);
	let to = point(4_f64, -2_f64, 8_f64);
	let up = vector(1_f64, 1_f64, 0_f64);
	let t = view_transform(&from_, &to, &up);
	assert_eq!(t, matrix4([[-0.50709_f64, 0.50709_f64, 0.67612_f64, -2.36643_f64], [0.76772_f64, 0.60609_f64, 0.12122_f64, -2.82843_f64], [-0.35857_f64, 0.59761_f64, -0.71714_f64, 0.0_f64], [0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64]]));
}
