#![allow(dead_code)]
use itertools::Itertools;

use crate::assign::assign;
use crate::astar::find_solutions;
use crate::prelude::*;

pub fn get_spellings(input: &MusicInput) -> Vec<Vec<Harp>> {
    let start = input.diagram;
    let end = input.goal;
    let mid = input
        .music
        .iter()
        .map(|(preset, other)| assign(preset, other))
        .collect::<Vec<Vec<Harp>>>();
    find_solutions(start, &mid, end)
        .into_iter()
        .map(|v| v.into_iter().map(|a| a.pedals).collect_vec())
        .collect_vec()
}

// result is one longer than spelling, since it includes
// changes left over to get to target state.
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
