// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]
use clap_verbosity_flag::LevelFilter;
use itertools::Itertools;
use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
// use std::time::Instant;

use harp_pedal_solver::cli::CLI;
use harp_pedal_solver::output::make_ly_file;
use harp_pedal_solver::parse::*;
use harp_pedal_solver::prelude::*;
use harp_pedal_solver::solve::*;

// Currently silently sets impossible measure to ~~~|~~~~
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
    let output = CLI
        .output
        .clone()
        .unwrap_or_else(|| PathBuf::from("pedals"));
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

    if log_level >= LevelFilter::Debug {
        debug!("Possible changes, with score {score}:");
        for choice in choices.iter().take(show) {
            debug!("{:?}", pedal_changes(choice));
        }
    }

    info!("Breaking up simultaneous pedal changes...");
    let mut solutions = solve(start, &mid, end);
    solutions.sort_by(|x, y| x.0.cmp(&y.0));

    if !solutions.is_empty() {
        if log_level >= LevelFilter::Info {
            info!("Possible solutions:");
            for s in solutions.iter().take(show) {
                println!("{s:?}");
            }
            info!(
                "and {} other possibilities.",
                solutions.len().saturating_sub(show)
            );
        }
    } else if !choices.is_empty() {
        if log_level >= LevelFilter::Warn {
            warn!("Found possible solutions, but could not avoid simultaneous pedal changes.");
            for choice in choices.iter().take(show) {
                println!("{:?}", pedal_changes(choice));
            }
            info!(
                "and {} other possibilities.",
                choices.len().saturating_sub(show)
            );
        }
    } else {
        error!("Could not find any solutions.");
        return;
    }

    let decision = if !solutions.is_empty() {
        solutions[0].to_owned().1
    } else {
        process_choice(&choices[0])
    };

    // Currently sometimes wrong if there are few changes
    // and few initial settins given.
    // Caused by using notes_to_harp instead of enharmonics.
    let mut start_diagram =
        full_initial(&decision.iter().map(|v| notes_to_harp(v)).collect_vec());
    // Take unset pedals from end.
    if let Some(d) = end {
        start_diagram = update_harp(d, start_diagram)
    }
    // Apply to initially unset pedals.
    if let Some(d) = start {
        start_diagram = update_harp(start_diagram, d)
    }
    let end_diagram = match start {
        Some(d) => update_harp_notes(update_harp([1; 7], d), &decision),
        None => update_harp_notes([1; 7], &decision),
    };

    let ly_file = make_ly_file(mid, start_diagram, end_diagram, decision);

    let mut ly_command = Command::new("lilypond")
        .args([
            "-l",
            &log_level.to_string(),
            "-o",
            &output.to_string_lossy(),
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    ly_command
        .stdin
        .as_mut()
        .unwrap()
        .write_all(ly_file.as_bytes())
        .unwrap();
    ly_command.wait().unwrap();
}
