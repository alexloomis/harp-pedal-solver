// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]

use harp_pedal_solver::cli::CLI;
use harp_pedal_solver::parse::*;
use harp_pedal_solver::prelude::*;
use harp_pedal_solver::solve::*;
// use std::time::Instant;
use std::fs;

// Currently silently sets impossible measure to ~~~|~~~~
#[allow(dead_code)]
const IMPOSSIBLE_CHORD: &str = "C
   G G# A
   C
";

fn main() {
    let input = fs::read_to_string(&CLI.file).expect("Unable to read file");
    let verbose = CLI.verbose || CLI.debug;
    let debug = CLI.debug;
    let show = match CLI.show {
        0 => usize::MAX,
        x => x,
    };

    let (start, mid, end) = parse(&input);
    if verbose {
        println!(
            "Starting setting:\n{}\n",
            match start {
                Some(h) => pedal_diagram(h),
                None => String::from("none"),
            }
        );
        println!("Music:\n{mid:?}\n");
        println!(
            "Final setting:\n{}\n",
            match end {
                Some(h) => pedal_diagram(h),
                None => String::from("none"),
            }
        );
    }

    println!("Checking enharmonic spellings...");
    let (choices, _score) = initial_solve(start, &mid, end);

    if debug {
        println!("\nPossible changes:");
        for choice in choices.iter().take(show) {
            println!("{:?}\n", pedal_changes(choice));
        }
    }

    let mut full_music = Vec::new();
    full_music.push(harp_to_notes(start.unwrap_or([0; 7])));
    full_music.append(
        &mut mid
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Vec<Note>>>(),
    );
    full_music.push(harp_to_notes(end.unwrap_or([0; 7])));

    println!("Breaking up simultaneous pedal changes...");
    let solutions = solve(start, &mid, end);

    if !solutions.is_empty() {
        println!("Possible solutions:\n");
        for s in solutions.iter().take(show) {
            println!("{s:?}");
        }
        println!(
            "and {} other possibilities.",
            solutions.len().saturating_sub(show)
        );
    } else if !choices.is_empty() {
        println!("Found possible solutions, but could not avoid simultaneous pedal changes.\n");
        for choice in choices.iter().take(show) {
            println!("{:?}\n", pedal_changes(choice));
        }
        println!(
            "and {} other possibilities.",
            choices.len().saturating_sub(show)
        );
    } else {
        println!("Could not find any solutions.");
    }
}
