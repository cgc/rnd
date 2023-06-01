
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_world_00_creating_a_world() {
    // Creating a world
	let mut w = world();
	assert_eq!(w.count, 0);
	assert_eq!(w.lights.len(), 0);
}

#[test]
fn test_world_01_the_default_world() {
    // The default world
	let light = point_light(&point(-10_f64, 10_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let mut s1 = sphere();
	s1.material.color = color(0.8_f64, 1.0_f64, 0.6_f64);
	s1.material.diffuse = 0.7_f64;
	s1.material.specular = 0.2_f64;
	let mut s2 = sphere();
	s2.set_transform(&(scaling(0.5_f64, 0.5_f64, 0.5_f64)));
	let mut w = default_world();
	assert_eq!(w.light(), light);
	assert!(w.objects.contains(&s1));
	assert!(w.objects.contains(&s2));
}

#[test]
fn test_world_02_intersect_a_world_with_a_ray() {
    // Intersect a world with a ray
	let mut w = default_world();
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersect_world(&w, &r);
	assert_eq!(xs.count, 4);
	assert!(equal(xs[0].t, 4_f64));
	assert!(equal(xs[1].t, 4.5_f64));
	assert!(equal(xs[2].t, 5.5_f64));
	assert!(equal(xs[3].t, 6_f64));
}

#[test]
fn test_world_03_shading_an_intersection() {
    // Shading an intersection
	let mut w = default_world();
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = &w.objects[0];
	let i = intersection(4_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	let mut c = shade_hit2(&w, &comps);
	assert_eq!(c, color(0.38066_f64, 0.47583_f64, 0.2855_f64));
}

#[test]
fn test_world_04_shading_an_intersection_from_the_inside() {
    // Shading an intersection from the inside
	let mut w = default_world();
	w.set_light(&(point_light(&point(0_f64, 0.25_f64, 0_f64), &color(1_f64, 1_f64, 1_f64))));
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = &w.objects[1];
	let i = intersection(0.5_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	let mut c = shade_hit2(&w, &comps);
	assert_eq!(c, color(0.90498_f64, 0.90498_f64, 0.90498_f64));
}

#[test]
fn test_world_05_the_color_when_a_ray_misses() {
    // The color when a ray misses
	let mut w = default_world();
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 1_f64, 0_f64));
	let mut c = color_at(&w, &r);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_world_06_the_color_when_a_ray_hits() {
    // The color when a ray hits
	let mut w = default_world();
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut c = color_at(&w, &r);
	assert_eq!(c, color(0.38066_f64, 0.47583_f64, 0.2855_f64));
}

#[test]
fn test_world_07_the_color_with_an_intersection_behind_the_ray() {
    // The color with an intersection behind the ray
	let mut w = default_world();
	let mut outer = &mut w.objects[0];
	outer.material.ambient = 1_f64;
	let mut inner = &mut w.objects[1];
	inner.material.ambient = 1_f64;
	let r = ray(&point(0_f64, 0_f64, 0.75_f64), &vector(0_f64, 0_f64, -1_f64));
	let exp_c = inner.material.color;
	let mut c = color_at(&w, &r);
	assert_eq!(c, exp_c);
}

#[test]
fn test_world_08_there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    // There is no shadow when nothing is collinear with point and light
	let mut w = default_world();
	let p = point(0_f64, 10_f64, 0_f64);
	assert!(!is_shadowed(&w, &p));
}

#[test]
fn test_world_09_the_shadow_when_an_object_is_between_the_point_and_the_light() {
    // The shadow when an object is between the point and the light
	let mut w = default_world();
	let p = point(10_f64, -10_f64, 10_f64);
	assert!(is_shadowed(&w, &p));
}

#[test]
fn test_world_10_there_is_no_shadow_when_an_object_is_behind_the_light() {
    // There is no shadow when an object is behind the light
	let mut w = default_world();
	let p = point(-20_f64, 20_f64, -20_f64);
	assert!(!is_shadowed(&w, &p));
}

#[test]
fn test_world_11_there_is_no_shadow_when_an_object_is_behind_the_point() {
    // There is no shadow when an object is behind the point
	let mut w = default_world();
	let p = point(-2_f64, 2_f64, -2_f64);
	assert!(!is_shadowed(&w, &p));
}

#[test]
fn test_world_12_shade_hit_is_given_an_intersection_in_shadow() {
    // shade_hit() is given an intersection in shadow
	let mut w = world();
	w.set_light(&(point_light(&point(0_f64, 0_f64, -10_f64), &color(1_f64, 1_f64, 1_f64))));
	let mut s1 = sphere();
	w.add(&s1);
	let mut s2 = sphere();
	s2.set_transform(&(translation(0_f64, 0_f64, 10_f64)));
	w.add(&s2);
	let r = ray(&point(0_f64, 0_f64, 5_f64), &vector(0_f64, 0_f64, 1_f64));
	let i = intersection(4_f64, &s2);
	let comps = prepare_computations2(&i, &r);
	let mut c = shade_hit2(&w, &comps);
	assert_eq!(c, color(0.1_f64, 0.1_f64, 0.1_f64));
}

#[test]
fn test_world_13_the_reflected_color_for_a_nonreflective_material() {
    // The reflected color for a nonreflective material
	let mut w = default_world();
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 0_f64, 1_f64));
	let mut shape = &mut w.objects[1];
	shape.material.ambient = 1_f64;
	let mut shape = &w.objects[1];
	let i = intersection(1_f64, &shape);
	let comps = prepare_computations2(&i, &r);
	let mut c = reflected_color2(&w, &comps);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_world_14_the_reflected_color_for_a_reflective_material() {
    // The reflected color for a reflective material
	let mut w = default_world();
	let mut shape = plane();
	shape.material.reflective = 0.5_f64;
	shape.set_transform(&(translation(0_f64, -1_f64, 0_f64)));
	w.add(&shape);
	let r = ray(&point(0_f64, 0_f64, -3_f64), &vector(0_f64, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	let i = intersection(2_f64.sqrt(), &shape);
	let comps = prepare_computations2(&i, &r);
	let mut c = reflected_color2(&w, &comps);
	assert_eq!(c, color(0.19032_f64, 0.2379_f64, 0.14274_f64));
}

#[test]
fn test_world_15_shade_hit_with_a_reflective_material() {
    // shade_hit() with a reflective material
	let mut w = default_world();
	let mut shape = plane();
	shape.material.reflective = 0.5_f64;
	shape.set_transform(&(translation(0_f64, -1_f64, 0_f64)));
	w.add(&shape);
	let r = ray(&point(0_f64, 0_f64, -3_f64), &vector(0_f64, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	let i = intersection(2_f64.sqrt(), &shape);
	let comps = prepare_computations2(&i, &r);
	let mut c = shade_hit2(&w, &comps);
	assert_eq!(c, color(0.87677_f64, 0.92436_f64, 0.82918_f64));
}

#[test]
fn test_world_16_color_at_with_mutually_reflective_surfaces() {
    // color_at() with mutually reflective surfaces
	let mut w = world();
	w.set_light(&(point_light(&point(0_f64, 0_f64, 0_f64), &color(1_f64, 1_f64, 1_f64))));
	let mut lower = plane();
	lower.material.reflective = 1_f64;
	lower.set_transform(&(translation(0_f64, -1_f64, 0_f64)));
	w.add(&lower);
	let mut upper = plane();
	upper.material.reflective = 1_f64;
	upper.set_transform(&(translation(0_f64, 1_f64, 0_f64)));
	w.add(&upper);
	let r = ray(&point(0_f64, 0_f64, 0_f64), &vector(0_f64, 1_f64, 0_f64));
	assert_ne!(color_at(&w, &r), BLACK);
}

#[test]
fn test_world_17_the_reflected_color_at_the_maximum_recursive_depth() {
    // The reflected color at the maximum recursive depth
	let mut w = default_world();
	let mut shape = plane();
	shape.material.reflective = 0.5_f64;
	shape.set_transform(&(translation(0_f64, -1_f64, 0_f64)));
	w.add(&shape);
	let r = ray(&point(0_f64, 0_f64, -3_f64), &vector(0_f64, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	let i = intersection(2_f64.sqrt(), &shape);
	let comps = prepare_computations2(&i, &r);
	let mut c = reflected_color3(&w, &comps, 0);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_world_18_the_refracted_color_with_an_opaque_surface() {
    // The refracted color with an opaque surface
	let mut w = default_world();
	let mut shape = &w.objects[0];
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections2(intersection(4_f64, &shape), intersection(6_f64, &shape));
	let comps = prepare_computations3(&xs[0], &r, &xs);
	let mut c = refracted_color(&w, &comps, 5);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_world_19_the_refracted_color_at_the_maximum_recursive_depth() {
    // The refracted color at the maximum recursive depth
	let mut w = default_world();
	let mut shape = &mut w.objects[0];
	shape.material.transparency = 1.0_f64;
	shape.material.refractive_index = 1.5_f64;
	let mut shape = &w.objects[0];
	let r = ray(&point(0_f64, 0_f64, -5_f64), &vector(0_f64, 0_f64, 1_f64));
	let xs = intersections2(intersection(4_f64, &shape), intersection(6_f64, &shape));
	let comps = prepare_computations3(&xs[0], &r, &xs);
	let mut c = refracted_color(&w, &comps, 0);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_world_20_the_refracted_color_under_total_internal_reflection() {
    // The refracted color under total internal reflection
	let mut w = default_world();
	let mut shape = &mut w.objects[0];
	shape.material.transparency = 1.0_f64;
	shape.material.refractive_index = 1.5_f64;
	let mut shape = &w.objects[0];
	let r = ray(&point(0_f64, 0_f64, 2_f64.sqrt() / 2_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = intersections2(intersection(-2_f64.sqrt() / 2_f64, &shape), intersection(2_f64.sqrt() / 2_f64, &shape));
	let comps = prepare_computations3(&xs[1], &r, &xs);
	let mut c = refracted_color(&w, &comps, 5);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_world_21_the_refracted_color_with_a_refracted_ray() {
    // The refracted color with a refracted ray
	let mut w = default_world();
	let mut A = &mut w.objects[0];
	A.material.ambient = 1.0_f64;
	A.material.set_pattern(&(test_pattern()));
	let mut B = &mut w.objects[1];
	B.material.transparency = 1.0_f64;
	B.material.refractive_index = 1.5_f64;
	let mut A = &w.objects[0];
	let mut B = &w.objects[1];
	let r = ray(&point(0_f64, 0_f64, 0.1_f64), &vector(0_f64, 1_f64, 0_f64));
	let xs = intersections4(intersection(-0.9899_f64, &A), intersection(-0.4899_f64, &B), intersection(0.4899_f64, &B), intersection(0.9899_f64, &A));
	let comps = prepare_computations3(&xs[2], &r, &xs);
	let mut c = refracted_color(&w, &comps, 5);
	assert_eq!(c, color(0_f64, 0.99888_f64, 0.04725_f64));
}

#[test]
fn test_world_22_shade_hit_with_a_transparent_material() {
    // shade_hit() with a transparent material
	let mut w = default_world();
	let mut floor = plane();
	floor.set_transform(&(translation(0_f64, -1_f64, 0_f64)));
	floor.material.transparency = 0.5_f64;
	floor.material.refractive_index = 1.5_f64;
	w.add(&floor);
	let mut ball = sphere();
	ball.material.color = color(1_f64, 0_f64, 0_f64);
	ball.material.ambient = 0.5_f64;
	ball.set_transform(&(translation(0_f64, -3.5_f64, -0.5_f64)));
	w.add(&ball);
	let r = ray(&point(0_f64, 0_f64, -3_f64), &vector(0_f64, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	let xs = intersections1(intersection(2_f64.sqrt(), &floor));
	let comps = prepare_computations3(&xs[0], &r, &xs);
	let mut c = shade_hit3(&w, &comps, 5);
	assert_eq!(c, color(0.93642_f64, 0.68642_f64, 0.68642_f64));
}

#[test]
fn test_world_23_shade_hit_with_a_reflective_transparent_material() {
    // shade_hit() with a reflective, transparent material
	let mut w = default_world();
	let r = ray(&point(0_f64, 0_f64, -3_f64), &vector(0_f64, -2_f64.sqrt() / 2_f64, 2_f64.sqrt() / 2_f64));
	let mut floor = plane();
	floor.set_transform(&(translation(0_f64, -1_f64, 0_f64)));
	floor.material.reflective = 0.5_f64;
	floor.material.transparency = 0.5_f64;
	floor.material.refractive_index = 1.5_f64;
	w.add(&floor);
	let mut ball = sphere();
	ball.material.color = color(1_f64, 0_f64, 0_f64);
	ball.material.ambient = 0.5_f64;
	ball.set_transform(&(translation(0_f64, -3.5_f64, -0.5_f64)));
	w.add(&ball);
	let xs = intersections1(intersection(2_f64.sqrt(), &floor));
	let comps = prepare_computations3(&xs[0], &r, &xs);
	let mut c = shade_hit3(&w, &comps, 5);
	assert_eq!(c, color(0.93391_f64, 0.69643_f64, 0.69243_f64));
}
