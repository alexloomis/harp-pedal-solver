use crate::cli::CLI;
use crate::cost::astar_cost;
use crate::prelude::*;
use itertools::Itertools;
use log::trace;
use pathfinding::directed::astar::{astar_bag, AstarSolution};
use std::iter;

pub struct AstarState {
    pub pedals: Harp,
    // (last_note, change_cost)
    pub last_left: Option<(Note, usize)>,
    pub last_right: Option<(Note, usize)>,
    pub beat: usize,
}

fn advance_memory(m: Option<(Note, usize)>) -> Option<(Note, usize)> {
    if let Some((note, cost)) = m {
        let new_cost = cost.saturating_sub(CLI.quick_change_decay);
        if new_cost == 0 {
            None
        } else {
            Some((note, new_cost))
        }
    } else {
        None
    }
}

impl AstarState {
    pub fn advance(&mut self, left: Option<Note>, right: Option<Note>) {
        self.beat += 1;
        match left {
            Some(note) => {
                set_pedal(&mut self.pedals, note);
                self.last_left = Some((note, CLI.quick_change_cost));
            }
            None => {
                self.last_left = advance_memory(self.last_left);
            }
        }
        match right {
            Some(note) => {
                set_pedal(&mut self.pedals, note);
                self.last_right = Some((note, CLI.quick_change_cost));
            }
            None => {
                self.last_right = advance_memory(self.last_right);
            }
        }
    }
}

fn possibilities(state: Harp) -> Vec<Harp> {
    let mut choices = iter::repeat(vec![1, 2, 3]).take(7).collect_vec();
    for (i, m) in state.iter().enumerate() {
        if *m != 0 {
            choices[i] = vec![*m];
        }
    }
    let mut out = Vec::with_capacity(choices.iter().map(|v| v.len()).product());
    for v in choices
        .into_iter()
        .map(|c| c.into_iter())
        .multi_cartesian_product()
    {
        let mut new = [0; 7];
        for (i, c) in v.iter().enumerate() {
            new[i] = *c;
        }
        out.push(new)
    }
    out
}

// Change at most one note per foot. Assumes state is fully determined.
// If things are slow, this is a likely culprit.
fn valid_targets(_state: Harp, target: Harp) -> Vec<Harp> {
    // let n_left = num_changes(state, target, 0..=2);
    // let n_right = num_changes(state, target, 3..=6);
    // if n_left > 1 || n_right > 1 {
    //     return vec![];
    // }
    possibilities(target)
    // .into_iter()
    // .filter(|t| {
    //     num_changes(state, *t, 0..=2) <= 1
    //         && num_changes(state, *t, 3..=6) <= 1
    // })
    // .collect_vec()
}

fn target_costs(state: Harp, targets: &[Harp]) -> Vec<(Harp, usize)> {
    let mut out = vec![];
    for target in targets {
        out.append(
            &mut valid_targets(state, *target)
                .into_iter()
                .map(|t| (t, astar_cost(state, t)))
                .collect_vec(),
        )
    }
    out
}

fn succ(
    state: Harp,
    i: usize,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Vec<((Harp, usize), usize)> {
    if i < mid.len() {
        trace!("i< = {i}");
        target_costs(state, &mid[i])
            .into_iter()
            .map(|(s, c)| ((s, i + 1), c))
            .collect_vec()
    } else if i == mid.len() {
        trace!("i= = {i}");
        target_costs(state, &[end])
            .into_iter()
            .map(|(s, c)| ((s, i + 1), c))
            .collect_vec()
    } else {
        trace!("i> = {i}");
        vec![]
    }
}

fn min_score_via_astar(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Option<(AstarSolution<(Harp, usize)>, usize)> {
    astar_bag(
        // Initial state is (start, 0)
        &(start, 0),
        // Given we are at (s, i), where can we go?
        |&(state, i)| succ(state, i, mid, end),
        // Heuristic giving a lower bound on the distance p to end
        |&(state, _)| astar_cost(state, end),
        // success
        |&(_, i)| i > mid.len(),
    )
}

pub fn find_spelling(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Vec<Vec<Harp>> {
    let mut best_score = usize::MAX;
    let mut best_choice = vec![];
    for s in possibilities(start) {
        if let Some((astar, score)) = min_score_via_astar(s, mid, end) {
            if score < best_score {
                best_score = score;
                best_choice = astar.into_iter().collect_vec();
            } else if score == best_score {
                best_choice.append(&mut astar.into_iter().collect_vec());
            }
            trace!("found one");
        }
    }
    let mut out = vec![];
    for mut path in best_choice {
        path.pop();
        out.push(path.into_iter().skip(1).map(|x| x.0).collect_vec());
    }
    out
}
