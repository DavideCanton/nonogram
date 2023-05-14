use criterion::{criterion_group, criterion_main, Criterion};
use nonogram::{schema::Cell, solver::solve_vec};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get_row", |b| {
        b.iter(|| {
            let starting_row = vec![Cell::Empty; 15];
            let labels = [7, 7];
            solve_vec(&labels, starting_row);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
