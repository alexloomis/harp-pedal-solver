// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]
use harp_pedal_solver::candidate::find_candidates;
use itertools::Itertools;
use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
// use std::time::Instant;

use harp_pedal_solver::cli::CONST;
use harp_pedal_solver::lilypond::make_ly_file_;
use harp_pedal_solver::parse::*;
use harp_pedal_solver::prelude::*;

// Currently silently sets impossible measure to ~~~|~~~~
fn main() -> ExitCode {
    let input = fs::read_to_string(&CONST.file).expect("Unable to read file");
    // let _show = match CONST.show {
    //     0 => usize::MAX,
    //     x => x,
    // };
    let log_level = CONST.verbose.log_level_filter();
    SimpleLogger::new()
        .with_level(log_level)
        .without_timestamps()
        .init()
        .unwrap();
    let output = CONST
        .output
        .clone()
        .unwrap_or_else(|| PathBuf::from("pedals"));
    let parsed = match parse(&input) {
        Ok(x) => x,
        Err(x) => {
            error!("Error parsing file:\n{x}");
            return ExitCode::FAILURE;
        }
    };

    debug!(
        "Starting setting: {}",
        pedal_diagram(parsed.start.unwrap_or([None; 7]))
    );
    debug!("Music: {:?}", parsed.this_any);
    debug!(
        "Final setting: {}",
        pedal_diagram(parsed.end.unwrap_or([None; 7]))
    );

    let mut measures = Vec::with_capacity(parsed.this_any.len());
    let measure_lengths = parsed.this_any.clone().into_iter().map(|v| v.len());

    let music_input = MusicInput {
        diagram: parsed.start.unwrap_or([None; 7]),
        music: parsed.this_any.into_iter().flatten().collect_vec(),
        goal: parsed.end.unwrap_or([None; 7]),
    };

    let candidates = find_candidates(&music_input);
    // candidates.sort_by(|x, y| x.cost.cmp(&y.cost));

    if !candidates.is_empty() {
        info!("Found {} possibilities...", candidates.len());
    } else {
        error!("Could not find any solutions.");
        return ExitCode::FAILURE;
    }

    let decision = &candidates[0];
    let spell = &decision.spelling;
    let mut j = 0;
    for n in measure_lengths {
        let mut measure = Vec::with_capacity(n);
        for _ in 0..n {
            measure.push(harp_to_notes(spell[j]));
            j += 1;
        }
        measures.push(measure);
    }

    let ly_file = make_ly_file_(
        measures,
        decision.diagram,
        decision.destination,
        &decision.pedals,
        decision.cost,
    );

    debug!("{ly_file}");

    fs::write("temp.ly", ly_file).expect("./ does not exist");

    let ly_command = Command::new("lilypond")
        .args([
            "-l",
            &log_level.to_string(),
            "-o",
            &output.to_string_lossy(),
            // "-E", // EPS, crops output
            "temp.ly",
        ])
        .status();

    match ly_command {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
