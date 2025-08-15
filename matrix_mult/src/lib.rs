use lalgebra_scalar::Scalar;
use std::ops::Mul;

impl Matrix<T> {
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
                result.0[i][j] = self
                    .row(i)
                    .iter()
                    .zip(other.col(j).iter())
                    .map(|(a, b)| *a * *b)
                    .sum();
            }
        }
        Some(result)
    }
}
