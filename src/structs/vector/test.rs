use super::*;

mod vec_norm {
    use super::*;

    #[test]
    fn should_return_vector_norm() {
        let vector = vec![2_f32; 4];

        const EXPECTED_NORM: f32 = 4_f32;
        let norm = vec_norm(&vector);

        assert_eq!(norm, EXPECTED_NORM);
    }
}

mod vec_inner_prod {
    use super::*;

    #[test]
    fn should_return_inner_prod() {
        let vector_a = vec![2_f32; 2];

        let inner_prod = vec_inner_prod(&vector_a, &vector_a);

        // <vector_a, vector_a> = ||vector_a||
        const EXPECTED_INNER_PROD: f32 = 8_f32;

        assert_eq!(inner_prod, EXPECTED_INNER_PROD);
    }

    // TODO: Write another test for inner_prod (without <a, a>)
    #[test]
    fn should_verify_ortho_between_two_vectors() {
        let vector_a = vec![1_f32, 0_f32];
        let vector_b = vec![0_f32, 1_f32];

        let inner_prod = vec_inner_prod(&vector_a, &vector_b);

        assert!(inner_prod < f32::EPSILON);
    }
}

mod col_vector {
    use super::*;

    #[test]
    fn should_return_a_vector_with_the_column_content() {
        const ROWS: usize = 3;
        const COLUMNS: usize = 3;
        let mut matrix = Matrix::new(ROWS, COLUMNS, None);

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                matrix[i][j] = j as f32;
            }
        }

        for i in 0..COLUMNS {
            let col = col_vector(&matrix, i);
            let v = vec![i as f32; ROWS];

            assert_eq!(col, v);
        }
    }
}

mod mul_vector {
    use super::*;

    #[test]
    fn should_multiply_a_vector_by_a_scalar() {
        const LAMBDA: f32 = 2_f32;

        let vector = vec![1_f32; 3];

        let vector_lambda = mul_vector(&vector, LAMBDA);

        let expected_vector = vec![2_f32; 3];

        assert_eq!(vector_lambda, expected_vector);
    }
}

mod sub_vector {
    use super::*;

    #[test]
    fn should_subtract_vectors() {
        let vector_a = vec![5_f32; 3];
        let vector_b = vec![2_f32; 3];

        let result = sub_vector(&vector_a, &vector_b);

        let expected_result = vec![3_f32; 3];

        assert_eq!(result, expected_result);
    }
}

mod is_ortho {
    use super::*;

    #[test]
    fn should_be_false_if_inner_prod_is_zero() {
        let vector_a = &vec![1_f32, 0_f32];
        let vector_b = &vec![0_f32, 1_f32];

        let inner_prod = vec_inner_prod(vector_a, vector_b);
        let is_zero = inner_prod < f32::EPSILON;

        let is_ortho = vec_is_ortho(vector_a, vector_b);
        assert_eq!(is_ortho, is_zero);
    }
}
