use std::f64::consts::PI;
use std::fs;
use std::time::Instant;

use ray_tracer_challenge::*;

fn ch4() {
    let mut c = canvas(100, 100);
    for i in 0..12 {
        let rad = i as f64 * 2. * PI / 12.;
        let pt = translation(50., 50., 0.) * rotation_z(rad) * scaling(40., 40., 30.) * point(1., 0., 0.);
        write_pixel(&mut c, pt.x as i64, pt.y as i64, &color(1., 1., 1.))
    }
    fs::write("output/ch4.ppm", canvas_to_ppm(&c)).unwrap();
}

fn ch5() {
    let mut c = canvas(100, 100);
    let s = 0.5;
    let mut sphere = sphere();
    sphere.set_transform(&(translation(0.1, 0.1, 1.) * scaling(s, s, s)));
    let o = point(0., 0., 0.);
    for x in 0..c.width {
        for y in 0..c.height {
            let d = translation(-1., -1., 0.) * scaling(2./(c.width as f64), 2./(c.height as f64), 1.) * point(x as f64, y as f64, 1.);
            let r = ray(&o, &normalize(&(d - o)));
            let ii = intersect(&sphere, &r);
            if ii.count > 0 {
                write_pixel(&mut c, x as i64, y as i64, &color(1., 0., 0.));
            }
        }
    }
    fs::write("output/ch5.ppm", canvas_to_ppm(&c)).unwrap();
}

fn ch7() {
    let mut camera = camera(300., 300., PI/2.);
    camera.set_transform(&view_transform(
        &point(0., 1.5, -5.),
        &point(0., 1., 0.),
        &point(0., 1., 0.),
    ));
    let w = default_world();
    let c = render(&camera, &w);
    fs::write("output/ch7.ppm", canvas_to_ppm(&c)).unwrap();
}

fn ch9() {
    let mut camera = camera(300., 300., PI/2.);
    camera.set_transform(&view_transform(
        &point(0., 1.5, -5.),
        &point(0., 1., 0.),
        &point(0., 1., 0.),
    ));
    let mut w = default_world();
    let mut p = plane();
    set_transform(&mut p, &translation(0., 0., -10.));
    w.add(&p);
    let c = render(&camera, &w);
    fs::write("output/ch9.ppm", canvas_to_ppm(&c)).unwrap();
}

fn ch10() {
    let mut camera = camera(300., 300., PI/2.);
    camera.set_transform(&view_transform(
        &point(0., 0., 0.),
        &point(0., 0., 1.),
        &point(0., 1., 0.),
    ));
    let mut w = world();
	w.add_light(&point_light(&point(-10_f64, 10_f64, -10_f64), &color(1_f64, 1_f64, 1_f64)));

    let mut p = plane();
    p.set_transform(&translation(0., -1., 0.));
    p.material.pattern = Some(stripe_pattern(&color(1., 0., 0.), &color(0., 0., 1.)));
    w.add(&p);

    let mut p = plane();
    p.set_transform(&(translation(0., 0., 5.) * rotation_x(PI/2.)));
    let mut pattern = gradient_pattern(&color(1., 0., 0.), &color(0., 0., 1.));
    pattern.set_transform(&scaling(10., 1., 1.));
    p.material.pattern = Some(pattern);
    w.add(&p);

    let mut s = sphere();
    s.material.color = color(1., 0., 0.);
    s.set_transform(&translation(0., 0., 3.));
    w.add(&s);

    let c = render(&camera, &w);
    fs::write("output/ch10.ppm", canvas_to_ppm(&c)).unwrap();
}

fn ch11() {
    let mut camera = camera(300., 300., PI/2.);
    camera.set_transform(&view_transform(
        &point(0., 0., 0.),
        &point(0., 0., 1.),
        &point(0., 1., 0.),
    ));
    let mut w = world();
	w.add_light(&point_light(&point(0., 10., -1.), &color(1_f64, 1_f64, 1_f64)));
    let mut pattern = checkers_pattern(&WHITE , &BLACK);
    // To avoid acne
    pattern.set_transform(&translation(0., 2.*EPSILON, 0.));

    let mut p = plane();
    p.set_transform(&translation(0., -1., 0.));
    p.material.pattern = Some(pattern);
    p.material.reflective = 0.2;
    w.add(&p);

    let mut p = plane();
    p.set_transform(&(translation(0., 0., 5.5) * rotation_x(PI/2.)));
    p.material.pattern = Some(pattern);
    w.add(&p);

    let t = translation(0., 0., 2.);
    let mut s = sphere();
    // let mut s = cylinder();
    // s.set_maximum(&3.);
    // s.set_minimum(&0.);
    // Copying pg159
    s.material.color = WHITE;
    s.material.ambient = 0.;
    s.material.diffuse = 0.;
    s.material.specular = 0.9;
    s.material.shininess = 300.;
    s.material.reflective = 0.9;
    s.material.transparency = 0.9;
    s.material.refractive_index = 1.05;
    s.set_transform(&t);
    w.add(&s);

    let mut s = sphere();
    s.material.color = color(1., 0., 0.);
    s.set_transform(&translation(0., 0., 5.));
    w.add(&s);

    let c = render(&camera, &w);
    fs::write("output/ch11.ppm", canvas_to_ppm(&c)).unwrap();
}

fn teapot_low() -> World {
    // Downloaded from https://graphics.cs.utah.edu/courses/cs6620/fall2019/prj05/teapot-low.obj
    let bytes = fs::read("objs/teapot-low.obj").unwrap();
    let mut p = parse_obj_file(&bytes);

    let mut s = obj_to_group(&mut p);
    let scale = 0.3;
    let t = rotation_x(-1.2*PI/2.).scale(scale, scale, scale).translate(0., -2., 7.5);
    s.set_transform(&t);
    s.freeze_and_optimize();

    let mut w = world();
	w.add_light(&point_light(&point(0., 7., 0.), &color(1_f64, 1_f64, 1_f64)));
    w.add(&s);

    let mut p = plane();
    p.set_transform(&translation(0., -2., 0.));
    p.material.pattern = Some(checkers_pattern(&color(0.7, 0.7, 0.7), &color(0.8, 0.8, 0.8)));
    w.add(&p);

    let mut p = plane();
    p.set_transform(&rotation_x(PI/2.).translate(10., -10., 10.));
    p.material.pattern = Some(checkers_pattern(&color(0.7, 0.7, 0.7), &color(0.8, 0.8, 0.8)));
    w.add(&p);

    w
}

fn teapot_high() -> World {
    // Downloaded from https://users.cs.utah.edu/~natevm/newell_teaset/newell_teaset.zip
    let bytes = fs::read("objs/teapot.obj").unwrap();
    let mut p = parse_obj_file(&bytes);
    let mut s = obj_to_group(&mut p);
    let scale = 0.9;
    let t = rotation_y(-PI/2.).rotate_x(-0.0 * PI / 2.).scale(scale, scale, scale).translate(0., -2., 4.5);
    s.set_transform(&t);
    s.freeze_and_optimize();
    let mut w = world();
	w.add_light(&point_light(&point(5., 7., -5.), &color(1_f64, 1_f64, 1_f64)));
    w.add(&s);

    let mut p = plane();
    p.set_transform(&translation(0., -2., 0.));
    p.material.pattern = Some(checkers_pattern(&color(0.7, 0.7, 0.7), &color(0.8, 0.8, 0.8)));
    w.add(&p);

    let mut p = plane();
    p.set_transform(&rotation_x(PI/2.).translate(10., -10., 10.));
    p.material.pattern = Some(checkers_pattern(&color(0.7, 0.7, 0.7), &color(0.8, 0.8, 0.8)));
    w.add(&p);

    w
}

fn render_world(w: &World, output: &str) {
    let mut camera = camera(600., 400., PI/2.);
    camera.set_transform(&view_transform(
        &point(0., 0., 0.),
        &point(0., 0., 1.),
        &point(0., 1., 0.),
    ));
    let c = render(&camera, &w);
    fs::write(output, canvas_to_ppm(&c)).unwrap();
}

fn render_scene(path: &str, output: &str) {
    let c = load(path).unwrap();
    fs::write(output, canvas_to_ppm(&c)).unwrap();
}

fn main() {
    let now = Instant::now();
    ch4();
    ch5();
    ch7();
    ch9();
    ch10();
    ch11();

    render_scene("book-code/cover.yml", "output/cover.ppm");

    for name in [
        "pg159",
        "table",
        "cylinders",
        "puppets",
        "reflect-refract",
        "groups",
    ] {
        render_scene(&format!("book-code/forum-scenes/{name}.yml"), &format!("output/{name}.ppm"));
    }

    render_world(&teapot_low(), "output/ch15.ppm");
    render_world(&teapot_high(), "output/ch15-high.ppm");

    let elapsed_time = now.elapsed();
    println!("Rendering done. {} seconds.", (elapsed_time.as_millis() as f64)/1000.);
}
