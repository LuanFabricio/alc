use super::matrix::Matrix;

#[cfg(test)]
mod test;

pub fn vec_norm(vec: &Vec<f32>) -> f32 {
    let res = vec_inner_prod(vec, vec);

    res.sqrt()
}

pub fn vec_inner_prod(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    if a.len() != b.len() {
        panic!("Invalid sizes: {} - {}", a.len(), b.len());
    }

    let mut res = 0_f32;

    for i in 0..a.len() {
        res += a[i] * b[i];
    }

    res
}

pub fn col_vector(matrix: &Matrix, col_index: usize) -> Vec<f32> {
    if matrix.columns <= col_index {
        panic!("col_index invalid: {} >= {}", col_index, matrix.columns);
    }

    let mut col_vec = Vec::new();

    for i in 0..matrix.rows {
        col_vec.push(matrix[i][col_index]);
    }

    col_vec
}

pub fn mul_vector(vec: &Vec<f32>, lambda: f32) -> Vec<f32> {
    let mut vec_out = vec.clone();

    for i in 0..vec_out.len() {
        vec_out[i] *= lambda;
    }

    vec_out
}

pub fn sub_vector(vec_a: &Vec<f32>, vec_b: &Vec<f32>) -> Vec<f32> {
    let mut vec_c = vec_a.clone();

    for i in 0..vec_c.len() {
        vec_c[i] -= vec_b[i];
    }

    vec_c
}
