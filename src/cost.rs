use crate::astar::{AstarState, Change};
use crate::cli::CONST;
use crate::prelude::*;

pub fn astar_cost(state: AstarState, target: AstarState) -> usize {
    let mut out = 0;
    out += pedal_cost(state.last_left, target.last_left);
    out += pedal_cost(state.last_right, target.last_right);
    out += num_same(target.pedals) * CONST.double_string_cost;
    out += num_crossed(target.pedals) * CONST.cross_string_cost;
    out += quick_change_cost(state.last_left, target.last_left);
    out += quick_change_cost(state.last_right, target.last_right);
    out += early_change_cost(target);
    out
}

pub fn astar_heuristic(state: AstarState, target: Harp) -> usize {
    let mut out = 0;
    out += CONST.pedal_cost * num_changes(state.pedals, target, 0..=6);
    out += CONST.double_string_cost * num_same(target);
    out += CONST.cross_string_cost * num_crossed(target);
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
                out += CONST
                    .quick_change_cost
                    .saturating_sub(CONST.quick_change_decay * old.time);
            }
        }
    }
    out
}

pub fn early_change_cost(state: AstarState) -> usize {
    let mut out = 0;
    for b in state.early {
        if b {
            out += CONST.early_change_cost;
        }
    }
    out
}

pub fn pedal_cost(old: Option<Change>, new: Option<Change>) -> usize {
    let mut out = 0;
    if let Some(new) = new {
        if let Some(old) = old {
            out += CONST.pedal_diatance_cost * pedal_diff(old.note, new.note);
        }
        if new.time == 0 {
            out += CONST.pedal_cost;
        }
    }
    out
}
