#![allow(dead_code)]
use itertools::Itertools;

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

fn merge_vec_opt<T>(u: Vec<Option<T>>, v: Vec<Option<T>>) -> Vec<Vec<T>> {
    let mut out = vec![];
    let z = u.into_iter().zip(v);
    for (u_, v_) in z {
        out.push(vec![u_, v_].into_iter().flatten().collect());
    }
    out
}

pub fn scored_changes(
    music: &[Vec<Note>],
    settings: &[Harp],
) -> Vec<(usize, Vec<Vec<Note>>)> {
    let (l_changes, r_changes) = pedal_changes(settings);
    let shift_cost =
        shift_cost(&l_changes).saturating_add(shift_cost(&r_changes));
    // Discards the initial pedal diagram.
    let (left_shifts, right_shifts) = shifted_changes(music, settings);
    left_shifts
        .into_iter()
        // For every combination of choices for left and right pedals,
        .cartesian_product(right_shifts.into_iter())
        .map(|(left, right)| {
            (
                // calculate their combined cost,
                pedal_cost(&left) + pedal_cost(&right) + shift_cost,
                // and combine them.
                merge_vec_opt(left, right),
            )
        })
        .collect_vec()
}

// pub fn scored_changes(
//     music: &[Vec<Note>],
//     settings: &[Harp],
// ) -> Vec<(usize, Vec<Vec<Note>>)> {
//     let mut out = vec![];
//     let (l_changes, r_changes) = pedal_changes(settings);
//     let shift_cost = shift_cost(&l_changes) + shift_cost(&r_changes);
//     // Discards the initial pedal diagram.
//     let (lefts, rights) = shifted_changes(music, settings);
//     for l in lefts.iter() {
//         for r in rights.iter() {
//             let mut pedals_l_r = vec![];
//             // l.len() should equal r.len()
//             for i in 0..l.len() {
//                 let mut pedals_l_r_i = vec![];
//                 pedals_l_r_i.extend(l[i]);
//                 pedals_l_r_i.extend(r[i]);
//                 pedals_l_r.push(pedals_l_r_i);
//             }
//             out.push((pedal_cost(l) + pedal_cost(r) + shift_cost, pedals_l_r));
//         }
//     }
//     out
// }

pub fn solve(
    x: Option<Harp>,
    y: &[Measure],
    z: Option<Harp>,
) -> Vec<(usize, Vec<Vec<Note>>)> {
    let mut out = vec![];
    let (choices, _) = initial_solve(x, y, z);
    let mut full_music = vec![harp_to_notes(x.unwrap_or([0; 7]))];
    full_music.append(&mut y.iter().flatten().map(|v| v.to_vec()).collect());
    full_music.push(harp_to_notes(z.unwrap_or([0; 7])));
    // If impossible, scored_changes should be empty.
    for settings in choices {
        out.append(&mut scored_changes(&full_music, &settings))
    }
    out
}

pub fn process_choice(choice: &[Harp]) -> Vec<Vec<Note>> {
    unset_seen(choice)
        .into_iter()
        .map(harp_to_notes)
        .skip(1)
        .collect_vec()
}
