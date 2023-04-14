// #![allow(dead_code)]
#![warn(clippy::needless_pass_by_value)]
use harp_pedal_solver::candidate::find_candidates;
use itertools::Itertools;
use log::{debug, error, info, warn};
use simple_logger::SimpleLogger;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
// use std::time::Instant;

use harp_pedal_solver::cli::CLI;
use harp_pedal_solver::output::make_ly_file_;
use harp_pedal_solver::parse::*;
use harp_pedal_solver::prelude::*;

// Currently silently sets impossible measure to ~~~|~~~~
fn main() {
    let input = fs::read_to_string(&CLI.file).expect("Unable to read file");
    let _show = match CLI.show {
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

    let measure_lengths = mid.iter().map(|v| v.len());

    let music_input = MusicInput {
        diagram: start.unwrap_or([None; 7]),
        music: mid
            .clone()
            .into_iter()
            .flatten()
            .map(|v| v.into_iter().map(note_to_pc).collect_vec())
            .collect_vec(),
        goal: end.unwrap_or([None; 7]),
    };

    let mut candidates = find_candidates(&music_input);
    candidates.sort_by(|x, y| x.cost.cmp(&y.cost));

    if !candidates.is_empty() {
        info!("Found {} possibilities...", candidates.len());
    } else {
        error!("Could not find any solutions.");
        return;
    }

    let decision = &candidates[0];
    let spell = &decision.spelling;
    let mut measures = Vec::with_capacity(measure_lengths.len());
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
    );

    debug!("{ly_file}");

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
