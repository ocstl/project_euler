use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    DimensionError,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Matrix<T: Clone> {
    rows: usize,
    columns: usize,
    data: Vec<T>,
}

impl<T: Clone + Debug> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix { rows: 0, columns: 0, data: Vec::new() }
    }

    pub fn from_iterator(rows: usize, cols: usize, input: impl IntoIterator<Item = T>)
            -> Result<Matrix<T>, MatrixError> {
        Matrix::from_iter(input).reshape(rows, cols)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn reshape(mut self, rows: usize, cols: usize) -> Result<Self, MatrixError> {
        if rows * cols == self.rows * self.columns {
            self.rows = rows;
            self.columns = cols;
            Ok(self)
        } else {
            Err(MatrixError::DimensionError)
        }
    }

    pub fn get(&self, idx: (usize, usize)) -> Option<&T> {
        if idx.0 < self.rows && idx.1 < self.columns {
            self.data.get(idx.0 * self.columns + idx.1)
        } else {
            None
        }
    }

    pub fn row_iterator(&self, row: usize) -> MatrixIterator<T> {
        MatrixIterator::new(
            MatrixIteratorType::Row, &self, (row, 0)
        )
    }

    pub fn column_iterator(&self, column: usize) -> MatrixIterator<T> {
        MatrixIterator::new(
            MatrixIteratorType::Column, &self, (0, column)
        )
    }

    pub fn diagonal_iterator(&self, row: usize, column: usize) -> MatrixIterator<T> {
        MatrixIterator::new(
            MatrixIteratorType::Diagonal, &self, (row, column)
        )
    }

    pub fn inverse_diagonal_iterator(&self, row: usize, column: usize) -> MatrixIterator<T> {
        MatrixIterator::new(
            MatrixIteratorType::InverseDiagonal, &self, (row, column)
        )
    }

    pub fn iterator_over_rows(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        (0..self.rows)
            .map(move|row| self.row_iterator(row))
    }

    pub fn iterator_over_columns(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        (0..self.columns)
            .map(move |column| self.column_iterator(column))
    }

    pub fn iterator_over_upper_diagonals(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        (0..self.columns)
            .map(move |column| self.diagonal_iterator(0, column))
    }

    pub fn iterator_over_lower_diagonals(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        (0..self.rows)
            .map(move |row| self.diagonal_iterator(row, 0))
    }

    /* Iterate over the upper right diagonals, then the lower left. */
    pub fn iterator_over_diagonals(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        self.iterator_over_upper_diagonals()
            .chain(self.iterator_over_lower_diagonals().skip(1))
    }

    pub fn iterator_over_upper_left_diagonals(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        (0..self.rows).rev()
            .map(move |row| self.inverse_diagonal_iterator(row, 0))
    }

    pub fn iterator_over_lower_right_diagonals(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        (0..self.columns)
            .map(move |column| self.inverse_diagonal_iterator(self.rows - 1, column))
    }

    /* Iterate over the upper left diagonals, then the lower right. */
    pub fn iterator_over_reverse_diagonals(&self) -> impl Iterator<Item = MatrixIterator<T>> {
        self.iterator_over_upper_left_diagonals()
            .chain(self.iterator_over_lower_right_diagonals().skip(1))
    }
}

impl<T: Clone + Debug> FromIterator<T> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut matrix = Matrix::new();

        for i in iter {
            matrix.data.push(i);
        }

        matrix.rows = 1;
        matrix.columns = matrix.data.len();
        matrix
    }
}

enum MatrixIteratorType {
    Row,
    Column,
    Diagonal,
    InverseDiagonal,
}

pub struct MatrixIterator<'a, T: Clone> {
    t: MatrixIteratorType,
    matrix: &'a Matrix<T>,
    idx: Option<(usize, usize)>,
}

impl<'a, T: Clone + Debug> MatrixIterator<'a, T> {
    fn new(t: MatrixIteratorType, matrix: &'a Matrix<T>, idx: (usize, usize))
           -> MatrixIterator<'a, T> {
        MatrixIterator { t, matrix, idx: Some(idx) }
    }
}

impl<'a, T: Clone + Debug> Iterator for MatrixIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(i) = self.idx {
            let result = self.matrix.get(i);
            self.idx = match self.t {
                MatrixIteratorType::Row => Some((i.0, i.1 + 1)),
                MatrixIteratorType::Column => Some((i.0 + 1, i.1)),
                MatrixIteratorType::Diagonal => Some((i.0 + 1, i.1 + 1)),
                MatrixIteratorType::InverseDiagonal => {
                    if i.0 == 0 {
                        None
                    } else {
                        Some((i.0 - 1, i.1 + 1))
                    }
                },
            };

            result
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sample_matrix {
        ($x:ident) => {
            let $x = Matrix { rows: 2, columns: 2, data: vec![1, 2, 3, 4] };
        }
    }

    #[test]
    fn from_iterator() {
        let actual = Matrix::from_iterator(2, 2, [1, 2, 3, 4].to_vec()).unwrap();
        sample_matrix!(expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dimension_error() {
        let actual = Matrix::from_iterator(1, 2, [1, 2, 3, 4].to_vec()).unwrap_err();
        let expected = MatrixError::DimensionError;
        assert_eq!(actual, expected);
    }

    #[test]
    fn row_iterator() {
        sample_matrix!(test);
        dbg!(test.clone());
        let mut iterators = test.iterator_over_rows();
        let mut iterator = iterators.next().unwrap();
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), None)
    }

    #[test]
    fn columns_iterator() {
        sample_matrix!(test);
        dbg!(test.clone());
        let mut iterators = test.iterator_over_columns();
        let mut iterator = iterators.next().unwrap();
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), None)
    }

    #[test]
    fn diagonals_iterator() {
        sample_matrix!(test);
        dbg!(test.clone());
        let mut iterators = test.iterator_over_diagonals();
        let mut iterator = iterators.next().unwrap();
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), None)
    }

    #[test]
    fn reverse_diagonals_iterator() {
        sample_matrix!(test);
        dbg!(test.clone());
        let mut iterators = test.iterator_over_reverse_diagonals();
        let mut iterator = iterators.next().unwrap();
        dbg!(iterator.idx);
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), None)
    }
}