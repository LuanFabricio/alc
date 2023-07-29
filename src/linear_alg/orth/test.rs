use super::*;

mod norm {
    use super::*;

    #[test]
    fn should_return_matrix_norm() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        let a = Matrix::create_identity(ROWS, COLUMNS);

        const EXPECTED_NORM: f32 = 3_f32;
        assert_eq!(a.norm(), EXPECTED_NORM.sqrt())
    }

    #[test]
    fn should_be_greater_than_zero_if_the_matrix_isnt_zero() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        let mut a = Matrix::create_identity(ROWS, COLUMNS);

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                a[i][j] = (i - j) as f32;
            }
        }

        assert!(a.norm() > 0_f32);
    }
}

// TODO: write test
mod inner_product {
    use super::*;
}

// TODO: write test
mod trace {
    use super::*;
}