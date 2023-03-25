use harp_pedal_solver::assign::*;
use harp_pedal_solver::base::*;
use harp_pedal_solver::shift::*;
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

#[test]
fn correct_num_assignations() {
    assert_eq!(10, assign(&[0, 3, 5, 7, 9]).len());
}

#[test]
fn chain_pushes_correctly() {
    let music: Vec<Vec<Note>> = vec![vec!["F", "G"], vec!["A"], vec!["Gb"]]
        .iter()
        .map(|v| v.iter().map(|n| read_note(n)).collect())
        .collect();
    let pedals: Vec<Vec<Note>> = vec![vec!["F", "G"], vec!["A"], vec![]]
        .iter()
        .map(|v| v.iter().map(|n| read_note(n)).collect())
        .collect();
    let input: Vec<(Vec<Note>, Vec<Note>)> =
        music.into_iter().zip(pedals).collect();
    let shifted = unravel_paths(change_builder(&input, &[]));
    assert_eq!(3, shifted.len());
}

#[test]
fn abandon_when_too_many() {
    let music: Vec<Vec<Note>> = vec![vec!["F", "G"], vec!["A"], vec!["Gb"]]
        .iter()
        .map(|v| v.iter().map(|n| read_note(n)).collect())
        .collect();
    let pedals: Vec<Vec<Note>> = vec![vec!["F", "G"], vec!["A"], vec!["C"]]
        .iter()
        .map(|v| v.iter().map(|n| read_note(n)).collect())
        .collect();
    let input: Vec<(Vec<Note>, Vec<Note>)> =
        music.into_iter().zip(pedals).collect();
    let shifted = unravel_paths(change_builder(&input, &[]));
    assert_eq!(0, shifted.len());
}

#[test]
fn abandon_when_impossible() {
    let music: Vec<Vec<Note>> =
        vec![vec!["F", "G"], vec!["F#", "G#"], vec!["Gb"]]
            .iter()
            .map(|v| v.iter().map(|n| read_note(n)).collect())
            .collect();
    let pedals: Vec<Vec<Note>> = vec![vec!["F", "G"], vec![], vec![]]
        .iter()
        .map(|v| v.iter().map(|n| read_note(n)).collect())
        .collect();
    let input: Vec<(Vec<Note>, Vec<Note>)> =
        music.into_iter().zip(pedals).collect();
    let shifted = unravel_paths(change_builder(&input, &[]));
    assert_eq!(0, shifted.len());
}

#[test]
fn can_have_none() {
    let music: Vec<Vec<Note>> =
        vec![vec!["F", "G"], vec!["F#", "G#"], vec!["Gb"]]
            .iter()
            .map(|v| v.iter().map(|n| read_note(n)).collect())
            .collect();
    let pedals: Vec<Vec<Note>> = vec![vec![], vec!["A"], vec!["F"]]
        .iter()
        .map(|v| v.iter().map(|n| read_note(n)).collect())
        .collect();
    let input: Vec<(Vec<Note>, Vec<Note>)> =
        music.into_iter().zip(pedals).collect();
    let shifted = unravel_paths(change_builder(&input, &[]));
    assert!(shifted[0][2].is_none());
}
