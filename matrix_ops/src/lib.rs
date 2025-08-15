use lalgebra_scalar::Scalar;
use matrix::Matrix;
use std::ops::{Add, Sub};

pub struct MatrixOps<T>(pub Matrix<T>);

impl<T: Scalar<Item = T>> Add for MatrixOps<T> {
    type Output = Option<Self>;
    fn add(&self, other: &Self) -> Option<Self> {
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

impl<T: Scalar<Item = T>> Sub for MatrixOps<T> {
    type Output = Option<Self>;
    fn sub(&self, other: &Self) -> Option<Self> {
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
