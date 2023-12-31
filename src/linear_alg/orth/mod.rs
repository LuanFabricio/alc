#[cfg(test)]
mod test;

use crate::structs::{
    matrix::Matrix,
    vector::{col_vector, mul_vector, sub_vector, vec_inner_prod, vec_norm},
};

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

        let res = a * b;
        trace(&res)
    }

    fn dec_qr(&self) -> (Matrix, Matrix) {
        assert!(self.rows >= self.columns);

        let mut q = Matrix::new(self.rows, self.columns, None);
        let mut r = Matrix::new(self.columns, self.columns, None);

        let n = if self.columns > self.rows {
            self.rows
        } else {
            self.columns
        };

        for i in 0..n {
            let mut w = col_vector(self, i);

            for j in 0..i {
                let q_col = col_vector(&q, j);
                let proj = vec_inner_prod(&w.clone(), &q_col);
                r[j][i] = proj;
                let proj = mul_vector(&q_col, proj);
                w = sub_vector(&w, &proj);
            }
            let norm = vec_norm(&w);
            r[i][i] = norm;
            let vec_q = mul_vector(&w, 1_f32 / norm);
            q.set_col_vector(i, vec_q);
        }

        (q, r)
    }

    fn ortho_gram_schmidt(&self) -> Self {
        let mut q = Matrix::new(self.rows, self.columns, Some(0_f32));

        for i in 0..self.columns {
            let mut w = col_vector(self, i);

            for j in 0..i {
                let q_col = col_vector(&q, j);
                let proj = vec_inner_prod(&w.clone(), &q_col);
                let proj = mul_vector(&q_col, proj);
                w = sub_vector(&w, &proj);
            }
            let vec_q = mul_vector(&w, 1_f32 / vec_norm(&w));
            q.set_col_vector(i, vec_q);
        }

        q
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
