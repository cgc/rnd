use crate::matrices::*;
use crate::tuples::*;

pub const PI: f64 = std::f64::consts::PI;

impl Matrix {
    pub fn translate(&self, x: f64, y: f64, z: f64) -> Matrix { translation(x, y, z) * *self }
    pub fn scale(&self, x: f64, y: f64, z: f64) -> Matrix { scaling(x, y, z) * *self }
    pub fn rotate_x(&self, rad: f64) -> Matrix { rotation_x(rad) * *self }
    pub fn rotate_y(&self, rad: f64) -> Matrix { rotation_y(rad) * *self }
    pub fn rotate_z(&self, rad: f64) -> Matrix { rotation_z(rad) * *self }
    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix { shearing(xy, xz, yx, yz, zx, zy) * *self }
}

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = matrix4(EMPTY_ZEROS);
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;
    identity_matrix + m
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = matrix4(EMPTY_ZEROS);
    m[(0, 0)] = x;
    m[(1, 1)] = y;
    m[(2, 2)] = z;
    m[(3, 3)] = 1.;
    m
}

pub fn rotation_x(rad: f64) -> Matrix {
    let mut m = matrix4(EMPTY_ZEROS);
    m[(0, 0)] = 1.;
    m[(1, 1)] = f64::cos(rad);
    m[(1, 2)] = -f64::sin(rad);
    m[(2, 1)] = f64::sin(rad);
    m[(2, 2)] = f64::cos(rad);
    m[(3, 3)] = 1.;
    m
}

pub fn rotation_y(rad: f64) -> Matrix {
    let mut m = matrix4(EMPTY_ZEROS);
    m[(1, 1)] = 1.;
    m[(2, 2)] = f64::cos(rad);
    m[(2, 0)] = -f64::sin(rad);
    m[(0, 2)] = f64::sin(rad);
    m[(0, 0)] = f64::cos(rad);
    m[(3, 3)] = 1.;
    m
}

pub fn rotation_z(rad: f64) -> Matrix {
    let mut m = matrix4(EMPTY_ZEROS);
    m[(0, 0)] = f64::cos(rad);
    m[(0, 1)] = -f64::sin(rad);
    m[(1, 0)] = f64::sin(rad);
    m[(1, 1)] = f64::cos(rad);
    m[(2, 2)] = 1.;
    m[(3, 3)] = 1.;
    m
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut m = identity_matrix;
    m[(0, 1)] = xy;
    m[(0, 2)] = xz;
    m[(1, 0)] = yx;
    m[(1, 2)] = yz;
    m[(2, 0)] = zx;
    m[(2, 1)] = zy;
    m
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = normalize(&(*to - *from));
    let upn = &normalize(up);
    let left = cross(&forward, &upn);
    let true_up = cross(&left, &forward);
    matrix4([
        [left.x, left.y, left.z, 0.],
        [true_up.x, true_up.y, true_up.z, 0.],
        [-forward.x, -forward.y, -forward.z, 0.],
        [0., 0., 0., 1.],
    ]) * translation(-from.x, -from.y, -from.z)
}
