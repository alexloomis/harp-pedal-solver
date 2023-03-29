#![allow(dead_code)]
use crate::assign::assign;
use crate::cost::{pedal_cost, shift_cost};
use crate::enharmonic::find_enharmonic_paths;
use crate::parse::Measure;
use crate::prelude::*;
use crate::shift::{num_shifts, shift};

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
    find_enharmonic_paths(start, &middle, end)
}

// Finds the pedal changes for each foot,
// some may be simultaneous with the same foot.
pub fn pedal_changes(music: &[Harp]) -> (Vec<Vec<Note>>, Vec<Vec<Note>>) {
    unset_seen(music)
        .iter()
        .map(|h| (harp_notes(*h, 0..=2), harp_notes(*h, 3..=6)))
        .unzip()
}

#[allow(clippy::type_complexity)]
pub fn shifted_changes(
    spellings: &[Vec<Note>],
    pedals: &[Harp],
) -> (Vec<Vec<Option<Note>>>, Vec<Vec<Option<Note>>>) {
    // The first bar is the pedal diagram,
    // so we will work with l[1..] and r[1..]
    let (l, r) = pedal_changes(pedals);
    if num_shifts(&l[1..]) == usize::MAX || num_shifts(&r[1..]) == usize::MAX {
        (vec![], vec![])
    } else {
        (
            shift(&spellings[1..], &l[1..]),
            shift(&spellings[1..], &r[1..]),
        )
    }
}

pub fn scored_changes(
    music: &[Vec<Note>],
    settings: &[Harp],
) -> Vec<(Vec<Vec<Note>>, usize)> {
    let mut out = vec![];
    let (l_changes, r_changes) = pedal_changes(settings);
    let shift_cost = shift_cost(&l_changes) + shift_cost(&r_changes);
    // Discards the initial pedal diagram.
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
            out.push((pedals_l_r, pedal_cost(l) + pedal_cost(r) + shift_cost));
        }
    }
    out
}

pub fn solve(
    x: Option<Harp>,
    y: &[Measure],
    z: Option<Harp>,
) -> Vec<(Vec<Vec<Note>>, usize)> {
    let mut out = vec![];
    let (choices, _) = initial_solve(x, y, z);
    let mut full_music = vec![harp_to_notes(x.unwrap_or([0; 7]))];
    full_music.append(&mut y.iter().flatten().map(|v| v.to_vec()).collect());
    full_music.push(harp_to_notes(z.unwrap_or([0; 7])));
    for settings in choices {
        out.append(&mut scored_changes(&full_music, &settings))
    }
    out
}
