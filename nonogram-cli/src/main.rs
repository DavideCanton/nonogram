use nonogram::{schema::NonogramSchema, solver::solve};

fn main() {
    let rows_labels = vec![
        vec![11],
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
        vec![13],
        vec![3, 8, 2],
        vec![11, 3],
        vec![2, 3, 2, 3],
        vec![4, 1, 3],
        vec![3, 1, 4],
        vec![2, 1, 3],
        vec![3, 3, 2],
        vec![5, 5],
        vec![9],
        vec![5],
    ];

    let cols_labels = vec![
        vec![5],
        vec![9],
        vec![1, 1, 3, 5],
        vec![2, 1, 3, 3],
        vec![1, 4, 1, 2],
        vec![1, 4, 3],
        vec![1, 3, 3],
        vec![1, 3, 2],
        vec![1, 4, 4],
        vec![1, 5, 5],
        vec![1, 3, 3],
        vec![2, 2, 2, 2],
        vec![1, 1, 1, 7],
        vec![9],
        vec![5],
    ];

    let mut schema = NonogramSchema::new(15, 15, rows_labels, cols_labels).unwrap();
    solve(&mut schema);
    schema.print();
}
