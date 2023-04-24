use crate::{Matrix, identity_matrix, Ray, canvas, World, Canvas, normalize, point, inverse, ray, color_at, write_pixel};

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub half_height: f64,
    pub half_width: f64,
    pub pixel_size: f64,
    transform: Matrix,
    inverse: Matrix,
}

impl Camera {
    pub fn transform(&self) -> Matrix { self.transform }
    pub fn inverse(&self) -> Matrix { self.inverse }
    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = *m;
        self.inverse = inverse(m);
    }
}

// Note: hsize/vsize should be usizes, but this seems a bit easier to manage for now
pub fn camera(hsize: f64, vsize: f64, field_of_view: f64) -> Camera {
    let aspect = hsize / vsize;
    let half_view = (field_of_view / 2.).tan();
    let half_width;
    let half_height;
    if aspect > 1. {
        half_width = half_view;
        half_height = half_view / aspect;
    } else {
        half_width = half_view * aspect;
        half_height = half_view;
    }
    let pixel_size = (half_width * 2.) / hsize;
    Camera {
        hsize: hsize as usize,
        vsize: vsize as usize,
        field_of_view,
        pixel_size,
        half_width,
        half_height,
        transform: identity_matrix,
        inverse: identity_matrix,
    }
}

pub fn ray_for_pixel(camera: &Camera, x: usize, y: usize) -> Ray {
    let xoffset = (x as f64 + 0.5) * camera.pixel_size;
    let yoffset = (y as f64 + 0.5) * camera.pixel_size;
    let canvas_point = camera.inverse() * point(
        camera.half_width - xoffset,
        camera.half_height - yoffset,
        -1.
    );
    let origin = camera.inverse() * point(0., 0., 0.);
    let direction = normalize(&(canvas_point - origin));
    ray(&origin, &direction)
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut c = canvas(camera.hsize, camera.vsize);
    for x in 0..c.width {
        for y in 0..c.height {
            let ray = ray_for_pixel(camera, x, y);
            let color = color_at(world, &ray);
            write_pixel(&mut c, x as i64, y as i64, &color);
        }
    }
    c
}
