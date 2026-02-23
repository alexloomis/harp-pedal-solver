use std::collections::HashMap;

use crate::model::{possible_starts, succ, ModelState};
use crate::prelude::*;

pub type DynamicState = Vec<(Vec<ModelState>, usize)>;

fn advance(state: &mut DynamicState, mid: &[Vec<Harp>], end: Harp) {
    let mut out = vec![];
    for (history, cost) in state.iter() {
        // Where can we go from the last state?
        let options = succ(*history.last().unwrap(), mid, end);
        for (option, delta_cost) in options {
            let mut extended = history.clone();
            extended.push(option);
            out.push((extended, *cost + delta_cost));
        }
    }
    *state = out
}

fn trim(state: &mut DynamicState) {
    let mut best: HashMap<ModelState, (Vec<ModelState>, usize)> =
        HashMap::new();

    for (history, cost) in state.drain(..) {
        let last = *history.last().expect("history should be nonempty");

        match best.get(&last) {
            None => {
                best.insert(last, (history, cost));
            }
            Some((_, best_cost)) => {
                if cost < *best_cost {
                    best.insert(last, (history, cost));
                }
            }
        }
    }

    *state = best.into_values().collect();
}

fn initial_state(harp: Harp) -> DynamicState {
    let mut out = vec![];
    let possible = possible_starts(harp);
    for pedals in possible {
        out.push((vec![ModelState::new(pedals)], 0))
    }
    out
}

pub fn find_solutions(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> (Vec<Vec<ModelState>>, usize) {
    let mut state = initial_state(start);

    // mid.len() intermediate steps, plus one to arrive at end
    for _ in 0..mid.len() + 1 {
        advance(&mut state, mid, end);
        trim(&mut state);
    }

    let mut best_histories: Vec<Vec<ModelState>> = vec![];
    let mut best_cost = usize::MAX;

    for (history, cost) in state.into_iter() {
        if cost < best_cost {
            best_cost = cost;
            best_histories = vec![history];
        } else if cost == best_cost {
            best_histories.push(history);
        }
    }

    // Trim start/end constraints
    let mut out = Vec::with_capacity(best_histories.len());
    for mut path in best_histories {
        if !path.is_empty() {
            path.pop();
        }
        let path = path.into_iter().skip(1).collect::<Vec<_>>();
        out.push(path);
    }

    (out, best_cost)
}
