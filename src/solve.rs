#![allow(dead_code)]
use itertools::Itertools;

use crate::assign::assign;
use crate::astar::find_spelling;
use crate::prelude::*;

// Refine may not work as intended when there's eg B and Cb simultaneously
pub fn get_spellings(input: &MusicInput) -> Vec<Vec<Harp>> {
    let start = input.diagram;
    let end = input.goal;
    let mid = input
        .music
        .iter()
        .map(|p| assign(p))
        .collect::<Vec<Vec<Harp>>>();
    find_spelling(start, &mid, end)
}

// result is one longer than spelling
pub fn get_pedal_changes(input: &CandidateBuilder) -> Vec<Vec<Note>> {
    let mut with_diagram =
        Vec::with_capacity(input.spelling.as_ref().unwrap().len() + 2);
    with_diagram.push(input.diagram.unwrap());
    for s in input.spelling.as_ref().unwrap() {
        with_diagram.push(*s);
    }
    with_diagram.push(input.destination.unwrap());
    unset_seen(&with_diagram)
        .iter()
        .map(|h| harp_to_notes(*h))
        .skip(1)
        .collect_vec()
}
