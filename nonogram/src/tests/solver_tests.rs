use crate::schema::{Cell::Crossed as X, Cell::Empty as N, Cell::Full as O};

mod number_tests {
    use std::collections::HashSet;

    use crate::solver::numbers;

    #[test]
    fn test_1() {
        let r: HashSet<_> = HashSet::from_iter(numbers(&[1, 2], 5).into_iter());

        assert_eq!(r.len(), 3);
        assert!(r.contains(&vec![0, 1, 1]));
        assert!(r.contains(&vec![0, 2, 0]));
        assert!(r.contains(&vec![1, 1, 0]));
    }

    #[test]
    fn test_2() {
        let r: HashSet<_> = HashSet::from_iter(numbers(&[1, 2], 7).into_iter());

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
        let r: HashSet<_> = HashSet::from_iter(numbers(&[1], 7).into_iter());

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
        let r: HashSet<_> = HashSet::from_iter(numbers(&[1, 1, 1], 7).into_iter());

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
    use super::{N, O, X};
    use crate::{schema::Cell, solver::intersect};
    use test_case::test_case;

    #[test_case(
        &[N,N,X,O,O,X,N,O,X,N],
        &[N,X,N,O,X,X,O,X,O,N],
        &[N,N,N,O,N,X,N,N,N,N];
        "case 1"
    )]
    #[test_case(
        &[N,N,X,O,O,X,N,O,X,N],
        &[N,N,N,O,O,X,N,X,O,N],
        &[N,N,N,O,O,X,N,N,N,N];
        "case 2"
    )]
    fn test(row1: &[Cell], row2: &[Cell], expected: &[Cell]) {
        let mut buf = Vec::from(row1);
        intersect(&mut buf, row2.iter().copied());
        assert_eq!(buf, expected);
    }
}

mod numbers_to_row_tests {
    use super::{O, X};
    use crate::{schema::Cell, solver::numbers_to_vec};
    use test_case::test_case;

    #[test_case(
        &[1, 2, 3],
        &[2, 4],
        &[X,O,O,X,X,O,O,O,O,X,X,X,];
        "case 1"
    )]
    #[test_case(
        &[0, 2, 3],
        &[2, 4],
        &[O,O,X,X,O,O,O,O,X,X,X,];
        "case 2"
    )]
    #[test_case(
        &[1, 2],
        &[2],
        &[X,O,O,X,X];
        "case 3"
    )]
    #[test_case(
        &[0, 0],
        &[3],
        &[O,O,O];
        "case empty"
    )]
    fn test(voids: &[usize], labels: &[usize], expected: &[Cell]) {
        let res: Vec<_> = numbers_to_vec(voids, labels).collect();
        assert_eq!(&res, expected);
    }

    #[test_case(&[1], &[2, 4]; "case 1")]
    #[test_case(&[1, 2, 3, 4], &[2, 4]; "case 2")]
    #[test_case(&[], &[2, 4]; "case 3")]
    #[test_case(&[1, 2], &[]; "case 4")]
    #[should_panic]
    fn test_panic(voids: &[usize], labels: &[usize]) {
        let _ = numbers_to_vec(voids, labels);
    }
}

mod get_row_tests {
    use super::{N, O, X};
    use crate::solver::solve_vec;

    #[test]
    pub fn test_get_row_1() {
        let starting_row = vec![N; 15];
        let labels = [7, 7];
        let res = solve_vec(&labels, starting_row);

        let mut exp = vec![O; 15];
        exp[7] = X;
        assert_eq!(res, exp);
    }

    #[test]
    pub fn test_get_row_2() {
        let starting_row = vec![N; 15];
        let labels = [8];
        let res = solve_vec(&labels, starting_row);

        let mut exp = vec![N; 15];
        exp[7] = O;
        assert_eq!(res, exp);
    }
}
