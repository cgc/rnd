use std::ops;
use std::cmp;
use crate::tools::equal;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool { equal(self.w, 1.) }
    pub fn is_vector(&self) -> bool { equal(self.w, 0.) }

    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn dot(&self, b: &Tuple) -> f64 {
        self.x * b.x +
            self.y * b.y +
            self.z * b.z +
            self.w * b.w
    }

    pub fn cross(&self, b: &Tuple) -> Tuple {
        vector(self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x)
    }

    pub fn normalized(&self) -> Tuple {
        *self / self.magnitude()
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        self.clone() - *normal * 2. * self.dot(normal)
    }
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 1.)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 0.)
}

pub fn magnitude(t: &Tuple) -> f64 {
    t.magnitude()
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.dot(b)
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    a.cross(b)
}

pub fn reflect(in_: &Tuple, normal: &Tuple) -> Tuple {
    in_.reflect(normal)
}

pub fn normalize(t: &Tuple) -> Tuple {
    t.normalized()
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("{index}"),
        }
    }
}

impl ops::IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("{index}"),
        }
    }
}

impl cmp::PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        equal(self.x, other.x) &&
            equal(self.y, other.y) &&
            equal(self.z, other.z) &&
            equal(self.w, other.w)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const BLACK: Color = Color { red: 0., green: 0., blue: 0. };
pub const WHITE: Color = Color { red: 1., green: 1., blue: 1. };

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color { red, green, blue }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        color(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        color(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        color(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        color(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl cmp::PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        equal(self.red, other.red) &&
            equal(self.green, other.green) &&
            equal(self.blue, other.blue)
    }
}
