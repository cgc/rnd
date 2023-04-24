
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_patterns_00_creating_a_stripe_pattern() {
    // Creating a stripe pattern
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = stripe_pattern(&white, &black);
	assert_eq!(pattern.a, white);
	assert_eq!(pattern.b, black);
}

#[test]
fn test_patterns_01_a_stripe_pattern_is_constant_in_y() {
    // A stripe pattern is constant in y
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = stripe_pattern(&white, &black);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 1_f64, 0_f64)), white);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 2_f64, 0_f64)), white);
}

#[test]
fn test_patterns_02_a_stripe_pattern_is_constant_in_z() {
    // A stripe pattern is constant in z
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = stripe_pattern(&white, &black);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 0_f64, 1_f64)), white);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 0_f64, 2_f64)), white);
}

#[test]
fn test_patterns_03_a_stripe_pattern_alternates_in_x() {
    // A stripe pattern alternates in x
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = stripe_pattern(&white, &black);
	assert_eq!(stripe_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(stripe_at(&pattern, &point(0.9_f64, 0_f64, 0_f64)), white);
	assert_eq!(stripe_at(&pattern, &point(1_f64, 0_f64, 0_f64)), black);
	assert_eq!(stripe_at(&pattern, &point(-0.1_f64, 0_f64, 0_f64)), black);
	assert_eq!(stripe_at(&pattern, &point(-1_f64, 0_f64, 0_f64)), black);
	assert_eq!(stripe_at(&pattern, &point(-1.1_f64, 0_f64, 0_f64)), white);
}

#[test]
fn test_patterns_04_stripes_with_an_object_transformation() {
    // Stripes with an object transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut object = sphere();
	set_transform(&mut object, &scaling(2_f64, 2_f64, 2_f64));
	let mut pattern = stripe_pattern(&white, &black);
	let mut c = stripe_at_object(&pattern, &object, &point(1.5_f64, 0_f64, 0_f64));
	assert_eq!(c, white);
}

#[test]
fn test_patterns_05_stripes_with_a_pattern_transformation() {
    // Stripes with a pattern transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut object = sphere();
	let mut pattern = stripe_pattern(&white, &black);
	set_pattern_transform(&mut pattern, &scaling(2_f64, 2_f64, 2_f64));
	let mut c = stripe_at_object(&pattern, &object, &point(1.5_f64, 0_f64, 0_f64));
	assert_eq!(c, white);
}

#[test]
fn test_patterns_06_stripes_with_both_an_object_and_a_pattern_transformation() {
    // Stripes with both an object and a pattern transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut object = sphere();
	set_transform(&mut object, &scaling(2_f64, 2_f64, 2_f64));
	let mut pattern = stripe_pattern(&white, &black);
	set_pattern_transform(&mut pattern, &translation(0.5_f64, 0_f64, 0_f64));
	let mut c = stripe_at_object(&pattern, &object, &point(2.5_f64, 0_f64, 0_f64));
	assert_eq!(c, white);
}

#[test]
fn test_patterns_07_the_default_pattern_transformation() {
    // The default pattern transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = test_pattern();
	assert_eq!(pattern.transform(), identity_matrix);
}

#[test]
fn test_patterns_08_assigning_a_transformation() {
    // Assigning a transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = test_pattern();
	set_pattern_transform(&mut pattern, &translation(1_f64, 2_f64, 3_f64));
	assert_eq!(pattern.transform(), translation(1_f64, 2_f64, 3_f64));
}

#[test]
fn test_patterns_09_a_pattern_with_an_object_transformation() {
    // A pattern with an object transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut shape = sphere();
	set_transform(&mut shape, &scaling(2_f64, 2_f64, 2_f64));
	let mut pattern = test_pattern();
	let mut c = pattern_at_shape(&pattern, &shape, &point(2_f64, 3_f64, 4_f64));
	assert_eq!(c, color(1_f64, 1.5_f64, 2_f64));
}

#[test]
fn test_patterns_10_a_pattern_with_a_pattern_transformation() {
    // A pattern with a pattern transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut shape = sphere();
	let mut pattern = test_pattern();
	set_pattern_transform(&mut pattern, &scaling(2_f64, 2_f64, 2_f64));
	let mut c = pattern_at_shape(&pattern, &shape, &point(2_f64, 3_f64, 4_f64));
	assert_eq!(c, color(1_f64, 1.5_f64, 2_f64));
}

#[test]
fn test_patterns_11_a_pattern_with_both_an_object_and_a_pattern_transformation() {
    // A pattern with both an object and a pattern transformation
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut shape = sphere();
	set_transform(&mut shape, &scaling(2_f64, 2_f64, 2_f64));
	let mut pattern = test_pattern();
	set_pattern_transform(&mut pattern, &translation(0.5_f64, 1_f64, 1.5_f64));
	let mut c = pattern_at_shape(&pattern, &shape, &point(2.5_f64, 3_f64, 3.5_f64));
	assert_eq!(c, color(0.75_f64, 0.5_f64, 0.25_f64));
}

#[test]
fn test_patterns_12_a_gradient_linearly_interpolates_between_colors() {
    // A gradient linearly interpolates between colors
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = gradient_pattern(&white, &black);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(0.25_f64, 0_f64, 0_f64)), color(0.75_f64, 0.75_f64, 0.75_f64));
	assert_eq!(pattern_at(&pattern, &point(0.5_f64, 0_f64, 0_f64)), color(0.5_f64, 0.5_f64, 0.5_f64));
	assert_eq!(pattern_at(&pattern, &point(0.75_f64, 0_f64, 0_f64)), color(0.25_f64, 0.25_f64, 0.25_f64));
}

#[test]
fn test_patterns_13_a_ring_should_extend_in_both_x_and_z() {
    // A ring should extend in both x and z
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = ring_pattern(&white, &black);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(1_f64, 0_f64, 0_f64)), black);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 1_f64)), black);
	assert_eq!(pattern_at(&pattern, &point(0.708_f64, 0_f64, 0.708_f64)), black);
}

#[test]
fn test_patterns_14_checkers_should_repeat_in_x() {
    // Checkers should repeat in x
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = checkers_pattern(&white, &black);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(0.99_f64, 0_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(1.01_f64, 0_f64, 0_f64)), black);
}

#[test]
fn test_patterns_15_checkers_should_repeat_in_y() {
    // Checkers should repeat in y
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = checkers_pattern(&white, &black);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0.99_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 1.01_f64, 0_f64)), black);
}

#[test]
fn test_patterns_16_checkers_should_repeat_in_z() {
    // Checkers should repeat in z
	let black = color(0_f64, 0_f64, 0_f64);
	let white = color(1_f64, 1_f64, 1_f64);
	let mut pattern = checkers_pattern(&white, &black);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 0_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 0.99_f64)), white);
	assert_eq!(pattern_at(&pattern, &point(0_f64, 0_f64, 1.01_f64)), black);
}
