use crate::{Color, Light, Tuple, dot, normalize, BLACK, reflect, Pattern, pattern_at, pattern_at_shape, Shape};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    // For tests
    pub fn pattern(&self) -> Pattern { self.pattern.unwrap() }
    pub fn set_pattern(&mut self, p: &Pattern) { self.pattern = Some(*p); }
}

pub const DEFAULT_MATERIAL: Material = Material {
    color: Color { red: 1., green: 1., blue: 1. },
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,

    reflective: 0.,
    transparency: 0.,
    refractive_index: 1.,
    pattern: None,
};

pub fn material() -> Material {
    DEFAULT_MATERIAL
}

pub fn lighting5(material: &Material, light: &Light, position: &Tuple, eyev: &Tuple, normalv: &Tuple) -> Color {
    lighting6(material, light, position, eyev, normalv, &false)
}

pub fn lighting6(material: &Material, light: &Light, position: &Tuple, eyev: &Tuple, normalv: &Tuple, in_shadow: &bool) -> Color {
    lighting7(material, None, light, position, eyev, normalv, in_shadow)
}

pub fn lighting7(material: &Material, object: Option<&Shape>, light: &Light, position: &Tuple, eyev: &Tuple, normalv: &Tuple, in_shadow: &bool) -> Color {
    let color = if let Some(p) = material.pattern {
        if let Some(o) = object {
            pattern_at_shape(&p, o, position)
        } else {
            pattern_at(&p, position)
        }
    } else {
        material.color
    };
    let effective_color = color * light.intensity;

    let lightv = normalize(&(light.position - *position));

    let ambient = effective_color * material.ambient;

    let light_dot_normal = dot(&lightv, normalv);
    let diffuse: Color;
    let specular: Color;

    if *in_shadow || light_dot_normal < 0. {
        diffuse = BLACK;
        specular = BLACK;
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = reflect(&(-lightv), normalv);
        let reflect_dot_eye = dot(&reflectv, eyev);
        if reflect_dot_eye <= 0. {
            specular = BLACK;
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}
