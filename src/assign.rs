use crate::prelude::*;
use crate::util::unravel_paths;
use itertools::Itertools;
use trees::{Forest, Tree};

// What pedals can be used to play a note,
// that aren't being used already for a different note?
fn viable_pedals(pc: PitchClass, used: &[Name]) -> Vec<Note> {
    // Each pich is playable on at most two pedals.
    let mut viable = Vec::with_capacity(2);
    for n in pc_to_notes(pc) {
        if !used.contains(&n.name) {
            viable.push(n);
        }
    }
    viable
}

// Given a note, and a list of used notes,
// create a child for every possible new pedal setting.
fn create_children(
    forest: &mut Forest<Note>,
    new_note: PitchClass,
    used: &[Name],
) {
    let pedals = viable_pedals(new_note, used);
    for pedal in pedals {
        forest.push_back(Tree::new(pedal));
    }
}

// Grow child nodes into trees, assigning remaining notes
// and not reusing used pedals.
fn grow_children(
    forest: &mut Forest<Note>,
    remaining_notes: &[PitchClass],
    used: &[Name],
) {
    for mut branch in forest.iter_mut() {
        let mut more_used = used.to_owned();
        more_used.push(branch.data().name);
        branch.append(assignment_builder(remaining_notes, &more_used));
    }
}

// If we expect grandchildren, kill barren children.
fn kill_barren_children(forest: &mut Forest<Note>) {
    for mut branch in forest.iter_mut() {
        if branch.has_no_child() {
            branch.detach();
        };
    }
}

// Build a tree of all possible assignments of notes,
// without reusing used pedals.
// All terminal nodes represent a complete assignment.
fn assignment_builder(notes: &[PitchClass], used: &[Name]) -> Forest<Note> {
    let mut forest = Forest::<Note>::new();
    if let Some((new_note, remaining_notes)) = notes.split_first() {
        create_children(&mut forest, *new_note, used);
        if !remaining_notes.is_empty() {
            grow_children(&mut forest, remaining_notes, used);
            kill_barren_children(&mut forest);
        }
    }
    forest
}

// Build a tree of all possible assignments of notes,
// All terminal nodes represent a complete assignment.
// Returns empty Forest if there are no possibilities.
pub fn assign(preset: &[Note], notes: &[PitchClass]) -> Vec<Harp> {
    let mut sanitized = Vec::new();
    // 1, 6, 11 can only be accessed by one pedal
    for x in [1, 6, 11, 0, 2, 3, 4, 5, 7, 8, 9, 10] {
        if notes.contains(&x) {
            sanitized.push(x);
        }
    }
    unravel_paths(assignment_builder(
        &sanitized,
        &preset.iter().map(|n| n.name).collect_vec(),
    ))
    .iter()
    .map(|v| notes_to_harp(v))
    .map(|h| update_harp(h, notes_to_harp(preset)))
    .collect_vec()
}
