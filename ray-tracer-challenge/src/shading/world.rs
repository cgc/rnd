use crate::{Light, Intersections, Ray, Shape, Color, C, Tuple, point_light, point, sphere, color, scaling, intersections, BLACK, magnitude, normalize, ray, lighting7, dot, schlick, prepare_computations3};

pub struct World {
    pub count: usize,
    pub lights: Vec<Light>,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn add(&mut self, shape: &Shape) {
        self.objects.push(shape.clone());
    }
    pub fn add_light(&mut self, light: &Light) {
        self.lights.push(*light);
    }

    // for testing
    pub fn light(&self) -> Light {
        *self.lights.first().unwrap()
    }
    pub fn set_light(&mut self, light: &Light) {
        self.lights = vec![*light];
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut v = vec![];
        for o in &self.objects {
            v.append(&mut o.intersect(ray));
        }
        intersections(v)
    }

    pub fn intersect_closest_hit(&self, ray: &Ray) -> Intersections {
        let mut v = vec![];
        for o in &self.objects {
            v.append(&mut o.intersect_closest_hit(ray));
        }
        intersections(v)
    }
}

pub fn world() -> World {
    World {
        count: 0,
        lights: vec![],
        objects: vec![],
    }
}

pub fn default_world() -> World {
	let light = point_light(&point(-10_f64, 10_f64, -10_f64), &color(1_f64, 1_f64, 1_f64));
	let mut s1 = sphere();
	s1.material.color = color(0.8_f64, 1.0_f64, 0.6_f64);
	s1.material.diffuse = 0.7_f64;
	s1.material.specular = 0.2_f64;
	let mut s2 = sphere();
	s2.set_transform(&scaling(0.5_f64, 0.5_f64, 0.5_f64));
    World {
        count: 2,
        lights: vec![light],
        objects: vec![s1, s2],
    }
}

pub fn intersect_world<'a>(world: &'a World, ray: &Ray) -> Intersections<'a> {
    world.intersect(ray)
}

pub const DEFAULT_REMAINING: usize = 5;

pub fn shade_hit2(world: &World, comps: &C) -> Color {
    shade_hit3(world, comps, DEFAULT_REMAINING)
}

pub fn shade_hit3(world: &World, comps: &C, remaining: usize) -> Color {
    let material = &comps.object.material;
    let mut surface = BLACK;
    for light in &world.lights {
        let shadowed = is_shadowed3(world, &comps.over_point, light);
        surface = surface + lighting7(material, Some(&comps.object), light, &comps.over_point, &comps.eyev, &comps.normalv, &shadowed);
    }
    let reflective = reflected_color3(world, comps, remaining);
    let refractive = refracted_color(world, comps, remaining);
    if material.reflective > 0. && material.transparency > 0. {
        let reflectance = schlick(comps);
        surface + reflective * reflectance + refractive * (1. - reflectance)
    } else {
        surface + reflective + refractive
    }
}

pub fn color_at(world: &World, ray: &Ray) -> Color {
    color_at3(world, ray, DEFAULT_REMAINING)
}

pub fn color_at3(world: &World, ray: &Ray, remaining: usize) -> Color {
    // TODO Optimize this by first looking for the closest hit, and
    // only getting all hits if the closest hit requires them.
    // i.e., the closest hit is transparent or a CSG
    let is = intersect_world(world, ray);
    if let Some(i) = is.hit() {
        let comps = prepare_computations3(&i, ray, &is);
        shade_hit3(world, &comps, remaining)
    } else {
        BLACK
    }
}

pub fn is_shadowed(world: &World, point: &Tuple) -> bool {
    // For testing
    is_shadowed3(world, point, &world.light())
}

pub fn is_shadowed3(world: &World, point: &Tuple, light: &Light) -> bool {
    let to_light = light.position - *point;
    let distance = magnitude(&to_light);
    let ray = ray(point, &normalize(&to_light));
    // We look for closest hit here, instead of looking for all hits.
    // TODO Optimize this to look for any hit closer than `distance`
    let xs = world.intersect_closest_hit(&ray);
    if let Some(i) = xs.hit_for_shadow() {
        i.t < distance
    } else {
        false
    }
}

pub fn reflected_color2(world: &World, comps: &C) -> Color {
    reflected_color3(world, comps, DEFAULT_REMAINING)
}

pub fn reflected_color3(world: &World, comps: &C, remaining: usize) -> Color {
    if remaining == 0 || comps.object.material.reflective == 0. {
        return BLACK;
    }
    let r = ray(&comps.over_point, &comps.reflectv);
    let c = color_at3(world, &r, remaining - 1);
    c * comps.object.material.reflective
}

pub fn refracted_color(world: &World, comps: &C, remaining: usize) -> Color {
    if remaining == 0 || comps.object.material.transparency == 0. {
        return BLACK;
    }
    let n_ratio = comps.n1 / comps.n2;
    let cos_i = dot(&comps.eyev, &comps.normalv);
    let sin2_t = n_ratio * n_ratio * (1. - cos_i * cos_i);
    if sin2_t >= 1. {
        return BLACK;
    }
    let cos_t = (1. - sin2_t).sqrt();
    let direction = comps.normalv * (n_ratio * cos_i - cos_t) -
        comps.eyev * n_ratio;
    let refract_ray = ray(&comps.under_point, &direction);
    color_at3(world, &refract_ray, remaining - 1) * comps.object.material.transparency
}
