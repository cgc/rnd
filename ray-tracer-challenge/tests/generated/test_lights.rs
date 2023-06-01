
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_lights_00_a_point_light_has_a_position_and_intensity() {
    // A point light has a position and intensity
	let intensity = color(1_f64, 1_f64, 1_f64);
	let position = point(0_f64, 0_f64, 0_f64);
	let light = point_light(&position, &intensity);
	assert_eq!(light.position, position);
	assert_eq!(light.intensity, intensity);
}
