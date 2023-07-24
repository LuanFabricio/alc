use super::*;

mod new {
    use super::*;

    #[test]
    fn should_initialize_without_initial_value() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 6;

        let matrix = Matrix::new(ROWS, COLUMNS, None);

        assert_eq!(matrix.rows, ROWS);
        assert_eq!(matrix.content.len(), ROWS);

        assert_eq!(matrix.columns, COLUMNS);
        assert_eq!(matrix.content[0].len(), COLUMNS);

        const DEFAULT_INITIAL_VALUE: f32 = 0_f32;
        assert_eq!(matrix.content[0][0], DEFAULT_INITIAL_VALUE);
    }

    #[test]
    fn should_initialize_with_initial_value() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 6;
        const INITIAL_VALUE: f32 = 42_f32;

        let matrix = Matrix::new(ROWS, COLUMNS, Some(INITIAL_VALUE));

        assert_eq!(matrix.rows, ROWS);
        assert_eq!(matrix.content.len(), ROWS);

        assert_eq!(matrix.columns, COLUMNS);
        assert_eq!(matrix.content[0].len(), COLUMNS);

        assert_eq!(matrix.content[0][0], INITIAL_VALUE);
    }
}

mod create_identity {
    use super::*;

    #[test]
    fn should_create_identity_matrix() {
        const SIZE: usize = 3;

        let identity = Matrix::create_identity(SIZE, SIZE);

        for i in 0..SIZE {
            assert!(identity.content[i][i] == 1_f32);
        }
    }
}

mod add {
    use super::*;

    #[test]
    fn should_add_if_sizes_are_equal() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 3;

        let matrix_a = Matrix::new(ROWS, COLUMNS, None);
        let matrix_b = Matrix::new(ROWS, COLUMNS, None);

        let matrix_c = matrix_a + matrix_b;

        assert_eq!(matrix_c.rows, ROWS);
        assert_eq!(matrix_c.columns, COLUMNS);
    }

    #[should_panic]
    #[test]
    fn should_panic_if_rows_arent_equal() {
        const COLUMNS: usize = 3;
        const ROWS_A: usize = 1;
        const ROWS_B: usize = 2;

        let matrix_a = Matrix::new(ROWS_A, COLUMNS, None);
        let matrix_b = Matrix::new(ROWS_B, COLUMNS, None);

        let _ = matrix_a + matrix_b;
    }

    #[should_panic]
    #[test]
    fn should_panic_if_columns_arent_equal() {
        const ROWS: usize = 3;
        const COLUMNS_A: usize = 1;
        const COLUMNS_B: usize = 2;

        let matrix_a = Matrix::new(ROWS, COLUMNS_A, None);
        let matrix_b = Matrix::new(ROWS, COLUMNS_B, None);

        let _ = matrix_a + matrix_b;
    }
}

mod mul {
    use super::*;

    #[test]
    fn should_multiply_if_sizes_are_ok() {
        const ROWS_A: usize = 3;
        const COLUMNS_A: usize = 4;
        const COLUMNS_B: usize = 2;

        let matrix_a = Matrix::new(ROWS_A, COLUMNS_A, None);
        let matrix_b = Matrix::new(COLUMNS_A, COLUMNS_B, None);

        let matrix_c = matrix_a * matrix_b;

        assert_eq!(matrix_c.rows, ROWS_A);
        assert_eq!(matrix_c.columns, COLUMNS_B);
    }

    #[should_panic]
    #[test]
    fn should_panic_if_a_columns_arent_equal_a_b_rows() {
        const ROWS_A: usize = 3;
        const COLUMNS_A: usize = 4;
        const ROWS_B: usize = 3;
        const COLUMNS_B: usize = 2;

        let matrix_a = Matrix::new(ROWS_A, COLUMNS_A, None);
        let matrix_b = Matrix::new(ROWS_B, COLUMNS_B, None);

        let _ = matrix_a * matrix_b;
    }
}
