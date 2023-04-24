use std::ops;
use std::cmp;
use crate::tools::equal;
use crate::tuples::{Tuple, tuple};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    data: [[f64; 4]; 4],
    shape: (usize, usize),
}

impl Matrix {
    fn is_square(&self) -> bool {
        self.shape.0 == self.shape.1
    }
    fn clear(&mut self) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                self[(i, j)] = 0.;
            }
        }
    }
    pub fn is_invertible(&self) -> bool {
        determinant(self) != 0.
    }
}

#[allow(non_upper_case_globals)]
pub const identity_matrix: Matrix = Matrix {
    data: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ],
    shape: (4, 4),
};

pub const EMPTY_NAN: [[f64; 4]; 4] = [[f64::NAN; 4]; 4];
pub const EMPTY_ZEROS: [[f64; 4]; 4] = [[0.; 4]; 4];

pub fn matrix4(data: [[f64; 4]; 4]) -> Matrix {
    Matrix { data: data, shape: (4, 4) }
}

pub fn matrix3(data: [[f64; 3]; 3]) -> Matrix {
    let shape = (3, 3);
    let mut d: [[f64; 4]; 4] = EMPTY_NAN;
    for i in 0..shape.0 {
        d[i][..shape.1].copy_from_slice(&data[i]);
    }
    Matrix { data: d, shape }
}

pub fn matrix2(data: [[f64; 2]; 2]) -> Matrix {
    let shape = (2, 2);
    let mut d: [[f64; 4]; 4] = EMPTY_NAN;
    for i in 0..shape.0 {
        d[i][..shape.1].copy_from_slice(&data[i]);
    }
    Matrix { data: d, shape }
}

pub fn transpose(m: &Matrix) -> Matrix {
    let mut rv = Matrix { data: EMPTY_NAN, shape: m.shape };
    for i in 0..m.shape.0 {
        for j in 0..m.shape.1 {
            rv[(i, j)] = m[(j, i)];
        }
    }
    rv
}

pub fn determinant(m: &Matrix) -> f64 {
    assert!(m.is_square());
    if m.shape == (2, 2) {
        m[(0, 0)] * m[(1, 1)] - m[(0, 1)] * m[(1, 0)]
    } else {
        let mut det = 0.;
        for col in 0..m.shape.1 {
            det += m[(0, col)] * cofactor(m, 0, col);
        }
        det
    }
}

pub fn submatrix(m: &Matrix, row: usize, col: usize) -> Matrix {
    let shape = (m.shape.0 - 1, m.shape.1 - 1);
    let mut d: [[f64; 4]; 4] = EMPTY_NAN;
    for src in 0..m.shape.0 {
        if src == row {
            continue;
        }
        let dest = if src < row { src } else { src - 1 };
        d[dest][..col].copy_from_slice(&m.data[src][..col]);
        d[dest][col..shape.1].copy_from_slice(&m.data[src][col+1..m.shape.1]);
    }
    Matrix { data: d, shape }
}

pub fn minor(m: &Matrix, row: usize, col: usize) -> f64 {
    determinant(&submatrix(m, row, col))
}

pub fn cofactor(m: &Matrix, row: usize, col: usize) -> f64 {
    let sign = if (row + col) % 2 == 0 { 1. } else { -1. };
    sign * minor(m, row, col)
}

pub fn inverse(m: &Matrix) -> Matrix {
    assert!(m.is_invertible());
    let det = determinant(m);
    let mut rv = Matrix { data: EMPTY_NAN, shape: m.shape };
    for i in 0..m.shape.0 {
        for j in 0..m.shape.1 {
            let c = cofactor(m, i, j);
            rv[(j, i)] = c / det;
        }
    }
    rv
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        assert!(self.shape == rhs.shape);
        let mut rv = Matrix { data: EMPTY_NAN, shape: self.shape };
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                rv[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        rv
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut rv = tuple(0., 0., 0., 0.);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                rv[i] += self[(i, j)] * rhs[j];
            }
        }
        rv

        // assert!(self.shape == (4, 4));
        // let mut rv = tuple(0., 0., 0., 0.);
        // for i in 0..self.shape.0 {
        //     rv[i] = self[(i, 0)] * rhs[0] + self[(i, 1)] * rhs[1] + self[(i, 2)] * rhs[2] + self[(i, 3)] * rhs[3];
        // }
        // rv
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert!(self.shape.1 == rhs.shape.0);
        let mut rv = Matrix { data: EMPTY_NAN, shape: (self.shape.0, rhs.shape.1) };
        rv.clear();
        for i in 0..self.shape.0 {
            for k in 0..rhs.shape.1 {
                for j in 0..self.shape.1 {
                    rv[(i, k)] += self[(i, j)] * rhs[(j, k)];
                }
            }
        }

        // assert!(self.shape == (4, 4));
        // for i in 0..self.shape.0 {
        //     for k in 0..rhs.shape.1 {
        //         rv[(i, k)] = self[(i, 0)] * rhs[(0, k)] + self[(i, 1)] * rhs[(1, k)] + self[(i, 2)] * rhs[(2, k)] + self[(i, 3)] * rhs[(3, k)];
        //     }
        // }

        rv
    }
}

impl cmp::PartialEq<Matrix> for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.shape != other.shape {
            return false
        }
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                if !equal(self[(i, j)], other[(i, j)]) {
                    return false
                }
            }
        }
        true
    }
}
