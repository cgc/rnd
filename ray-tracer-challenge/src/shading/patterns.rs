use std::{ops::Rem};

use crate::{Color, Tuple, Shape, Matrix, identity_matrix, WHITE, inverse, color, BLACK, world_to_object};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PatternType {
    None,
    TestPattern,
    Stripe,
    Checkers,
    Gradient,
    Ring,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub pattern_type: PatternType,
    transform: Matrix,
    inverse: Matrix,
}


impl Pattern {
    pub fn new(pattern_type: PatternType, a: &Color, b: &Color) -> Pattern {
        Pattern { transform: identity_matrix, inverse: identity_matrix, a: *a, b: *b, pattern_type }
    }
    pub fn transform(&self) -> Matrix { self.transform }
    pub fn inverse(&self) -> Matrix { self.inverse }
    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = *m;
        self.inverse = inverse(m);
    }
}

pub fn set_pattern_transform(pattern: &mut Pattern, transform: &Matrix) {
    pattern.set_transform(transform);
}

pub fn floor_i64(x: f64) -> i64 {
    x.floor() as i64
}

pub fn pattern_at(pattern: &Pattern, point: &Tuple) -> Color {
    let a = pattern.a;
    let b = pattern.b;
    match pattern.pattern_type {
        PatternType::TestPattern => color(point.x, point.y, point.z),
        PatternType::Stripe => if floor_i64(point.x).rem(2) == 0 { a } else { b }
        PatternType::Gradient => a + (b - a) * (point.x - point.x.floor()),
        PatternType::Ring => if (((point.x * point.x + point.z * point.z).sqrt()).floor() as i64).rem(2) == 0 { a } else { b },
        PatternType::Checkers => if (floor_i64(point.x) + floor_i64(point.y) + floor_i64(point.z)).rem(2) == 0 { a } else { b }
        PatternType::None => WHITE,
    }
}

pub fn pattern_at_shape(pattern: &Pattern, object: &Shape, point: &Tuple) -> Color {
    let object_point = world_to_object(object, point);
    let pattern_point = pattern.inverse() * object_point;
    pattern_at(pattern, &pattern_point)
}

pub fn stripe_pattern(color1: &Color, color2: &Color) -> Pattern {
    Pattern::new(PatternType::Stripe, color1, color2)
}

pub fn stripe_at(pattern: &Pattern, point: &Tuple) -> Color {
    pattern_at(pattern, point)
}

pub fn stripe_at_object(pattern: &Pattern, object: &Shape, point: &Tuple) -> Color {
    pattern_at_shape(pattern, object, point)
}

pub fn test_pattern() -> Pattern {
    Pattern::new(PatternType::TestPattern, &BLACK, &BLACK)
}

pub fn checkers_pattern(color1: &Color, color2: &Color) -> Pattern {
    Pattern::new(PatternType::Checkers, color1, color2)
}

pub fn gradient_pattern(color1: &Color, color2: &Color) -> Pattern {
    Pattern::new(PatternType::Gradient, color1, color2)
}

pub fn ring_pattern(color1: &Color, color2: &Color) -> Pattern {
    Pattern::new(PatternType::Ring, color1, color2)
}
