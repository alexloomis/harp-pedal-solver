use crate::cli::CLI;
use crate::prelude::*;

pub fn astar_cost(start: Harp, finish: Harp) -> usize {
    let mut out = 0;
    let l_count = num_changes(start, finish, 0..=2);
    let r_count = num_changes(start, finish, 3..=6);
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

pub fn pedal_cost(new: &[Vec<Note>]) -> usize {
    let mut cost = 0;
    let mut change_cost: usize = 0;
    // Doesn't matter what, since change cost starts at zero.
    let mut last_note = Note {
        name: Name::C,
        modifier: Modifier::Flat,
    };
    for change in new {
        if change.is_empty() {
            change_cost = change_cost.saturating_sub(CLI.quick_change_decay);
        } else {
            // TODO: treat multiple changes differently
            cost += change_cost
                * pedal_diff(last_note, change[0])
                * CLI.pedal_diatance_cost;
            change_cost = CLI.quick_change_cost;
            last_note = change[0];
        }
    }
    cost
}

pub fn pedal_cost_both(pedals: &Pedals) -> usize {
    let (left, right) = unzip_pedals(pedals);
    pedal_cost(&left) + pedal_cost(&right)
}
