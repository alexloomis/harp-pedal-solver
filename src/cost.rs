use crate::astar::{AstarState, Change};
use crate::cli::CLI;
use crate::prelude::*;

pub fn astar_cost(state: AstarState, target: AstarState) -> usize {
    let mut out = astar_heuristic(state, target.pedals);
    out += pedal_cost(state.last_left, target.last_left);
    out += pedal_cost(state.last_right, target.last_right);
    out += quick_change_cost(state.last_left, target.last_left);
    out += quick_change_cost(state.last_right, target.last_right);
    out
}

pub fn astar_heuristic(state: AstarState, end: Harp) -> usize {
    let start = state.pedals;
    let mut out = 0;
    let l_count = num_changes(start, end, 0..=2);
    let r_count = num_changes(start, end, 3..=6);
    out += CLI.pedal_cost * (l_count + r_count);
    // if l_count > 1 {};
    // if r_count > 1 {};
    out += CLI.double_string_cost * num_same(end);
    out += CLI.cross_string_cost * num_crossed(end);
    out
}

fn pedal_diff(old: Note, new: Note) -> usize {
    let f = |n: Note| name_to_usize(n.name);
    f(old).saturating_sub(f(new)) + f(new).saturating_sub(f(old))
}

pub fn quick_change_cost(old: Option<Change>, new: Option<Change>) -> usize {
    let mut out = 0;
    // cost is decayed quick_change_cost
    if let Some(old) = old {
        if let Some(new) = new {
            if old.note != new.note {
                out += old.cost;
                out += CLI.pedal_diatance_cost * pedal_diff(old.note, new.note);
            }
        }
    }
    out
}

pub fn pedal_cost(old: Option<Change>, new: Option<Change>) -> usize {
    let mut out = 0;
    if let Some(new) = new {
        out = CLI.pedal_cost;
        if new.early {
            out += CLI.early_change_cost;
        }
        if let Some(old) = old {
            if old.note == new.note {
                out = 0;
            }
        }
    }
    out
}
