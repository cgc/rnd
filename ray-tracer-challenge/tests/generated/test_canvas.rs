
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use ray_tracer_challenge::*;
use std::fs::read;


#[test]
fn test_canvas_00_creating_a_canvas() {
    // Creating a canvas
	let mut c = canvas(10, 20);
	assert_eq!(c.width, 10);
	assert_eq!(c.height, 20);
	assert_eq!(c, color(0_f64, 0_f64, 0_f64));
}

#[test]
fn test_canvas_01_writing_pixels_to_a_canvas() {
    // Writing pixels to a canvas
	let mut c = canvas(10, 20);
	let red = color(1_f64, 0_f64, 0_f64);
	write_pixel(&mut c, 2, 3, &red);
	assert_eq!(pixel_at(&c, 2, 3), red);
}

#[test]
fn test_canvas_02_constructing_the_ppm_header() {
    // Constructing the PPM header
	let mut c = canvas(5, 3);
	let ppm = canvas_to_ppm(&c);
	assert_eq!(lines(&ppm, 0, 3), "P3\n5 3\n255");
}

#[test]
fn test_canvas_03_constructing_the_ppm_pixel_data() {
    // Constructing the PPM pixel data
	let mut c = canvas(5, 3);
	let c1 = color(1.5_f64, 0_f64, 0_f64);
	let c2 = color(0_f64, 0.5_f64, 0_f64);
	let c3 = color(-0.5_f64, 0_f64, 1_f64);
	write_pixel(&mut c, 0, 0, &c1);
	write_pixel(&mut c, 2, 1, &c2);
	write_pixel(&mut c, 4, 2, &c3);
	let ppm = canvas_to_ppm(&c);
	assert_eq!(lines(&ppm, 3, 6), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}

#[test]
fn test_canvas_04_splitting_long_lines_in_ppm_files() {
    // Splitting long lines in PPM files
	let mut c = canvas(10, 2);
	c.fill(&color(1_f64, 0.8_f64, 0.6_f64));
	let ppm = canvas_to_ppm(&c);
	assert_eq!(lines(&ppm, 3, 7), "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153");
}

#[test]
fn test_canvas_05_ppm_files_are_terminated_by_a_newline_character() {
    // PPM files are terminated by a newline character
	let mut c = canvas(5, 3);
	let ppm = canvas_to_ppm(&c);
	assert_eq!(ppm.chars().last().unwrap(), "\n".chars().last().unwrap());
}
