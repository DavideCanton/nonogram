use array2d::Array2D;
use itertools::Itertools;
use log::{info, log, Level};
use std::{
    error,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Full,
    Crossed,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Full => write!(f, "O"),
            Self::Crossed => write!(f, "X"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidLabel(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLabel(s) => write!(f, "Invalid label, {}", s),
        }
    }
}

impl error::Error for Error {}

pub type Labels = Vec<usize>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct NonogramSchema {
    data: Array2D<Cell>,
    row_lbl: Vec<Labels>,
    col_lbl: Vec<Labels>,
}

fn _validate(rows: usize, rows_labels: &Vec<Labels>, cols: usize) -> Result<()> {
    if rows_labels.len() != rows {
        let msg = format!("expected {}, found {}", rows, rows_labels.len());
        return Err(Error::InvalidLabel(msg));
    }

    for (index, labels) in rows_labels.iter().enumerate() {
        let v = labels.iter().copied().sum::<usize>() + labels.len() - 1;
        if v > cols {
            let msg = format!(
                "Label sum {} exceeds maximum allowed {} at index {}",
                v, cols, index
            );
            return Err(Error::InvalidLabel(msg));
        }
    }

    Ok(())
}

impl NonogramSchema {
    pub fn new(
        rows: usize,
        cols: usize,
        row_labels: Vec<Labels>,
        col_labels: Vec<Labels>,
    ) -> Result<Self> {
        _validate(rows, &row_labels, cols)?;
        _validate(cols, &col_labels, rows)?;

        let schema = NonogramSchema {
            data: Array2D::filled_with(Cell::Empty, rows, cols),
            row_lbl: row_labels,
            col_lbl: col_labels,
        };
        Ok(schema)
    }

    pub fn rows(&self) -> usize {
        self.data.num_rows()
    }

    pub fn cols(&self) -> usize {
        self.data.num_columns()
    }

    pub fn row_at(&self, i: usize) -> Vec<Cell> {
        self.data.row_iter(i).unwrap().copied().collect()
    }

    pub fn set_row_at(&mut self, i: usize, row: &[Cell]) {
        for (j, r) in row.iter().enumerate() {
            self.data.set(i, j, *r).unwrap();
        }
    }

    pub fn col_at(&self, j: usize) -> Vec<Cell> {
        self.data.column_iter(j).unwrap().copied().collect()
    }

    pub fn set_col_at(&mut self, j: usize, col: &[Cell]) {
        for (i, r) in col.iter().enumerate() {
            self.data.set(i, j, *r).unwrap();
        }
    }

    pub fn row_label_at(&self, i: usize) -> &[usize] {
        &self.row_lbl[i]
    }

    pub fn col_label_at(&self, j: usize) -> &[usize] {
        &self.col_lbl[j]
    }

    pub fn solved_row(&self, i: usize) -> bool {
        self.is_solved(self.data.row_iter(i).unwrap().copied(), &self.row_lbl[i])
    }

    pub fn solved_col(&self, j: usize) -> bool {
        self.is_solved(self.data.column_iter(j).unwrap().copied(), &self.col_lbl[j])
    }

    pub fn print_solved(&self) {
        for i in 0..self.rows() {
            let row = self
                .row_at(i)
                .iter()
                .map(|c| match c {
                    Cell::Empty => panic!(),
                    Cell::Crossed => ' ',
                    Cell::Full => 'O',
                })
                .join("");
            info!("{:?}", row);
        }
    }

    pub fn print(&self, level: Level) {
        for i in 0..self.rows() {
            log!(level, "{:?}", self.row_at(i));
        }
    }

    fn is_solved(&self, data: impl Iterator<Item = Cell>, labels: &Labels) -> bool {
        let values: Labels = data
            .group_by(|&e| e == Cell::Full)
            .into_iter()
            .filter(|(k, _)| *k)
            .map(|(_, g)| g.into_iter().count())
            .collect();
        values == *labels
    }
}

// #[cfg(test)]
// mod tests {
//     use super::NonogramSchema;

//     #[test]
//     fn new_ok() {
//         let r = NonogramSchema::new(
//             3,
//             2,
//             vec![vec![1, 2], vec![1, 2], vec![3, 4]],
//             vec![vec![3, 4], vec![3, 4]],
//         );
//         assert!(r.is_ok());
//     }
// }
