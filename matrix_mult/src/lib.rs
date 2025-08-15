use lalgebra_scalar::Scalar;
use std::ops::Mul;

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

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T: Scalar<Item = T>> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }
        let mut result = Matrix::zero(self.number_of_rows(), other.number_of_cols());
        for i in 0..self.number_of_rows() {
            for j in 0..other.number_of_cols() {
                let mut sum = T::zero();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k] * other.0[k][j];
                }
                result.0[i][j] = sum;
            }
        }
        Some(result)
    }
}
