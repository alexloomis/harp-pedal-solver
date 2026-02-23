use crate::cost::astar_heuristic;
use crate::model::{possible_starts, succ, ModelState};
use crate::prelude::*;
use itertools::Itertools;
use pathfinding::directed::astar::{astar_bag, AstarSolution};

fn min_score_via_astar(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Option<(AstarSolution<ModelState>, usize)> {
    astar_bag(
        // Initial state
        &ModelState::new(start),
        // Given we are at state, where can we go?
        |&state| succ(state, mid, end),
        // Heuristic giving a lower bound on the distance p to end
        |&state| astar_heuristic(state, end),
        // success
        |&state| state.beat > mid.len(),
    )
}

pub fn find_solutions(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> (Vec<Vec<ModelState>>, usize) {
    let mut best_score = usize::MAX;
    let mut best_choice = vec![];
    for s in possible_starts(start) {
        if let Some((astar, score)) = min_score_via_astar(s, mid, end) {
            if score < best_score {
                best_score = score;
                best_choice = astar.into_iter().collect_vec();
                // Allow tied options from a diffeent pedal setting
                // } else if score == best_score {
                //     best_choice.append(&mut astar.into_iter().collect_vec());
            }
        }
    }
    let mut out = vec![];
    for mut path in best_choice {
        path.pop();
        out.push(path.into_iter().skip(1).collect_vec());
    }
    (out, best_score)
}
