
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_materials_00_the_default_material() {
    // The default material
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let mut m = material();
	assert_eq!(m.color, color(1_f64, 1_f64, 1_f64));
	assert!(equal(m.ambient, 0.1_f64));
	assert!(equal(m.diffuse, 0.9_f64));
	assert!(equal(m.specular, 0.9_f64));
	assert!(equal(m.shininess, 200.0_f64));
}

#[test]
fn test_materials_01_reflectivity_for_the_default_material() {
    // Reflectivity for the default material
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let mut m = material();
	assert!(equal(m.reflective, 0.0_f64));
}

#[test]
fn test_materials_02_transparency_and_refractive_index_for_the_default_material() {
    // Transparency and Refractive Index for the default material
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let mut m = material();
	assert!(equal(m.transparency, 0.0_f64));
	assert!(equal(m.refractive_index, 1.0_f64));
}

#[test]
fn test_materials_03_lighting_with_the_eye_between_the_light_and_the_surface() {
    // Lighting with the eye between the light and the surface
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let eyev = vector(0_f64, 0_f64, -1_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 0_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let result = lighting5(&m, &light, &position, &eyev, &normalv);
	assert_eq!(result, color(1.9_f64, 1.9_f64, 1.9_f64));
}

#[test]
fn test_materials_04_lighting_with_the_eye_between_light_and_surface_eye_offset_45_() {
    // Lighting with the eye between light and surface, eye offset 45°
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let eyev = vector(0_f64, 2_f64.sqrt() / 2_f64, -2_f64.sqrt() / 2_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 0_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let result = lighting5(&m, &light, &position, &eyev, &normalv);
	assert_eq!(result, color(1.0_f64, 1.0_f64, 1.0_f64));
}

#[test]
fn test_materials_05_lighting_with_eye_opposite_surface_light_offset_45_() {
    // Lighting with eye opposite surface, light offset 45°
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let eyev = vector(0_f64, 0_f64, -1_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 10_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let result = lighting5(&m, &light, &position, &eyev, &normalv);
	assert_eq!(result, color(0.7364_f64, 0.7364_f64, 0.7364_f64));
}

#[test]
fn test_materials_06_lighting_with_eye_in_the_path_of_the_reflection_vector() {
    // Lighting with eye in the path of the reflection vector
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let eyev = vector(0_f64, -2_f64.sqrt() / 2_f64, -2_f64.sqrt() / 2_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 10_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let result = lighting5(&m, &light, &position, &eyev, &normalv);
	assert_eq!(result, color(1.6364_f64, 1.6364_f64, 1.6364_f64));
}

#[test]
fn test_materials_07_lighting_with_the_light_behind_the_surface() {
    // Lighting with the light behind the surface
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let eyev = vector(0_f64, 0_f64, -1_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 0_f64, 10_f64), &color(1_f64, 1_f64, 1_f64));
	let result = lighting5(&m, &light, &position, &eyev, &normalv);
	assert_eq!(result, color(0.1_f64, 0.1_f64, 0.1_f64));
}

#[test]
fn test_materials_08_lighting_with_the_surface_in_shadow() {
    // Lighting with the surface in shadow
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	let eyev = vector(0_f64, 0_f64, -1_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 0_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let in_shadow = true;
	let result = lighting6(&m, &light, &position, &eyev, &normalv, &in_shadow);
	assert_eq!(result, color(0.1_f64, 0.1_f64, 0.1_f64));
}

#[test]
fn test_materials_09_lighting_with_a_pattern_applied() {
    // Lighting with a pattern applied
	let mut m = material();
	let position = point(0_f64, 0_f64, 0_f64);
	m.set_pattern(&(stripe_pattern(&color(1_f64, 1_f64, 1_f64), &color(0_f64, 0_f64, 0_f64))));
	m.ambient = 1_f64;
	m.diffuse = 0_f64;
	m.specular = 0_f64;
	let eyev = vector(0_f64, 0_f64, -1_f64);
	let normalv = vector(0_f64, 0_f64, -1_f64);
	let light = point_light(&point(0_f64, 0_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let c1 = lighting6(&m, &light, &point(0.9_f64, 0_f64, 0_f64), &eyev, &normalv, &false);
	let c2 = lighting6(&m, &light, &point(1.1_f64, 0_f64, 0_f64), &eyev, &normalv, &false);
	assert_eq!(c1, color(1_f64, 1_f64, 1_f64));
	assert_eq!(c2, color(0_f64, 0_f64, 0_f64));
}
