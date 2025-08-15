use lalgebra_scalar::Scalar;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = Matrix::zero(n, n);
        for i in 0..n {
            mat.0[i][i] = T::one();
        }
        mat
    }
}

impl<T: Scalar<Item = T>> Add for Matrix<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut result = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result.0[i][j] = self.0[i][j] + other.0[i][j];
            }
        }
        Some(result)
    }
}

impl<T: Scalar<Item = T>> Sub for Matrix<T> {
    type Output = Option<Self>;
    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut result = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        Some(result)
    }
}
