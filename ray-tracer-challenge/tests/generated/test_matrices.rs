
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_matrices_00_constructing_and_inspecting_a_4x4_matrix() {
    // Constructing and inspecting a 4x4 matrix
	let M = matrix4([[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64], [5.5_f64, 6.5_f64, 7.5_f64, 8.5_f64], [9.0_f64, 10.0_f64, 11.0_f64, 12.0_f64], [13.5_f64, 14.5_f64, 15.5_f64, 16.5_f64]]);
	assert!(equal(M[(0, 0)], 1_f64));
	assert!(equal(M[(0, 3)], 4_f64));
	assert!(equal(M[(1, 0)], 5.5_f64));
	assert!(equal(M[(1, 2)], 7.5_f64));
	assert!(equal(M[(2, 2)], 11_f64));
	assert!(equal(M[(3, 0)], 13.5_f64));
	assert!(equal(M[(3, 2)], 15.5_f64));
}

#[test]
fn test_matrices_01_a_2x2_matrix_ought_to_be_representable() {
    // A 2x2 matrix ought to be representable
	let M = matrix2([[-3.0_f64, 5.0_f64], [1.0_f64, -2.0_f64]]);
	assert_eq!(M[(0, 0)], -3_f64);
	assert!(equal(M[(0, 1)], 5_f64));
	assert!(equal(M[(1, 0)], 1_f64));
	assert_eq!(M[(1, 1)], -2_f64);
}

#[test]
fn test_matrices_02_a_3x3_matrix_ought_to_be_representable() {
    // A 3x3 matrix ought to be representable
	let M = matrix3([[-3.0_f64, 5.0_f64, 0.0_f64], [1.0_f64, -2.0_f64, -7.0_f64], [0.0_f64, 1.0_f64, 1.0_f64]]);
	assert_eq!(M[(0, 0)], -3_f64);
	assert_eq!(M[(1, 1)], -2_f64);
	assert!(equal(M[(2, 2)], 1_f64));
}

#[test]
fn test_matrices_03_matrix_equality_with_identical_matrices() {
    // Matrix equality with identical matrices
	let mut A = matrix4([[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64], [5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64], [9.0_f64, 8.0_f64, 7.0_f64, 6.0_f64], [5.0_f64, 4.0_f64, 3.0_f64, 2.0_f64]]);
	let mut B = matrix4([[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64], [5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64], [9.0_f64, 8.0_f64, 7.0_f64, 6.0_f64], [5.0_f64, 4.0_f64, 3.0_f64, 2.0_f64]]);
	assert_eq!(A, B);
}

#[test]
fn test_matrices_04_matrix_equality_with_different_matrices() {
    // Matrix equality with different matrices
	let mut A = matrix4([[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64], [5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64], [9.0_f64, 8.0_f64, 7.0_f64, 6.0_f64], [5.0_f64, 4.0_f64, 3.0_f64, 2.0_f64]]);
	let mut B = matrix4([[2.0_f64, 3.0_f64, 4.0_f64, 5.0_f64], [6.0_f64, 7.0_f64, 8.0_f64, 9.0_f64], [8.0_f64, 7.0_f64, 6.0_f64, 5.0_f64], [4.0_f64, 3.0_f64, 2.0_f64, 1.0_f64]]);
	assert_ne!(A, B);
}

#[test]
fn test_matrices_05_multiplying_two_matrices() {
    // Multiplying two matrices
	let mut A = matrix4([[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64], [5.0_f64, 6.0_f64, 7.0_f64, 8.0_f64], [9.0_f64, 8.0_f64, 7.0_f64, 6.0_f64], [5.0_f64, 4.0_f64, 3.0_f64, 2.0_f64]]);
	let mut B = matrix4([[-2.0_f64, 1.0_f64, 2.0_f64, 3.0_f64], [3.0_f64, 2.0_f64, 1.0_f64, -1.0_f64], [4.0_f64, 3.0_f64, 6.0_f64, 5.0_f64], [1.0_f64, 2.0_f64, 7.0_f64, 8.0_f64]]);
	assert_eq!(A * B, matrix4([[20.0_f64, 22.0_f64, 50.0_f64, 48.0_f64], [44.0_f64, 54.0_f64, 114.0_f64, 108.0_f64], [40.0_f64, 58.0_f64, 110.0_f64, 102.0_f64], [16.0_f64, 26.0_f64, 46.0_f64, 42.0_f64]]));
}

#[test]
fn test_matrices_06_a_matrix_multiplied_by_a_tuple() {
    // A matrix multiplied by a tuple
	let mut A = matrix4([[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64], [2.0_f64, 4.0_f64, 4.0_f64, 2.0_f64], [8.0_f64, 6.0_f64, 4.0_f64, 1.0_f64], [0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64]]);
	let b = tuple(1_f64, 2_f64, 3_f64, 1_f64);
	assert_eq!(A * b, tuple(18_f64, 24_f64, 33_f64, 1_f64));
}

#[test]
fn test_matrices_07_multiplying_a_matrix_by_the_identity_matrix() {
    // Multiplying a matrix by the identity matrix
	let mut A = matrix4([[0.0_f64, 1.0_f64, 2.0_f64, 4.0_f64], [1.0_f64, 2.0_f64, 4.0_f64, 8.0_f64], [2.0_f64, 4.0_f64, 8.0_f64, 16.0_f64], [4.0_f64, 8.0_f64, 16.0_f64, 32.0_f64]]);
	assert_eq!(A * identity_matrix, A);
}

#[test]
fn test_matrices_08_multiplying_the_identity_matrix_by_a_tuple() {
    // Multiplying the identity matrix by a tuple
	let a = tuple(1_f64, 2_f64, 3_f64, 4_f64);
	assert_eq!(identity_matrix * a, a);
}

#[test]
fn test_matrices_09_transposing_a_matrix() {
    // Transposing a matrix
	let mut A = matrix4([[0.0_f64, 9.0_f64, 3.0_f64, 0.0_f64], [9.0_f64, 8.0_f64, 0.0_f64, 8.0_f64], [1.0_f64, 8.0_f64, 5.0_f64, 3.0_f64], [0.0_f64, 0.0_f64, 5.0_f64, 8.0_f64]]);
	assert_eq!(transpose(&A), matrix4([[0.0_f64, 9.0_f64, 1.0_f64, 0.0_f64], [9.0_f64, 8.0_f64, 8.0_f64, 0.0_f64], [3.0_f64, 0.0_f64, 5.0_f64, 5.0_f64], [0.0_f64, 8.0_f64, 3.0_f64, 8.0_f64]]));
}

#[test]
fn test_matrices_10_transposing_the_identity_matrix() {
    // Transposing the identity matrix
	let mut A = transpose(&identity_matrix);
	assert_eq!(A, identity_matrix);
}

#[test]
fn test_matrices_11_calculating_the_determinant_of_a_2x2_matrix() {
    // Calculating the determinant of a 2x2 matrix
	let mut A = matrix2([[1.0_f64, 5.0_f64], [-3.0_f64, 2.0_f64]]);
	assert!(equal(determinant(&A), 17_f64));
}

#[test]
fn test_matrices_12_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    // A submatrix of a 3x3 matrix is a 2x2 matrix
	let mut A = matrix3([[1.0_f64, 5.0_f64, 0.0_f64], [-3.0_f64, 2.0_f64, 7.0_f64], [0.0_f64, 6.0_f64, -3.0_f64]]);
	assert_eq!(submatrix(&A, 0, 2), matrix2([[-3.0_f64, 2.0_f64], [0.0_f64, 6.0_f64]]));
}

#[test]
fn test_matrices_13_a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    // A submatrix of a 4x4 matrix is a 3x3 matrix
	let mut A = matrix4([[-6.0_f64, 1.0_f64, 1.0_f64, 6.0_f64], [-8.0_f64, 5.0_f64, 8.0_f64, 6.0_f64], [-1.0_f64, 0.0_f64, 8.0_f64, 2.0_f64], [-7.0_f64, 1.0_f64, -1.0_f64, 1.0_f64]]);
	assert_eq!(submatrix(&A, 2, 1), matrix3([[-6.0_f64, 1.0_f64, 6.0_f64], [-8.0_f64, 8.0_f64, 6.0_f64], [-7.0_f64, -1.0_f64, 1.0_f64]]));
}

#[test]
fn test_matrices_14_calculating_a_minor_of_a_3x3_matrix() {
    // Calculating a minor of a 3x3 matrix
	let mut A = matrix3([[3.0_f64, 5.0_f64, 0.0_f64], [2.0_f64, -1.0_f64, -7.0_f64], [6.0_f64, -1.0_f64, 5.0_f64]]);
	let mut B = submatrix(&A, 1, 0);
	assert!(equal(determinant(&B), 25_f64));
	assert!(equal(minor(&A, 1, 0), 25_f64));
}

#[test]
fn test_matrices_15_calculating_a_cofactor_of_a_3x3_matrix() {
    // Calculating a cofactor of a 3x3 matrix
	let mut A = matrix3([[3.0_f64, 5.0_f64, 0.0_f64], [2.0_f64, -1.0_f64, -7.0_f64], [6.0_f64, -1.0_f64, 5.0_f64]]);
	assert_eq!(minor(&A, 0, 0), -12_f64);
	assert_eq!(cofactor(&A, 0, 0), -12_f64);
	assert!(equal(minor(&A, 1, 0), 25_f64));
	assert_eq!(cofactor(&A, 1, 0), -25_f64);
}

#[test]
fn test_matrices_16_calculating_the_determinant_of_a_3x3_matrix() {
    // Calculating the determinant of a 3x3 matrix
	let mut A = matrix3([[1.0_f64, 2.0_f64, 6.0_f64], [-5.0_f64, 8.0_f64, -4.0_f64], [2.0_f64, 6.0_f64, 4.0_f64]]);
	assert!(equal(cofactor(&A, 0, 0), 56_f64));
	assert!(equal(cofactor(&A, 0, 1), 12_f64));
	assert_eq!(cofactor(&A, 0, 2), -46_f64);
	assert_eq!(determinant(&A), -196_f64);
}

#[test]
fn test_matrices_17_calculating_the_determinant_of_a_4x4_matrix() {
    // Calculating the determinant of a 4x4 matrix
	let mut A = matrix4([[-2.0_f64, -8.0_f64, 3.0_f64, 5.0_f64], [-3.0_f64, 1.0_f64, 7.0_f64, 3.0_f64], [1.0_f64, 2.0_f64, -9.0_f64, 6.0_f64], [-6.0_f64, 7.0_f64, 7.0_f64, -9.0_f64]]);
	assert!(equal(cofactor(&A, 0, 0), 690_f64));
	assert!(equal(cofactor(&A, 0, 1), 447_f64));
	assert!(equal(cofactor(&A, 0, 2), 210_f64));
	assert!(equal(cofactor(&A, 0, 3), 51_f64));
	assert_eq!(determinant(&A), -4071_f64);
}

#[test]
fn test_matrices_18_testing_an_invertible_matrix_for_invertibility() {
    // Testing an invertible matrix for invertibility
	let mut A = matrix4([[6.0_f64, 4.0_f64, 4.0_f64, 4.0_f64], [5.0_f64, 5.0_f64, 7.0_f64, 6.0_f64], [4.0_f64, -9.0_f64, 3.0_f64, -7.0_f64], [9.0_f64, 1.0_f64, 7.0_f64, -6.0_f64]]);
	assert_eq!(determinant(&A), -2120_f64);
	assert!(A.is_invertible());
}

#[test]
fn test_matrices_19_testing_a_noninvertible_matrix_for_invertibility() {
    // Testing a noninvertible matrix for invertibility
	let mut A = matrix4([[-4.0_f64, 2.0_f64, -2.0_f64, -3.0_f64], [9.0_f64, 6.0_f64, 2.0_f64, 6.0_f64], [0.0_f64, -5.0_f64, 1.0_f64, -5.0_f64], [0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]]);
	assert!(equal(determinant(&A), 0_f64));
	assert!(!A.is_invertible());
}

#[test]
fn test_matrices_20_calculating_the_inverse_of_a_matrix() {
    // Calculating the inverse of a matrix
	let mut A = matrix4([[-5.0_f64, 2.0_f64, 6.0_f64, -8.0_f64], [1.0_f64, -5.0_f64, 1.0_f64, 8.0_f64], [7.0_f64, 7.0_f64, -6.0_f64, -7.0_f64], [1.0_f64, -3.0_f64, 7.0_f64, 4.0_f64]]);
	let mut B = inverse(&A);
	assert!(equal(determinant(&A), 532_f64));
	assert_eq!(cofactor(&A, 2, 3), -160_f64);
	assert_eq!(B[(3, 2)], -160_f64 / 532_f64);
	assert!(equal(cofactor(&A, 3, 2), 105_f64));
	assert_eq!(B[(2, 3)], 105_f64 / 532_f64);
	assert_eq!(B, matrix4([[0.21805_f64, 0.45113_f64, 0.2406_f64, -0.04511_f64], [-0.80827_f64, -1.45677_f64, -0.44361_f64, 0.52068_f64], [-0.07895_f64, -0.22368_f64, -0.05263_f64, 0.19737_f64], [-0.52256_f64, -0.81391_f64, -0.30075_f64, 0.30639_f64]]));
}

#[test]
fn test_matrices_21_calculating_the_inverse_of_another_matrix() {
    // Calculating the inverse of another matrix
	let mut A = matrix4([[8.0_f64, -5.0_f64, 9.0_f64, 2.0_f64], [7.0_f64, 5.0_f64, 6.0_f64, 1.0_f64], [-6.0_f64, 0.0_f64, 9.0_f64, 6.0_f64], [-3.0_f64, 0.0_f64, -9.0_f64, -4.0_f64]]);
	assert_eq!(inverse(&A), matrix4([[-0.15385_f64, -0.15385_f64, -0.28205_f64, -0.53846_f64], [-0.07692_f64, 0.12308_f64, 0.02564_f64, 0.03077_f64], [0.35897_f64, 0.35897_f64, 0.4359_f64, 0.92308_f64], [-0.69231_f64, -0.69231_f64, -0.76923_f64, -1.92308_f64]]));
}

#[test]
fn test_matrices_22_calculating_the_inverse_of_a_third_matrix() {
    // Calculating the inverse of a third matrix
	let mut A = matrix4([[9.0_f64, 3.0_f64, 0.0_f64, 9.0_f64], [-5.0_f64, -2.0_f64, -6.0_f64, -3.0_f64], [-4.0_f64, 9.0_f64, 6.0_f64, 4.0_f64], [-7.0_f64, 6.0_f64, 6.0_f64, 2.0_f64]]);
	assert_eq!(inverse(&A), matrix4([[-0.04074_f64, -0.07778_f64, 0.14444_f64, -0.22222_f64], [-0.07778_f64, 0.03333_f64, 0.36667_f64, -0.33333_f64], [-0.02901_f64, -0.1463_f64, -0.10926_f64, 0.12963_f64], [0.17778_f64, 0.06667_f64, -0.26667_f64, 0.33333_f64]]));
}

#[test]
fn test_matrices_23_multiplying_a_product_by_its_inverse() {
    // Multiplying a product by its inverse
	let mut A = matrix4([[3.0_f64, -9.0_f64, 7.0_f64, 3.0_f64], [3.0_f64, -8.0_f64, 2.0_f64, -9.0_f64], [-4.0_f64, 4.0_f64, 4.0_f64, 1.0_f64], [-6.0_f64, 5.0_f64, -1.0_f64, 1.0_f64]]);
	let mut B = matrix4([[8.0_f64, 2.0_f64, 2.0_f64, 2.0_f64], [3.0_f64, -1.0_f64, 7.0_f64, 0.0_f64], [7.0_f64, 0.0_f64, 5.0_f64, 4.0_f64], [6.0_f64, -2.0_f64, 0.0_f64, 5.0_f64]]);
	let mut C = A * B;
	assert_eq!(C * inverse(&B), A);
}
