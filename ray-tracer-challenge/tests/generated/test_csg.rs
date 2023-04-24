
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_csg_00_csg_is_created_with_an_operation_and_two_shapes() {
    // CSG is created with an operation and two shapes
	let mut s1 = sphere();
	let mut s2 = cube();
	let mut c = csg("union", &s1, &s2);
	assert_eq!(c.operation(), "union");
	assert_eq!(c.left(), s1);
	assert_eq!(c.right(), s2);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex00() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &true, &true, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex01() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &true, &true, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex02() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &true, &false, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex03() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &true, &false, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex04() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &false, &true, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex05() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &false, &true, &false);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex06() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &false, &false, &true);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex07() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("union", &false, &false, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex08() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &true, &true, &true);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex09() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &true, &true, &false);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex10() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &true, &false, &true);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex11() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &true, &false, &false);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex12() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &false, &true, &true);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex13() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &false, &true, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex14() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &false, &false, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex15() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("intersection", &false, &false, &false);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex16() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &true, &true, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex17() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &true, &true, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex18() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &true, &false, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex19() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &true, &false, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex20() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &false, &true, &true);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex21() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &false, &true, &false);
	assert_eq!(result, true);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex22() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &false, &false, &true);
	assert_eq!(result, false);
}

#[test]
fn test_csg_01_evaluating_the_rule_for_a_csg_operation_ex23() {
    // Evaluating the rule for a CSG operation
	let result = intersection_allowed("difference", &false, &false, &false);
	assert_eq!(result, false);
}

#[test]
fn test_csg_02_filtering_a_list_of_intersections_ex00() {
    // Filtering a list of intersections
	let mut s1 = sphere();
	let mut s2 = cube();
	let mut c = csg("union", &s1, &s2);
	let xs = intersections4(intersection(1_f64, &s1), intersection(2_f64, &s2), intersection(3_f64, &s1), intersection(4_f64, &s2));
	let result = filter_intersections(&c, &xs);
	assert_eq!(result.count, 2);
	assert_eq!(result[0], xs[0]);
	assert_eq!(result[1], xs[3]);
}

#[test]
fn test_csg_02_filtering_a_list_of_intersections_ex01() {
    // Filtering a list of intersections
	let mut s1 = sphere();
	let mut s2 = cube();
	let mut c = csg("intersection", &s1, &s2);
	let xs = intersections4(intersection(1_f64, &s1), intersection(2_f64, &s2), intersection(3_f64, &s1), intersection(4_f64, &s2));
	let result = filter_intersections(&c, &xs);
	assert_eq!(result.count, 2);
	assert_eq!(result[0], xs[1]);
	assert_eq!(result[1], xs[2]);
}

#[test]
fn test_csg_02_filtering_a_list_of_intersections_ex02() {
    // Filtering a list of intersections
	let mut s1 = sphere();
	let mut s2 = cube();
	let mut c = csg("difference", &s1, &s2);
	let xs = intersections4(intersection(1_f64, &s1), intersection(2_f64, &s2), intersection(3_f64, &s1), intersection(4_f64, &s2));
	let result = filter_intersections(&c, &xs);
	assert_eq!(result.count, 2);
	assert_eq!(result[0], xs[0]);
	assert_eq!(result[1], xs[1]);
}

#[test]
fn test_csg_03_a_ray_misses_a_csg_object() {
    // A ray misses a CSG object
	let mut c = csg("union", &sphere(), &cube());
	let r = ray(&point(0_f64, 2_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&c, &r);
	assert!(xs.is_empty());
}

#[test]
fn test_csg_04_a_ray_hits_a_csg_object() {
    // A ray hits a CSG object
	let mut s1 = sphere();
	let mut s2 = sphere();
	set_transform(&mut s2, &translation(0_f64, 0_f64, 0.5_f64));
	let mut c = csg("union", &s1, &s2);
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = local_intersect(&c, &r);
	assert_eq!(xs.count, 2);
	assert!(equal(xs[0].t, 4_f64));
	assert_eq!(xs[0].object, &s1);
	assert!(equal(xs[1].t, 6.5_f64));
	assert_eq!(xs[1].object, &s2);
}
