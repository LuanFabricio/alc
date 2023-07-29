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

    fn inner_product(&self, other: &Matrix) -> f32 {
        let a = self.transpose();
        let b = other.clone();

        println!("{}, {}", a.rows, a.columns);
        println!("{}, {}", b.rows, b.columns);

        let res = a * b;
        trace(&res)
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
