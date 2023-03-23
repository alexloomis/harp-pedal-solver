use harp_pedal_solver::assign::*;
use harp_pedal_solver::util::*;
use trees::*;

#[test]
fn unravel_correctly() {
    let mut forest = Forest::<usize>::new();
    forest.push_back(tr(0) / tr(1));
    forest.push_back(tr(2) / tr(3) / (tr(4) / tr(5) / tr(6)));
    assert_eq!(
        unravel_paths(forest),
        vec![vec![1, 0], vec![3, 2], vec![5, 4, 2], vec![6, 4, 2]]
    );
}

#[test]
fn can_assign_empty() {
    assert!(can_assign(&[]));
}

#[test]
fn can_assign_nonempty() {
    assert!(can_assign(&[0, 3, 5, 7, 9]));
}

#[test]
fn cant_assign() {
    assert!(!can_assign(&[11, 0, 1]));
}

// #[test]
// fn can_assign_1() {
//     assert!(can_assign(&[1, 2, 4, 5, 7, 9, 11]));
// }
//
// #[test]
// fn can_assign_2() {
//     assert!(can_assign(&[0, 3, 5, 6, 7, 9, 10]));
// }
//
// #[test]
// fn can_assign_3() {
//     assert!(!can_assign(&[0, 1, 3, 5, 6, 7, 9, 10]));
// }

#[test]
fn correct_num_assignations() {
    assert_eq!(10, assign(&[0, 3, 5, 7, 9]).len());
}
