use crate::structs::matrix::Matrix;

mod change_line {
    use super::*;

    #[test]
    fn should_create_a_change_line_matrix() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        const LINE_1: usize = 0;
        const LINE_2: usize = 2;

        let matrix_change_line = Matrix::change_line(ROWS, COLUMNS, LINE_1, LINE_2);

        assert_eq!(matrix_change_line.content[LINE_1][LINE_2], 1_f32);
        assert_eq!(matrix_change_line.content[LINE_2][LINE_1], 1_f32);
    }

    #[test]
    fn should_change_line() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        const LINE_1: usize = 0;
        const LINE_2: usize = 1;

        let matrix_change_line = Matrix::change_line(ROWS, COLUMNS, LINE_1, LINE_2);

        let mut matrix = Matrix::create_identity(COLUMNS, ROWS);
        const ALPHA: f32 = 3.5;

        matrix.content[LINE_1][LINE_1] *= ALPHA;
        matrix.content[LINE_2][LINE_2] *= ALPHA * ALPHA;

        let matrix = matrix_change_line * matrix;

        assert_eq!(matrix.content[LINE_1][LINE_2], ALPHA * ALPHA);
        assert_eq!(matrix.content[LINE_2][LINE_1], ALPHA);
    }
}

mod multiply_line {
    use super::*;

    #[test]
    fn should_create_a_multiply_line_matrix() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        const LINE: usize = 2;
        const ALPHA: f32 = 4.2_f32;

        let matrix_multiply_line = Matrix::multiply_line(ROWS, COLUMNS, LINE, ALPHA);

        assert_eq!(matrix_multiply_line.content[LINE][LINE], ALPHA);
    }

    #[test]
    fn should_multiply_line() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        const LINE: usize = 2;
        const ALPHA: f32 = 4.2;

        let matrix_multiply_line = Matrix::multiply_line(ROWS, COLUMNS, LINE, ALPHA);

        let matrix = Matrix::create_identity(COLUMNS, ROWS);
        let matrix = matrix_multiply_line * matrix;

        assert_eq!(matrix.content[LINE][LINE], ALPHA);
    }
}

mod sum_line {
    use super::*;

    #[test]
    fn should_create_a_sum_line_matrix() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        const LINE_1: usize = 0;
        const LINE_2: usize = 1;
        const ALPHA: f32 = 4.2_f32;

        let matrix_sum_line = Matrix::sum_line(ROWS, COLUMNS, LINE_1, LINE_2, ALPHA);

        assert_eq!(matrix_sum_line.content[LINE_1][LINE_2], ALPHA);
    }

    #[test]
    fn should_sum_line() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        const LINE_1: usize = 0;
        const LINE_2: usize = 1;
        const ALPHA: f32 = 2_f32;

        let matrix_sum_line = Matrix::sum_line(ROWS, COLUMNS, LINE_1, LINE_2, ALPHA);

        let mut matrix = Matrix::create_identity(COLUMNS, ROWS);
        matrix.content[LINE_2][LINE_1] = 1_f32;
        let matrix = matrix_sum_line * matrix;

        assert_eq!(matrix.content[LINE_1][LINE_2], ALPHA);
        assert_eq!(matrix.content[LINE_1][LINE_1], ALPHA + 1_f32);
    }
}
