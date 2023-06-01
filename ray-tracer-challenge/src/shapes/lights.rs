

use crate::{Tuple, Color};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

pub fn point_light(position: &Tuple, intensity: &Color) -> Light {
    Light { position: *position, intensity: *intensity }
}
