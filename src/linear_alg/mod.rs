mod elementary_matrix;
mod orth;

use crate::structs::matrix::Matrix;

// FIX: move solve system to another file.
pub fn solve_system(mut system: Matrix) -> Matrix {
    for i in 1..system.rows {
        let (mut pivot_index, mut pivot) = current_pivot(&system, i);

        // TODO: review for linear dependency
        while pivot_index < i {
            let sum_line = find_next_matrix(&system, i, pivot_index, pivot);
            system = sum_line * system;
            (pivot_index, pivot) = current_pivot(&system, i);
        }
    }

    system
}

pub fn calc_lu(mut system: Matrix) -> (Matrix, Matrix) {
    let mut l_matrices = vec![];

    // TODO: Move the solve problem core to another function and reuse for solve_system and calc_lu.
    for i in 1..system.rows {
        let (mut pivot_index, mut pivot) = current_pivot(&system, i);

        while pivot_index < i {
            let sum_line = find_next_matrix(&system, i, pivot_index, pivot);
            system = sum_line * system;
            l_matrices.push(find_next_matrix(&system, i, pivot_index, -pivot));
            (pivot_index, pivot) = current_pivot(&system, i);
        }
    }

    l_matrices.reverse();
    let mut mat_l = Matrix::create_identity(system.rows, system.columns);
    for sum_mat in l_matrices {
        mat_l = sum_mat * mat_l;
    }

    (mat_l, system)
}

fn find_next_matrix(matrix: &Matrix, line: usize, pivot_index: usize, pivot: f32) -> Matrix {
    Matrix::sum_line(
        matrix.rows,
        matrix.columns,
        line,
        pivot_index,
        -pivot / matrix[pivot_index][pivot_index],
    )
}

fn current_pivot(system: &Matrix, line: usize) -> (usize, f32) {
    let mut current_column = 0;
    for cell in system[line].iter() {
        if *cell != 0_f32 {
            return (current_column, *cell);
        }
        current_column += 1;
    }

    (current_column, 0_f32)
}

// FIX: move tests to another file and review tests.
#[cfg(test)]
mod test {
    use super::*;

    mod solve_system {
        use super::*;

        #[test]
        fn should_find_systems_solution() {
            const ROWS: usize = 3;
            const COLUMNS: usize = 3;

            let mut matrix = Matrix::new(ROWS, COLUMNS, None);

            // row 1 = [1, 1, 1]
            matrix[0][0] = 1_f32;
            matrix[0][1] = 1_f32;
            matrix[0][2] = 1_f32;

            // row 2 = [2, 3, 4]
            matrix[1][0] = 2_f32;
            matrix[1][1] = 3_f32;
            matrix[1][2] = 4_f32;

            // row 3 = [3, 4 ,10]
            matrix[2][0] = 3_f32;
            matrix[2][1] = 4_f32;
            matrix[2][2] = 10_f32;

            let mut expected_solution = Matrix::new(ROWS, COLUMNS, None);
            // row 1 = [1, 1, 1]
            expected_solution[0][0] = 1_f32;
            expected_solution[0][1] = 1_f32;
            expected_solution[0][2] = 1_f32;

            // row 2 = [0, 1, 2]
            expected_solution[1][0] = 0_f32;
            expected_solution[1][1] = 1_f32;
            expected_solution[1][2] = 2_f32;

            // row 2 = [0, 0, 5]
            expected_solution[2][0] = 0_f32;
            expected_solution[2][1] = 0_f32;
            expected_solution[2][2] = 5_f32;

            matrix = solve_system(matrix);

            for i in 0..matrix.rows {
                assert_eq!(matrix[i], expected_solution[i]);
            }
        }
    }

    mod calc_lu {
        use super::*;

        // TODO: Maybe refactor later
        #[test]
        fn should_find_l_and_u_matrix() {
            const ROWS: usize = 3;
            const COLUMNS: usize = 3;

            let mut matrix = Matrix::new(ROWS, COLUMNS, None);
            // row 1 [3 2 4]
            matrix[0][0] = 1_f32;
            matrix[0][1] = 1_f32;
            matrix[0][2] = 1_f32;
            // row 2 [1 1 2]
            matrix[1][0] = 1_f32;
            matrix[1][1] = 2_f32;
            matrix[1][2] = 2_f32;
            // row 3 [4 3 2]
            matrix[2][0] = 4_f32;
            matrix[2][1] = 3_f32;
            matrix[2][2] = 2_f32;

            // expected l
            // 1  0 0
            // 1  1 0
            // 4 -1 0
            let mut expected_l = Matrix::create_identity(ROWS, COLUMNS);
            expected_l[1][0] = 1_f32;
            expected_l[2][0] = 4_f32;
            expected_l[2][1] = -1_f32;

            // expected u
            // 1 1  1
            // 0 1  1
            // 0 0 -1
            let mut expected_u = Matrix::create_identity(ROWS, COLUMNS);
            expected_u[0][1] = 1_f32;
            expected_u[0][2] = 1_f32;
            expected_u[1][2] = 1_f32;
            expected_u[2][2] = -1_f32;

            let (l, u) = calc_lu(matrix);

            for i in 0..ROWS {
                assert_eq!(l[i], expected_l[i]);
                assert_eq!(u[i], expected_u[i]);
            }
        }
    }

    mod find_next_matrix {
        use super::*;

        #[test]
        fn should_get_next_elementary_matrix() {
            const ROWS: usize = 3;
            const COLUMNS: usize = 4;
            const INITIAL_VALUE: f32 = 1_f32;

            let mut matrix = Matrix::new(ROWS, COLUMNS, Some(INITIAL_VALUE));

            matrix[1][2] = 0_f32;
            matrix[2][0] = 0_f32;

            let expected_matrices = [
                Matrix::sum_line(ROWS, COLUMNS, 0, 0, -1_f32), // +/- identity
                Matrix::sum_line(ROWS, COLUMNS, 1, 0, -1_f32),
                Matrix::sum_line(ROWS, COLUMNS, 2, 1, -1_f32),
            ];

            for i in 0..ROWS {
                let (pivot_index, pivot) = current_pivot(&matrix, i);
                let sum_matrix = find_next_matrix(&matrix, i, pivot_index, pivot);
                assert_eq!(sum_matrix[i], expected_matrices[i][i]);
            }
        }
    }

    mod current_pivot {
        use super::*;

        #[test]
        fn should_get_firs_non_zero_item() {
            const ROWS: usize = 3;
            const COLUMNS: usize = 4;
            const DEFAULT_CELL_VALUE: f32 = 1_f32;

            let mut matrix = Matrix::new(ROWS, COLUMNS, Some(DEFAULT_CELL_VALUE));

            const TEST_CELLS: [(usize, usize); 3] = [(1, 2), (2, 0), (2, 1)];
            matrix[1][2] = 0_f32;
            matrix[2][0] = 0_f32;
            matrix[2][1] = 0_f32;

            for (row, column) in TEST_CELLS {
                matrix[row][column] = 0_f32;
            }

            const EXPECTED_PIVOTS: [(usize, f32); 3] = [
                (0, DEFAULT_CELL_VALUE),
                (0, DEFAULT_CELL_VALUE),
                (2, DEFAULT_CELL_VALUE),
            ];

            for i in 0..ROWS {
                let (pivot_index, pivot) = current_pivot(&matrix, i);

                assert_eq!(pivot_index, EXPECTED_PIVOTS[i].0);
                assert_eq!(pivot, EXPECTED_PIVOTS[i].1);
            }
        }
    }
}
