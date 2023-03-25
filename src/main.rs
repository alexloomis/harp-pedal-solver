// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]

use harp_pedal_solver::base::*;
use harp_pedal_solver::parse::*;
// use harp_pedal_solver::shift::*;
use harp_pedal_solver::solve::*;
use std::time::Instant;

#[allow(dead_code)]
const AQUARIUM: &str = "^^^|^^-^
   A C Eb
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
const FIREMUSIC: &str =
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

// Currently silently sets impossible measure to ~~~|~~~~
#[allow(dead_code)]
const IMPOSSIBLE_CHORD: &str = "C
   G G# A
   C
";

fn main() {
    let parsed = parse(FIREMUSIC);
    println!("Input:\n{parsed:?}\n");
    let now = Instant::now();
    // paths: Vec<Vec<Harp>>
    let (paths, score) = solve(&parsed);
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
