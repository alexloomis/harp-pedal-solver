// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]

use harp_pedal_solver::assign::*;
use harp_pedal_solver::base::*;
use harp_pedal_solver::transition::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let initial_state = notes_to_harp(
        &vec!["Ab", "Bb", "Cb", "Db", "Eb", "Fb", "G"]
            .iter()
            .map(|n| read_note(n))
            .collect::<Vec<Note>>(),
    );
    let end_state = notes_to_harp(
        &vec!["Ab", "Bb", "Cb", "Eb"]
            .iter()
            .map(|n| read_note(n))
            .collect::<Vec<Note>>(),
    );
    let music: Vec<Vec<PitchClass>> = vec![
        vec!["A", "C", "Eb"],
        vec!["F"],
        vec!["Bb", "Db", "G"],
        vec!["Eb"],
        vec!["Ab", "B", "F"],
        vec!["Db"],
        vec!["A", "Eb", "Gb"],
        vec!["D"],
        vec!["D", "E", "G"],
        vec!["A"],
        vec!["B", "D", "F"],
        vec!["Bb"],
        vec!["A", "C", "Eb"],
        vec!["A"],
    ]
    .iter()
    .map(|m| m.iter().map(|n| note_to_pc(read_note(n))).collect())
    .collect();
    let medial_states =
        music.iter().map(|p| assign(p)).collect::<Vec<Vec<Harp>>>();
    // paths: Vec<Vec<Harp>>
    let (paths, score) = find_paths(initial_state, &medial_states, end_state);
    let pretty: Vec<Vec<Vec<Note>>> = paths
        .iter()
        .map(|v| unset_seen(v).iter().map(|h| harp_to_notes(*h)).collect())
        .collect();
    let p_len = pretty.len();
    for p in pretty {
        println!("Found:");
        println!("{:?}\n", &p[1..]);
    }
    println!("Found {p_len} ways with a cost of {score}.\n");
    println!("Elapsed time: {:.2?}", now.elapsed());
}
