// generate all numbers for stops
// generate all rows
// intersect

use std::{collections::VecDeque, iter};

use log::{debug, Level, info};

use crate::schema::{
    Cell::{self, Crossed as X, Empty as N, Full as O},
    NonogramSchema,
};

/// This function can be used to compute the intersection of two rows in a two-dimensional grid
/// of `nonogram::schema::Cell` values, where the intersection is defined as the set of elements
/// that appear in both rows.
///
/// Cells that are different in the two rows are set to `nonogram::Cell::Empty`.
/// It mutates the `row1` slice.
///
/// # Examples
///
/// ```ignore
/// use nonogram::schema::Cell::{Crossed as X, Full as O, Empty as N};
///
/// let row1 = &[X, O, O, X, N];
/// let row2 = &[O, X, O, X, N];
/// let expc = &[N, N, O, X, N];
/// let mut buf = Vec::from(row1);
/// intersect(&mut buf, row2);
/// assert_eq!(buf, expc);
/// ```
pub(crate) fn intersect(row1: &mut [Cell], row2: &[Cell]) {
    for (c, &c2) in row1.iter_mut().zip(row2) {
        if *c != c2 {
            *c = N;
        }
    }
}

pub(crate) fn comply(row1: &[Cell], row2: &[Cell]) -> bool {
    row1.iter()
        .zip(row2)
        .filter(|(&c, &c2)| c != N && c2 != N)
        .all(|(&c, &c2)| c == c2)
}

/// Generates all possible solutions to the provided row information.
///
/// The solutions are generated by taking all possible empty cells between the provided
/// labels, considering a row length of `length`.
///
/// Solutions are returned in a vector of vectors, each one of the vectors representing
/// the empty cells between labels.
///
/// For example, given the labels `[1, 2]` and a length of `5`, the results should be:
/// - `[0, 1, 1]`, representing the row `[O.XX.]`
/// - `[0, 2, 0]`, representing the row `[O..XX]`
/// - `[1, 1, 0]`, representing the row `[.O.XX]`
pub(crate) fn numbers(labels: &[usize], length: usize) -> Vec<Vec<usize>> {
    let len = labels.len() + 1;
    let sum = length - labels.iter().sum::<usize>();

    let mut buf = Vec::with_capacity(100);
    let mut cur = vec![0; len];
    _rec(&mut cur, 0, 0, sum, &mut buf);

    buf
}

fn _rec(cur: &mut [usize], index: usize, cur_sum: usize, sum: usize, buf: &mut Vec<Vec<usize>>) {
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

/// Generates an iterator of `nonogram::schema::Cell`.
/// The cells returned by the iterator are generated by zipping
/// `crossed_nums` and `full_nums`.
///
/// This function panics if `crossed_nums.len() != full_nums.len() + 1`.
///
/// Any number can be zero.
///
/// Crossed values are taken first, and the last is used for generating
/// the last set of values.
///
/// # Examples
///
/// ```ignore
/// use nonogram::schema::Cell::{Crossed as X, Full as O};
/// let crossed_nums = &[1, 2, 3];
/// let full_nums = &[3, 2];
/// let row = numbers_to_vec(crossed_nums, full_nums);
/// assert_eq!(&row, &[X, O, O, O, X, X, O, O, X, X, X]);
///
/// use nonogram::schema::Cell::{Crossed as X, Full as O};
/// let crossed_nums = &[0, 1];
/// let full_nums = &[2];
/// let row = numbers_to_vec(crossed_nums, full_nums);
/// assert_eq!(&row, &[O, X, X, O]);
/// ```
pub(crate) fn numbers_to_vec(crossed_nums: &[usize], full_nums: &[usize]) -> Vec<Cell> {
    if crossed_nums.len() != full_nums.len() + 1 {
        panic!("Invalid");
    }

    let last = *crossed_nums.last().unwrap();

    crossed_nums
        .iter()
        .zip(full_nums)
        .flat_map(|(&empties, &filled)| {
            iter::repeat(X)
                .take(empties)
                .chain(iter::repeat(O).take(filled))
        })
        .chain(iter::repeat(X).take(last))
        .collect()
}

pub fn solve_vec(labels: &[usize], starting_vec: &[Cell]) -> Vec<Cell> {
    let length = starting_vec.len();
    let mut iter = numbers(labels, length).into_iter();
    let all_empty = starting_vec.iter().all(|&c| c == N);

    let cur = if all_empty {
        let mut cur = numbers_to_vec(&iter.next().unwrap(), labels);

        for r in iter {
            let r = numbers_to_vec(&r, labels);
            intersect(&mut cur, &r);
        }

        cur
    } else {
        let mut cur = None;

        for r in iter {
            let r = numbers_to_vec(&r, labels);
            if !comply(&r, starting_vec) {
                continue;
            }
            if cur.is_none() {
                cur = Some(r.clone());
            }
            intersect(cur.as_deref_mut().unwrap(), &r);
        }

        cur.unwrap()
    };

    let mut res = starting_vec.to_vec();

    for (x, y) in res.iter_mut().zip(cur) {
        if *x == N {
            *x = y;
        }
    }

    res
}

trait Solver {
    fn at(&self, schema: &NonogramSchema, i: usize) -> Vec<Cell>;
    fn label_at<'a>(&self, schema: &'a NonogramSchema, i: usize) -> &'a [usize];
    fn set_in_schema(&self, schema: &mut NonogramSchema, i: usize, solved: &[Cell]);
    fn solved(&self, schema: &NonogramSchema, i: usize) -> bool;
    fn enum_variant(&self, i: usize) -> SolverEnum;
}

#[derive(PartialEq, Eq)]
struct RowSolver;

impl Solver for RowSolver {
    fn at(&self, schema: &NonogramSchema, i: usize) -> Vec<Cell> {
        schema.row_at(i)
    }

    fn label_at<'a>(&self, schema: &'a NonogramSchema, i: usize) -> &'a [usize] {
        schema.row_label_at(i)
    }

    fn set_in_schema(&self, schema: &mut NonogramSchema, i: usize, solved: &[Cell]) {
        schema.set_row_at(i, solved);
    }

    fn solved(&self, schema: &NonogramSchema, i: usize) -> bool {
        schema.solved_row(i)
    }

    fn enum_variant(&self, i: usize) -> SolverEnum {
        SolverEnum::Col(i)
    }
}

struct ColSolver;

impl Solver for ColSolver {
    fn at(&self, schema: &NonogramSchema, i: usize) -> Vec<Cell> {
        schema.col_at(i)
    }

    fn label_at<'a>(&self, schema: &'a NonogramSchema, i: usize) -> &'a [usize] {
        schema.col_label_at(i)
    }

    fn set_in_schema(&self, schema: &mut NonogramSchema, i: usize, solved: &[Cell]) {
        schema.set_col_at(i, solved);
    }

    fn solved(&self, schema: &NonogramSchema, i: usize) -> bool {
        schema.solved_col(i)
    }

    fn enum_variant(&self, i: usize) -> SolverEnum {
        SolverEnum::Row(i)
    }
}

fn _solve(
    solver: Box<dyn Solver>,
    schema: &mut NonogramSchema,
    i: usize,
    modified: &mut VecDeque<SolverEnum>,
    bools_solved: &mut [bool],
) {
    if bools_solved[i] {
        return;
    }
    let col = solver.at(schema, i);
    let labels = solver.label_at(schema, i);
    let solved = solve_vec(labels, &col);
    for (i, (a, b)) in col.iter().zip(solved.iter()).enumerate() {
        if *a != *b {
            modified.push_back(solver.enum_variant(i));
        }
    }

    solver.set_in_schema(schema, i, &solved);
    if solver.solved(schema, i) {
        bools_solved[i] = true;
    }
}

#[derive(Debug)]
enum SolverEnum {
    Row(usize),
    Col(usize),
}

pub fn solve(schema: &mut NonogramSchema) {
    let mut rows_solved = vec![false; schema.rows()];
    let mut cols_solved = vec![false; schema.cols()];

    let mut modified = VecDeque::new();

    for i in 0..schema.rows() {
        modified.push_back(SolverEnum::Row(i));
    }

    for j in 0..schema.cols() {
        modified.push_back(SolverEnum::Col(j));
    }

    let mut cnt = 0;

    while !modified.is_empty() {
        let solver_enum = modified.pop_front().unwrap();

        if cnt % 100 == 0 {
            info!("Iteration {}", cnt);
        }
        cnt += 1;
        
        let (bools, solver, i): (&mut [bool], Box<dyn Solver>, usize) = match solver_enum {
            SolverEnum::Row(i) => (&mut rows_solved, Box::new(RowSolver), i),
            SolverEnum::Col(i) => (&mut cols_solved, Box::new(ColSolver), i),
        };
        debug!("Got {:?} -> {:?}", solver_enum, solver.label_at(schema, i));

        _solve(solver, schema, i, &mut modified, bools);
        schema.print(Level::Debug);
    }

    if !rows_solved.iter().all(|v| *v) || !cols_solved.iter().all(|v| *v) {
        panic!("impossible");
    }
}
