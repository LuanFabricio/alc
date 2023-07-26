#[cfg(test)]
mod test;

use crate::structs::matrix::Matrix;

impl Matrix {
    fn norm(&self) -> f32 {
        let a = self.clone();
        let a_t = self.transpose();

        let res = a * a_t;
        trace(&res).sqrt()
    }

    fn inner_product(&self, other: &Matrix) -> Self {
        let a = self.transpose();
        let b = other.clone();

        a * b
    }
}

pub fn trace(matrix: &Matrix) -> f32 {
    let mut sum = 0_f32;

    let n = if matrix.rows > matrix.columns {
        matrix.columns
    } else {
        matrix.rows
    };

    for i in 0..n {
        sum += matrix[i][i];
    }

    sum
}
