use crate::structs::matrix::Matrix;

#[cfg(test)]
mod test;

impl Matrix {
    pub fn change_line(rows: usize, columns: usize, line1: usize, line2: usize) -> Matrix {
        let mut output = Matrix::create_identity(rows, columns);

        let tmp = output.content[line2].clone();
        output.content[line2] = output.content[line1].clone();
        output.content[line1] = tmp;

        output
    }

    pub fn multiply_line(rows: usize, columns: usize, line: usize, alpha: f32) -> Matrix {
        let mut output = Matrix::create_identity(rows, columns);

        output.content[line][line] *= alpha;

        output
    }

    pub fn sum_line(rows: usize, columns: usize, line1: usize, line2: usize, alpha: f32) -> Matrix {
        let mut output = Matrix::create_identity(rows, columns);

        output.content[line1][line2] = alpha;

        output
    }
}
