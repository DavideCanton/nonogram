use criterion::{criterion_group, criterion_main};

mod solve_vec;

criterion_group!(benches, solve_vec::solve_vec_bench);
criterion_main!(benches);
