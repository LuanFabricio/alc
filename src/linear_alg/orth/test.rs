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
                a[i][j] = (i + j) as f32;
            }
        }

        assert!(a.norm() > 0_f32);
    }
}

// TODO: write test
mod inner_product {
    use super::*;

    #[test]
    fn should_return_inner_prod() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        let a = Matrix::create_identity(ROWS, COLUMNS);

        const EXPECTED_PROD: f32 = 3_f32;

        assert_eq!(a.inner_product(&a), EXPECTED_PROD);
    }
}

// TODO: write test
mod trace {
    use super::*;

    #[test]
    fn should_sum_main_diagonal() {
        const SIZE: usize = 3;

        let a = Matrix::create_identity(SIZE, SIZE);

        assert_eq!(trace(&a), SIZE as f32);
    }

    #[test]
    fn should_sum_smallest_size_between_rows_and_columns() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        let a = Matrix::create_identity(ROWS, COLUMNS);

        assert_eq!(trace(&a), ROWS as f32);
    }
}

mod ortho_gram_schmidt {
    use super::*;

    #[test]
    fn should_create_vectors() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 3;

        let mut matrix = Matrix::new(ROWS, COLUMNS, None);

        matrix.set_col_vector(0, vec![0_f32, 1_f32, 1_f32]);
        matrix.set_col_vector(1, vec![1_f32, 0_f32, 1_f32]);
        matrix.set_col_vector(2, vec![1_f32, 1_f32, 0_f32]);

        let ortho = matrix.ortho_gram_schmidt();

        let expected_q1 = vec![0_f32, 1_f32 / 2_f32.sqrt(), 1_f32 / 2_f32.sqrt()];
        let expected_q2 = vec![
            2_f32 / 6_f32.sqrt(),
            -1_f32 / 6_f32.sqrt(),
            1_f32 / 6_f32.sqrt(),
        ];
        let expected_q3 = vec![
            1_f32 / 3_f32.sqrt(),
            1_f32 / 3_f32.sqrt(),
            -1_f32 / 3_f32.sqrt(),
        ];

        let expected_q_list = [expected_q1, expected_q2, expected_q3];

        for i in 0..expected_q_list.len() {
            let expected_q = expected_q_list[i].clone();
            let current_q = col_vector(&ortho, i);
            for j in 0..ROWS {
                assert!(current_q[j] - expected_q[j] < f32::EPSILON);
            }
        }
    }

    #[test]
    fn should_create_ortho_vectors() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 3;

        let mut matrix = Matrix::new(ROWS, COLUMNS, None);

        matrix.set_col_vector(0, vec![0_f32, 1_f32, 1_f32]);
        matrix.set_col_vector(1, vec![1_f32, 0_f32, 1_f32]);
        matrix.set_col_vector(2, vec![1_f32, 1_f32, 0_f32]);

        let ortho = matrix.ortho_gram_schmidt();

        for i in 0..COLUMNS {
            let a = col_vector(&ortho, i);
            for j in i + 1..COLUMNS {
                let b = col_vector(&ortho, j);
                assert!(vec_inner_prod(&a, &b) < f32::EPSILON);
            }
        }
    }

    #[test]
    fn should_create_identity_when_mulptiply_by_transpose() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 3;

        let mut matrix = Matrix::new(ROWS, COLUMNS, None);

        matrix.set_col_vector(0, vec![0_f32, 1_f32, 1_f32]);
        matrix.set_col_vector(1, vec![1_f32, 0_f32, 1_f32]);
        matrix.set_col_vector(2, vec![1_f32, 1_f32, 0_f32]);

        let ortho = matrix.ortho_gram_schmidt();

        let res = ortho.transpose() * ortho;
        let indentity = Matrix::create_identity(ROWS, COLUMNS);

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                assert!(res[i][j] - indentity[i][j] < f32::EPSILON);
            }
        }
    }
}

mod dec_rq {
    use super::*;

    #[test]
    fn should_the_q_and_r_results_should_generate_the_original_matrix() {
        const ROWS: usize = 5;
        const COLUMNS: usize = 4;

        let mut matrix = Matrix::new(ROWS, COLUMNS, None);

        // Writing the original matrix.
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                matrix[i][j] = 1_f32 + (i * COLUMNS + j) as f32;
            }
        }

        let (q, r) = matrix.dec_qr();

        let qr = q * r;

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                assert!(matrix[i][j] - qr[i][j].round() < f32::EPSILON);
            }
        }
    }

    #[should_panic]
    #[test]
    fn should_panic_if_matrix_rows_are_bigger_than_columns() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 4;

        let matrix = Matrix::new(ROWS, COLUMNS, None);

        matrix.dec_qr();
    }

    #[test]
    fn should_calc_qr() {
        const ROWS: usize = 4;
        const COLUMNS: usize = 3;

        let mut matrix = Matrix::new(ROWS, COLUMNS, None);
        matrix[0] = vec![1_f32, 1_f32, 1_f32];
        matrix[1] = vec![0_f32, 1_f32, 1_f32];
        matrix[2] = vec![1_f32, 1_f32, 1_f32];
        matrix[3] = vec![1_f32, 0_f32, 1_f32];

        let mut expected_q = Matrix::new(ROWS, COLUMNS, None);
        expected_q[0] = vec![
            1_f32 / 3_f32.sqrt(),
            1_f32 / 15_f32.sqrt(),
            -(10_f32.sqrt()) / 10_f32,
        ];

        expected_q[1] = vec![0_f32, 3_f32 / 15_f32.sqrt(), 10_f32.sqrt() / 10_f32];

        expected_q[2] = vec![
            1_f32 / 3_f32.sqrt(),
            1_f32 / 15_f32.sqrt(),
            -(10_f32.sqrt()) / 10_f32,
        ];

        expected_q[3] = vec![
            1_f32 / 3_f32.sqrt(),
            -2_f32 / 15_f32.sqrt(),
            10_f32.sqrt() / 10_f32,
        ];

        let mut expected_r = Matrix::new(COLUMNS, COLUMNS, None);

        expected_r[0] = vec![3_f32.sqrt(), 2_f32 / 3_f32.sqrt(), 3_f32.sqrt()];

        expected_r[1] = vec![0_f32, 15_f32.sqrt() / 3_f32, 3_f32 / 15_f32.sqrt()];

        expected_r[2][2] = 10_f32.sqrt() / 5_f32;

        let (q, r) = matrix.dec_qr();

        assert_eq!(q.rows, expected_q.rows);
        assert_eq!(q.columns, expected_q.columns);

        assert_eq!(r.rows, expected_r.rows);
        assert_eq!(r.columns, expected_r.columns);

        // Testing q
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                assert!(expected_q[i][j] - q[i][j] < f32::EPSILON);
            }
        }

        // Testing r
        for i in 0..COLUMNS {
            for j in 0..COLUMNS {
                assert!(expected_r[i][j] - r[i][j] < f32::EPSILON);
            }
        }
    }
}
