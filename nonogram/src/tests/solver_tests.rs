use crate::schema::Cell;

mod number_tests {
    use std::collections::HashSet;

    use crate::solver::numbers;

    #[test]
    fn test_1() {
        let r: HashSet<_> = numbers(&[1, 2], 5).collect();

        assert_eq!(r.len(), 3);
        assert!(r.contains(&vec![0, 1, 1]));
        assert!(r.contains(&vec![0, 2, 0]));
        assert!(r.contains(&vec![1, 1, 0]));
    }

    #[test]
    fn test_2() {
        let r: HashSet<_> = numbers(&[1, 2], 7).collect();

        assert_eq!(r.len(), 10);

        assert!(r.contains(&vec![0, 1, 3]));
        assert!(r.contains(&vec![0, 2, 2]));
        assert!(r.contains(&vec![0, 3, 1]));
        assert!(r.contains(&vec![0, 4, 0]));
        assert!(r.contains(&vec![1, 1, 2]));
        assert!(r.contains(&vec![1, 2, 1]));
        assert!(r.contains(&vec![1, 3, 0]));
        assert!(r.contains(&vec![2, 1, 1]));
        assert!(r.contains(&vec![2, 2, 0]));
        assert!(r.contains(&vec![3, 1, 0]));
    }

    #[test]
    fn test_3() {
        let r: HashSet<_> = numbers(&[1], 7).collect();

        assert_eq!(r.len(), 7);

        for i in 0..=6 {
            assert!(r.contains(&vec![i, 6 - i]));
        }
    }

    #[test]
    fn test_4() {
        // 0 1 1 2
        // 0 2 1 1
        // 0 1 2 1
        // 0 3 1 0
        // 0 1 3 0
        // 0 2 2 0
        // 1 1 1 1
        // 1 2 1 0
        // 1 1 2 0
        // 2 1 1 0
        let r: HashSet<_> = numbers(&[1, 1, 1], 7).collect();

        assert_eq!(r.len(), 10);

        assert!(r.contains(&vec![0, 1, 1, 2]));
        assert!(r.contains(&vec![0, 2, 1, 1]));
        assert!(r.contains(&vec![0, 1, 2, 1]));
        assert!(r.contains(&vec![0, 3, 1, 0]));
        assert!(r.contains(&vec![0, 1, 3, 0]));
        assert!(r.contains(&vec![0, 2, 2, 0]));
        assert!(r.contains(&vec![1, 1, 1, 1]));
        assert!(r.contains(&vec![1, 2, 1, 0]));
        assert!(r.contains(&vec![1, 1, 2, 0]));
        assert!(r.contains(&vec![2, 1, 1, 0]));
    }
}

mod intersect_tests {
    use super::{from_cell, to_cell};
    use crate::solver::intersect;
    use itertools::Itertools;
    use test_case::test_case;

    #[test_case(
        "..XOOX.OX.", 
        ".X.OXXOXO.",
        "...O.X...."; 
        "case 1"
    )]
    #[test_case(
        "..XOOX.OX.", 
        "...OOX.XO.",
        "...OOX...."; 
        "case 2"
    )]
    fn test(row1: &'static str, row2: &'static str, expected: &'static str) {
        let mut row1 = row1.chars().map(to_cell).collect_vec();
        let row2 = row2.chars().map(to_cell).collect_vec();

        intersect(&mut row1, row2.into_iter());

        let res: String = row1.into_iter().map(from_cell).collect();
        assert_eq!(&res, expected);
    }
}

mod numbers_to_row_tests {
    use super::from_cell;
    use test_case::test_case;

    use crate::{schema::Label, solver::numbers_to_row};

    #[test_case(
        &[1, 2, 3], 
        &[2, 4],
        "XOOXXOOOOXXX"; 
        "case 1"
    )]
    #[test_case(
        &[0, 2, 3], 
        &[2, 4],
        "OOXXOOOOXXX"; 
        "case 2"
    )]
    #[test_case(
        &[1, 2], 
        &[2],
        "XOOXX"; 
        "case 3"
    )]
    #[test_case(
        &[0, 0], 
        &[3],
        "OOO"; 
        "case empty"
    )]
    fn test(voids: &[Label], labels: &[Label], expected: &'static str) {
        let res: String = numbers_to_row(voids, labels).map(from_cell).collect();
        assert_eq!(&res, expected);
    }

    #[test_case(&[1], &[2, 4]; "case 1")]
    #[test_case(&[1, 2, 3, 4], &[2, 4]; "case 2")]
    #[test_case(&[], &[2, 4]; "case 3")]
    #[test_case(&[1, 2], &[]; "case 4")]
    #[should_panic]
    fn test_panic(voids: &[Label], labels: &[Label]) {
        let _ = numbers_to_row(voids, labels);
    }
}

fn to_cell(c: char) -> Cell {
    match c {
        'X' => Cell::Crossed,
        'O' => Cell::Full,
        _ => Cell::Empty,
    }
}

fn from_cell(c: Cell) -> char {
    match c {
        Cell::Crossed => 'X',
        Cell::Full => 'O',
        Cell::Empty => '.',
    }
}
