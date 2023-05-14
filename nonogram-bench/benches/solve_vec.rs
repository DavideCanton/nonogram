use criterion::{Criterion, black_box};
use nonogram::{schema::Cell, solver::solve_vec};

#[inline(always)]
fn run(len: usize, labels: &[usize]) {
    let starting_row = vec![Cell::Empty; len];
    solve_vec(black_box(labels), black_box(&starting_row));
}

pub fn solve_vec_bench(c: &mut Criterion) {
    c.bench_function("solve_vec_easy", |b| {
        b.iter(|| {
            run(15, &[7, 7]);
        })
    });

    c.bench_function("solve_vec_no", |b| {
        b.iter(|| {
            run(15, &[1; 8]);
        })
    });
}
