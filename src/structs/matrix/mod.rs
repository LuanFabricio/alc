use std::ops::{Add, Index, IndexMut, Mul};

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub content: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(rows: usize, columns: usize, initial_value: Option<f32>) -> Self {
        let initial_value = initial_value.unwrap_or(0_f32);

        Self {
            rows,
            columns,
            content: vec![vec![initial_value; columns]; rows],
        }
    }

    pub fn from(content: Vec<Vec<f32>>) -> Self {
        let rows = content.len();
        let columns = content[0].len();

        for i in 0..rows {
            assert_eq!(content[i].len(), columns);
        }

        Self {
            rows,
            columns,
            content,
        }
    }

    pub fn create_identity(rows: usize, columns: usize) -> Self {
        let mut matrix = Self::new(rows, columns, Some(0_f32));

        let n = if rows > columns { columns } else { rows };

        for i in 0..n {
            matrix.content[i][i] = 1_f32;
        }

        matrix
    }

    pub fn transpose(&self) -> Self {
        let mut t = Self::new(self.columns, self.rows, None);

        for i in 0..self.rows {
            for j in 0..self.columns {
                t.content[j][i] = self.content[i][j];
            }
        }

        t
    }

    pub fn set_col_vector(&mut self, col_index: usize, vec: Vec<f32>) {
        if self.rows != vec.len() {
            panic!("Matrix rows != vec.len(), {} != {}", self.rows, vec.len());
        }

        for i in 0..self.rows {
            self[i][col_index] = vec[i];
        }
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.rows != other.rows || self.columns != other.columns {
            panic!("Size not compatible");
        }

        let mut output = self.content.clone();

        for i in 0..self.rows {
            for j in 0..self.columns {
                output[i][j] = self.content[i][j] + other.content[i][j];
            }
        }

        Self {
            rows: self.rows,
            columns: self.columns,
            content: output,
        }
    }
}

// TODO: research how to use reference (`&self` and `other: &Self`)
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.columns != other.rows {
            panic!("Size not compatible, {} != {}", self.columns, other.rows);
        }

        let rows = self.rows;
        let columns = other.columns;
        let k = self.columns;

        let mut output = vec![vec![0_f32; columns]; rows];

        // TODO: Move to anoter function.
        for i in 0..rows {
            for j in 0..columns {
                // Calc cell value
                for c in 0..k {
                    output[i][j] += self.content[i][c] * other.content[c][j];
                }
            }
        }

        Self {
            rows,
            columns,
            content: output,
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        self.content.get(index).unwrap()
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.content.get_mut(index).unwrap()
    }
}
