use criterion::Criterion;
use nonogram::{schema::Cell, solver::solve_vec};

pub fn solve_vec_bench(c: &mut Criterion) {
    c.bench_function("solve_vec_easy", |b| {
        b.iter(|| {
            let starting_row = vec![Cell::Empty; 15];
            let labels = [7, 7];
            solve_vec(&labels, &starting_row);
        })
    });

    c.bench_function("solve_vec_no", |b| {
        b.iter(|| {
            let starting_row = vec![Cell::Empty; 15];
            let labels = [1; 8];
            solve_vec(&labels, &starting_row);
        })
    });
}
