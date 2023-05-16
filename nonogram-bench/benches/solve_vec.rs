use criterion::{black_box, Criterion};
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

    c.bench_function("slow", |b| {
        b.iter(|| {
            run(38, &[1, 2, 1, 1, 2, 2, 4, 5, 9]);
        })
    });
}
