// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]

use harp_pedal_solver::base::*;
use harp_pedal_solver::parse::*;
use harp_pedal_solver::shift::*;
use harp_pedal_solver::solve::*;
// use std::time::Instant;

#[allow(dead_code)]
const AQUARIUM: &str = "^^^|^^-^ A C Eb
    F
    Bb Db G |
    Eb 
    Ab B F
    Db
    A Eb Gb |
    D
    D E G
    A
    B D F |
    Bb
    A C Eb
    A
    ~^^|^~~^
";

#[allow(dead_code)]
const FIRE_MUSIC: &str =
    "$pedal diagram symbols: ^ flat, - natural, v sharp, ~ no preference
^-^|^-^^
        Ab C Eb
        Cb Eb Gb
        D F Bb
        Eb G C A |
        Ab Fb Cb
        G Eb Bb
        Gb Eb A Bb
        F D Ab C |
        G E C
        G Eb Bb
        Gb D A
        Fb Db Bb G |
        Ab C Eb
        Eb Cb Gb
        Bb F D
        D# Gb A C |
        E G# B
        vv-|-vv-
        ";

#[allow(dead_code)]
const FIRE_MUSIC_EIGHTHS: &str =
    "$pedal diagram symbols: ^ flat, - natural, v sharp, ~ no preference
^-^|^-^^
        Ab C Eb
        Ab C Eb
        Cb Eb Gb
        Cb Eb Gb
        D F Bb
        D F Bb
        Eb G C A
        Eb G C A |
        Ab Fb Cb
        Ab Fb Cb
        G Eb Bb
        G Eb Bb
        Gb Eb A Bb
        Gb Eb A Bb
        F D Ab C
        F D Ab C |
        G E C
        G E C
        G Eb Bb
        G Eb Bb
        Gb D A
        Gb D A
        Fb Db Bb G
        Fb Db Bb G |
        Ab C Eb
        Ab C Eb
        Eb Cb Gb
        Eb Cb Gb
        Bb F D
        Bb F D
        D# Gb A C
        D# Gb A C |
        E G# B
        E G# B vv-|-vv-
        ";

// Currently silently sets impossible measure to ~~~|~~~~
#[allow(dead_code)]
const IMPOSSIBLE_CHORD: &str = "C
   G G# A
   C
";

#[allow(dead_code)]
const EASY: &str = "C | C# | D | ";

fn main() {
    let (start, mid, end) = parse(AQUARIUM);
    println!("Starting setting:\n{start:?}\n");
    println!("Music:\n{mid:?}\n");
    println!("Final setting:\n{end:?}\n");

    let (choices, _score) = initial_solve(start, &mid, end);
    println!("First pass:");
    for choice in choices.iter() {
        println!("{choice:?}");
    }

    println!("\nChanges:");
    for choice in choices.iter() {
        println!("{:?}\n", initial_pedal_changes(choice));
    }

    let mut full_music = Vec::new();
    full_music.push(harp_to_notes(start.unwrap_or([0; 7])));
    // full_music = vec![];
    full_music.append(
        &mut mid
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Vec<Note>>>(),
    );
    full_music.push(harp_to_notes(end.unwrap_or([0; 7])));

    println!("\nShifted changes:");
    for choice in choices.iter() {
        println!(
            "Shifting\n{:?}\nwith music\n{:?}\n",
            &choice[1..],
            full_music
        );
        println!("{:?}", shifted_changes(&full_music, &choice));
    }

    println!("\nScored changes:");
    for choice in choices.iter() {
        println!("{:?}", scored_changes(&full_music, &choice));
    }

    println!("\nSolutions:");
    for s in solve(start, &mid, end) {
        println!("{s:?}");
    }
}

// fn main() {
//     let music: Vec<Vec<Note>> = vec![vec!["F", "G"], vec!["A"], vec!["Gb"]]
//         .iter()
//         .map(|v| v.iter().map(|n| read_note(n)).collect())
//         .collect();
//     let pedals: Vec<Vec<Note>> = vec![vec![], vec!["A"], vec!["F"]]
//         .iter()
//         .map(|v| v.iter().map(|n| read_note(n)).collect())
//         .collect();
//     let input: Vec<(Vec<Note>, Vec<Note>)> =
//         music.into_iter().zip(pedals).collect();
//     let shifted = change_builder(&input, &[]);
//     println!("{:?}", unravel_paths(shifted));
// }
