use crate::prelude::{note_to_pc, pedal_symbol_opt, Accidental, Name, Note};
use itertools::Itertools;

// Index 0 is D, 1 is C, etc. (see pedal_to_u8)
// Value 0 is unassigned, 1 is flat, 2 is natural, 3 is sharp, others undefined
pub type Harp = [u8; 7];

pub fn pedal_diagram(harp: Harp) -> String {
    let mut out = String::from("");
    for (i, h) in harp.into_iter().enumerate() {
        if i == 3 {
            out.push('|');
        }
        out.push(pedal_symbol_opt(u8_to_modifier(h)));
    }
    out
}

// Define pedals
pub fn name_to_usize(name: Name) -> usize {
    match name {
        Name::D => 0,
        Name::C => 1,
        Name::B => 2,
        Name::E => 3,
        Name::F => 4,
        Name::G => 5,
        Name::A => 6,
    }
}

fn usize_to_name(u: usize) -> Name {
    match u {
        0 => Name::D,
        1 => Name::C,
        2 => Name::B,
        3 => Name::E,
        4 => Name::F,
        5 => Name::G,
        6 => Name::A,
        x => panic!("Invalid pedal index {x}"),
    }
}

fn modifier_to_u8(modifier: Accidental) -> u8 {
    match modifier {
        Accidental::Flat => 1,
        Accidental::Natural => 2,
        Accidental::Sharp => 3,
    }
}

fn u8_to_modifier(u: u8) -> Option<Accidental> {
    match u {
        0 => None,
        1 => Some(Accidental::Flat),
        2 => Some(Accidental::Natural),
        3 => Some(Accidental::Sharp),
        x => panic!("Invalid modifier {x}"),
    }
}

pub fn idx_to_note(idx: usize, val: u8) -> Option<Note> {
    u8_to_modifier(val).map(|m| Note {
        name: usize_to_name(idx),
        modifier: m,
    })
}

pub fn set_pedal(harp: &mut Harp, note: Note) {
    harp[name_to_usize(note.name)] = modifier_to_u8(note.modifier);
}

pub fn notes_to_harp(notes: &[Note]) -> Harp {
    let mut out = [0; 7];
    for note in notes {
        out[name_to_usize(note.name)] = modifier_to_u8(note.modifier);
    }
    out
}

pub fn harp_notes<R>(harp: Harp, range: R) -> Vec<Note>
where
    R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>,
{
    // Only really need capacity range.len()
    let mut out = Vec::with_capacity(7);
    for i in range {
        if let Some(h) = u8_to_modifier(harp[i]) {
            out.push(Note {
                name: usize_to_name(i),
                modifier: h,
            });
        }
    }
    out
}

pub fn harp_to_notes(harp: Harp) -> Vec<Note> {
    harp_notes(harp, 0..=6)
}

pub fn update_harp(state: Harp, change: Harp) -> Harp {
    let mut out = state;
    for i in 0..=6 {
        if change[i] != 0 {
            out[i] = change[i];
        }
    }
    out
}

pub fn update_harps(state: Harp, changes: &[Harp]) -> Harp {
    changes.iter().fold(state, |x, y| update_harp(x, *y))
}

pub fn update_harp_notes(state: Harp, changes: &[Vec<Note>]) -> Harp {
    let harp_changes = changes.iter().map(|v| notes_to_harp(v)).collect_vec();
    update_harps(state, &harp_changes)
}

// Any pedal that is never seen is set to flat.
pub fn full_initial(music: &[Harp]) -> Harp {
    let out = music.to_vec();
    // let last = out.pop().unwrap();
    let last = [1; 7];
    update_harps(last, &out.into_iter().rev().collect::<Vec<Harp>>())
}

// Unsets repeated notes.
// If note appears for first time partway through, add it to initial.
pub fn unset_seen(harps: &[Harp]) -> Vec<Harp> {
    let mut state = full_initial(harps);
    let mut out = Vec::with_capacity(harps.len());
    out.push(state);
    for harp in harps[1..].iter() {
        let mut new = [0; 7];
        for j in 0..=6 {
            // If the value at j is set and is new
            if harp[j] != 0 && state[j] != harp[j] {
                new[j] = harp[j];
            }
        }
        out.push(new);
        state = update_harp(state, new);
    }
    out
}

pub fn harp_changes<R>(start: Harp, finish: Harp, range: R) -> Vec<(usize, u8)>
where
    R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>,
{
    let mut out = Vec::with_capacity(7);
    for i in range {
        // If both start[i] and finish[i] are defined, and they differ
        if start[i] * finish[i] != 0 && start[i] != finish[i] {
            out.push((i, finish[i]))
        }
    }
    out
}

pub fn num_changes<R>(start: Harp, finish: Harp, range: R) -> usize
where
    R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>,
{
    let mut out = 0;
    for i in range {
        // If both start[i] and finish[i] are defined, and they differ
        if start[i] * finish[i] != 0 && start[i] != finish[i] {
            out += 1;
        }
    }
    out
}

pub fn num_crossed(state: Harp) -> usize {
    let mut out = 0;
    // If C is flat and B is sharp
    if state[1] == 1 && state[2] == 3 {
        out += 1;
    }
    // If E is sharp and C is flat
    if state[3] == 3 && state[2] == 1 {
        out += 1;
    }
    out
}

pub fn num_same(state: Harp) -> usize {
    let pitches = harp_to_notes(state);
    // If performance ever becomes an issue,
    // replace unique() with a specialized filter
    pitches.len() - pitches.iter().map(|n| note_to_pc(*n)).unique().count()
}

// Essentially, ([E, F, G], [E, F#, A]) -> F#
pub fn get_pedal_changes(state: Harp, target: Harp) -> (Vec<Note>, Vec<Note>) {
    let mut left_shifts = Vec::with_capacity(3);
    for (i, pedal) in target[..=2].iter().enumerate() {
        if state[i] != *pedal {
            if let Some(note) = idx_to_note(i, *pedal) {
                left_shifts.push(note)
            }
        }
    }
    let mut right_shifts = Vec::with_capacity(4);
    for (i, pedal) in target[3..].iter().enumerate() {
        if state[i] != *pedal {
            if let Some(note) = idx_to_note(i, *pedal) {
                right_shifts.push(note)
            }
        }
    }
    (left_shifts, right_shifts)
}

pub fn is_change(state: Harp, note: Note) -> bool {
    let idx = name_to_usize(note.name);
    let m = modifier_to_u8(note.modifier);
    !(state[idx] == 0 || state[idx] == m)
}
