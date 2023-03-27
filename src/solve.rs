#![allow(dead_code)]
use crate::assign::assign;
use crate::parse::Measure;
use crate::prelude::*;
use crate::shift::{num_shifts, shift};
use crate::transition::find_paths;

// Run the initial solver.
pub fn initial_solve(
    x: Option<Harp>,
    y: &[Measure],
    z: Option<Harp>,
) -> (Vec<Vec<Harp>>, usize) {
    let start = x.unwrap_or([0; 7]);
    let end = z.unwrap_or([0; 7]);
    let middle = y
        .iter()
        .flatten()
        .map(|m| {
            m.iter()
                .map(|n| note_to_pc(*n))
                .collect::<Vec<PitchClass>>()
        })
        .map(|p| assign(&p))
        .collect::<Vec<Vec<Harp>>>();
    find_paths(start, &middle, end)
}

// Finds the pedal changes for each foot,
// some may be simultaneous with the same foot.
pub fn initial_pedal_changes(
    music: &[Harp],
) -> (Vec<Vec<Note>>, Vec<Vec<Note>>) {
    unset_seen(music)
        .iter()
        .map(|h| (harp_notes(*h, 0..=2), harp_notes(*h, 3..=6)))
        .unzip()
}

// Drops the first (fake) bar.
pub fn shifted_changes(
    music: &[Vec<Note>],
    settings: &[Harp],
) -> (Vec<Vec<Option<Note>>>, Vec<Vec<Option<Note>>>) {
    let (l, r) = initial_pedal_changes(settings);
    if num_shifts(&l[1..]) == usize::MAX || num_shifts(&r[1..]) == usize::MAX {
        (vec![], vec![])
    } else {
        (shift(&music[1..], &l[1..]), shift(&music[1..], &r[1..]))
    }
}

fn score(_pedals: &[Option<Note>]) -> usize {
    1
}

pub fn scored_changes(
    music: &[Vec<Note>],
    settings: &[Harp],
) -> Vec<(Vec<Vec<Note>>, usize)> {
    let mut out = vec![];
    let (lefts, rights) = shifted_changes(music, settings);
    for l in lefts.iter() {
        for r in rights.iter() {
            let mut pedals_l_r = vec![];
            // l.len() should equal r.len()
            for i in 0..l.len() {
                let mut pedals_l_r_i = vec![];
                if let Some(p_l) = l[i] {
                    pedals_l_r_i.push(p_l);
                }
                if let Some(p_r) = r[i] {
                    pedals_l_r_i.push(p_r);
                }
                pedals_l_r.push(pedals_l_r_i);
            }
            out.push((pedals_l_r, score(l) + score(r)));
        }
    }
    out
}

// Something after this comment does not work as intended.

pub fn solve(
    x: Option<Harp>,
    y: &[Measure],
    z: Option<Harp>,
) -> Vec<(Vec<Vec<Note>>, usize)> {
    let mut out = vec![];
    let (choices, _) = initial_solve(x, y, z);
    let mut full_music = vec![harp_to_notes(x.unwrap_or([0; 7]))];
    // let mut full_music = vec![];
    full_music.append(&mut y.iter().flatten().map(|v| v.to_vec()).collect());
    full_music.push(harp_to_notes(z.unwrap_or([0; 7])));
    for settings in choices {
        out.append(&mut scored_changes(&full_music, &settings))
        // out.append(&mut scored_changes(&full_music, &settings[1..]))
    }
    out
}
