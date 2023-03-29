use crate::cli::CLI;
use crate::prelude::*;
use crate::shift::num_shifts;

// How many pedal changes to go between two states?

pub fn enharmonic_cost(start: Harp, finish: Harp) -> usize {
    let mut out = 0;
    let l_count = changes(start, finish, 0..=2);
    let r_count = changes(start, finish, 3..=6);
    out += CLI.pedal_cost * (l_count + r_count);
    if l_count > 1 {
        out += CLI.double_change_cost * (l_count - 1);
    }
    if r_count > 1 {
        out += CLI.double_change_cost * (r_count - 1);
    }
    out += CLI.double_string_cost * num_same(finish);
    out += CLI.cross_string_cost * num_crossed(finish);
    out
}

fn pedal_diff(old: Note, new: Note) -> usize {
    let f = |n: Note| name_to_usize(n.name);
    f(old).saturating_sub(f(new)) + f(new).saturating_sub(f(old))
}

pub fn pedal_cost(new: &[Option<Note>]) -> usize {
    let mut cost = 0;
    let mut change_cost: usize = 0;
    // Doesn't matter what, since change cost starts at zero.
    let mut last_note = Note {
        name: Name::C,
        modifier: Modifier::Flat,
    };
    for change in new {
        match change {
            Some(note) => {
                cost += change_cost
                    * pedal_diff(last_note, *note)
                    * CLI.pedal_diatance_cost;
                change_cost = CLI.quick_change_cost;
                last_note = *note;
            }
            None => {
                change_cost =
                    change_cost.saturating_sub(CLI.quick_change_decay);
            }
        }
    }
    cost
}

pub fn shift_cost(old: &[Vec<Note>]) -> usize {
    num_shifts(old) * CLI.early_cost
}
