use array2d::Array2D;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Full,
    Crossed,
}

pub enum Error {
    InvalidLabel(String),
}

pub type Labels = Vec<usize>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct NonogramSchema {
    data: Array2D<Cell>,
    rows_labels: Vec<Labels>,
    cols_labels: Vec<Labels>,
}

fn _validate(rows: usize, rows_labels: &Vec<Labels>, cols: usize) -> Result<()> {
    if rows_labels.len() != rows {
        let msg = format!(
            "Invalid label size, expected {}, found {}",
            rows,
            rows_labels.len()
        );
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
        rows_labels: Vec<Labels>,
        cols_labels: Vec<Labels>,
    ) -> Result<Self> {
        _validate(rows, &rows_labels, cols)?;
        _validate(cols, &cols_labels, rows)?;

        let schema = NonogramSchema {
            data: Array2D::filled_with(Cell::Empty, rows, cols),
            rows_labels,
            cols_labels,
        };
        Ok(schema)
    }

    pub fn rows(&self) -> usize {
        self.data.num_rows()
    }

    pub fn cols(&self) -> usize {
        self.data.num_columns()
    }

    pub fn solved(&self) -> bool {
        let res = self
            .data
            .rows_iter()
            .zip(&self.rows_labels)
            .all(|(data, labels)| self.is_solved(data.copied(), labels));

        if !res {
            return false;
        }

        let res = self
            .data
            .columns_iter()
            .zip(&self.cols_labels)
            .all(|(data, labels)| self.is_solved(data.copied(), labels));

        res
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

#[cfg(test)]
mod tests {
    use super::NonogramSchema;

    #[test]
    fn new_ok() {
        let r = NonogramSchema::new(
            3,
            2,
            vec![vec![1, 2], vec![1, 2], vec![3, 4]],
            vec![vec![3, 4], vec![3, 4]],
        );
        assert!(r.is_ok());
    }
}
