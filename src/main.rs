// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]

use harp_pedal_solver::cli::CLI;
use harp_pedal_solver::output::make_ly_file;
use harp_pedal_solver::parse::*;
use harp_pedal_solver::prelude::*;
use harp_pedal_solver::solve::*;
use log::{debug, error, info, warn};
// use std::time::Instant;
use clap_verbosity_flag::Level;
use simple_logger::SimpleLogger;
// use simple_logger::SimpleLogger;
use std::fs;

// Currently silently sets impossible measure to ~~~|~~~~
#[allow(dead_code)]
const IMPOSSIBLE_CHORD: &str = "C
   G G# A
   C
";

fn main() {
    let input = fs::read_to_string(&CLI.file).expect("Unable to read file");
    let show = match CLI.show {
        0 => usize::MAX,
        x => x,
    };
    let log_level = CLI.verbose.log_level_filter();
    SimpleLogger::new()
        .with_level(log_level)
        .without_timestamps()
        .init()
        .unwrap();
    let (start, mid, end) = match parse(&input) {
        Ok(x) => x,
        Err(x) => {
            error!("Error parsing file:\n{x}");
            return;
        }
    };

    debug!(
        "Starting setting: {}",
        match start {
            Some(h) => pedal_diagram(h),
            None => String::from("none"),
        }
    );
    debug!("Music: {mid:?}");
    debug!(
        "Final setting: {}",
        match end {
            Some(h) => pedal_diagram(h),
            None => String::from("none"),
        }
    );

    info!("Checking enharmonic spellings...");
    let (choices, score) = initial_solve(start, &mid, end);

    if CLI.verbose.log_level() >= Some(Level::Debug) {
        debug!("Possible changes, with score {score}:");
        for choice in choices.iter().take(show) {
            debug!("{:?}", pedal_changes(choice));
        }
    }

    // let mut full_music = Vec::new();
    // full_music.push(harp_to_notes(start.unwrap_or([0; 7])));
    // full_music.append(
    //     &mut mid
    //         .clone()
    //         .into_iter()
    //         .flatten()
    //         .collect::<Vec<Vec<Note>>>(),
    // );
    // full_music.push(harp_to_notes(end.unwrap_or([0; 7])));

    info!("Breaking up simultaneous pedal changes...");
    let mut solutions = solve(start, &mid, end);
    solutions.sort_by(|x, y| x.1.cmp(&y.1));

    if !solutions.is_empty() {
        println!("Possible solutions:\n");
        for s in solutions.iter().take(show) {
            println!("{s:?}");
        }
        println!(
            "\nand {} other possibilities.",
            solutions.len().saturating_sub(show)
        );
    } else if !choices.is_empty() {
        println!("Found possible solutions, but could not avoid simultaneous pedal changes.\n");
        for choice in choices.iter().take(show) {
            println!("{:?}\n", pedal_changes(choice));
        }
        println!(
            "\nand {} other possibilities.",
            choices.len().saturating_sub(show)
        );
    } else {
        println!("Could not find any solutions.");
    }

    println!(
        "{}",
        make_ly_file(mid, start.unwrap(), end.unwrap(), solutions[0].0.clone())
    );
}
