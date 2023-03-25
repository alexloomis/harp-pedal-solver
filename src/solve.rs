#![allow(dead_code)]
use crate::assign::assign;
use crate::base::*;
use crate::parse::Measure;
use crate::transition::find_paths;

// Run the initial solver.
pub fn solve(
    (x, y, z): &(Option<Harp>, Vec<Measure>, Option<Harp>),
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

// Finds the pedal changes for each foot.
fn pedal_changes(music: &[Harp]) -> (Vec<Vec<Note>>, Vec<Vec<Note>>) {
    unset_seen(music)
        .iter()
        .map(|h| (harp_notes(*h, 0..=2), harp_notes(*h, 3..=6)))
        .unzip()
}
