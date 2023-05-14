// generate all numbers for stops
// generate all rows
// intersect

use std::iter;

use crate::schema::{Cell, Label};

pub(crate) fn intersect(row1: &mut [Cell], row2: impl Iterator<Item = Cell>) {
    for (c, c2) in row1.iter_mut().zip(row2) {
        if *c != c2 {
            *c = Cell::Empty;
        }
    }
}

pub(crate) fn numbers(labels: &[Label], length: usize) -> impl Iterator<Item = Vec<Label>> {
    let len = labels.len() + 1;
    let sum = length - labels.iter().sum::<usize>();

    let mut buf = Vec::with_capacity(100);
    let mut cur = vec![0; len];
    _rec(&mut cur, 0, 0, sum, &mut buf);

    buf.into_iter()
}

fn _rec(cur: &mut [Label], index: Label, cur_sum: Label, sum: Label, buf: &mut Vec<Vec<usize>>) {
    let last = cur.len() - 1;
    if index > last {
        if cur_sum == sum {
            buf.push(cur.to_vec());
        }
        return;
    }

    let inf = if index == 0 || index == last { 0 } else { 1 };
    let sup = sum; // TODO improve

    for i in inf..=sup {
        cur[index] = i;
        _rec(cur, index + 1, cur_sum + i, sum, buf);
    }
}

pub(crate) fn numbers_to_row<'a>(
    voids: &'a [Label],
    labels: &'a [Label],
) -> Box<dyn Iterator<Item = Cell> + 'a> {
    if voids.len() != labels.len() + 1 {
        panic!("Invalid");
    }

    let last = *voids.last().unwrap();

    Box::new(
        voids
            .iter()
            .zip(labels)
            .flat_map(|(&empties, &filled)| {
                iter::repeat(Cell::Crossed)
                    .take(empties)
                    .chain(iter::repeat(Cell::Full).take(filled))
            })
            .chain(iter::repeat(Cell::Crossed).take(last)),
    )
}

fn get_row(labels: &[Label], starting_row: Vec<Cell>) -> Vec<Cell> {
    let length = starting_row.len();
    let mut iter = numbers(labels, length);

    let mut cur = if starting_row.iter().all(|&c| c == Cell::Empty) {
        numbers_to_row(&iter.next().unwrap(), labels).collect()
    } else {
        starting_row
    };

    for r in iter {
        let r = numbers_to_row(&r, labels);
        intersect(&mut cur, r.into_iter());
    }

    cur
}
