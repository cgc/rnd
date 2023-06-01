
#![allow(non_snake_case)]
#![allow(unused_mut)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_camera_00_constructing_a_camera() {
    // Constructing a camera
	let hsize = 160_f64;
	let vsize = 120_f64;
	let field_of_view = π / 2_f64;
	let mut c = camera(160_f64, 120_f64, π / 2_f64);
	assert!(c.hsize == 160);
	assert!(c.vsize == 120);
	assert!(c.field_of_view == π / 2_f64);
	assert!(c.transform() == identity_matrix);
}

#[test]
fn test_camera_01_the_pixel_size_for_a_horizontal_canvas() {
    // The pixel size for a horizontal canvas
	let mut c = camera(200_f64, 125_f64, π / 2_f64);
	assert!(equal(c.pixel_size, 0.01_f64));
}

#[test]
fn test_camera_02_the_pixel_size_for_a_vertical_canvas() {
    // The pixel size for a vertical canvas
	let mut c = camera(125_f64, 200_f64, π / 2_f64);
	assert!(equal(c.pixel_size, 0.01_f64));
}

#[test]
fn test_camera_03_constructing_a_ray_through_the_center_of_the_canvas() {
    // Constructing a ray through the center of the canvas
	let mut c = camera(201_f64, 101_f64, π / 2_f64);
	let r = ray_for_pixel(&mut c, 100, 50);
	assert!(r.origin == point(0_f64, 0_f64, 0_f64));
	assert!(r.direction == vector(0_f64, 0_f64, -1_f64));
}

#[test]
fn test_camera_04_constructing_a_ray_through_a_corner_of_the_canvas() {
    // Constructing a ray through a corner of the canvas
	let mut c = camera(201_f64, 101_f64, π / 2_f64);
	let r = ray_for_pixel(&mut c, 0, 0);
	assert!(r.origin == point(0_f64, 0_f64, 0_f64));
	assert!(r.direction == vector(0.66519_f64, 0.33259_f64, -0.66851_f64));
}

#[test]
fn test_camera_05_constructing_a_ray_when_the_camera_is_transformed() {
    // Constructing a ray when the camera is transformed
	let mut c = camera(201_f64, 101_f64, π / 2_f64);
	c.set_transform(&(rotation_y(π / 4_f64) * translation(0_f64, -2_f64, 5_f64)));
	let r = ray_for_pixel(&mut c, 100, 50);
	assert!(r.origin == point(0_f64, 2_f64, -5_f64));
	assert!(r.direction == vector(2_f64.sqrt() / 2_f64, 0_f64, -2_f64.sqrt() / 2_f64));
}

#[test]
fn test_camera_06_rendering_a_world_with_a_camera() {
    // Rendering a world with a camera
	let mut w = default_world();
	let mut c = camera(11_f64, 11_f64, π / 2_f64);
	let from_ = point(0_f64, 0_f64, -5_f64);
	let to = point(0_f64, 0_f64, 0_f64);
	let up = vector(0_f64, 1_f64, 0_f64);
	c.set_transform(&(view_transform(&from_, &to, &up)));
	let image = render(&mut c, &mut w);
	assert!(pixel_at(&image, 5, 5) == color(0.38066_f64, 0.47583_f64, 0.2855_f64));
}
