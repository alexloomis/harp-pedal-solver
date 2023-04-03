use crate::cost::enharmonic_cost;
use crate::prelude::{num_changes, update_harp, Harp};
use pathfinding::directed::astar::{astar_bag, AstarSolution};

// Given possible successors `a`, what are the possible states,
// the new index, and what is the transition enharmonic_cost?
fn annotate_successors(
    a: &[Harp],
    state: Harp,
    i: usize,
) -> Vec<((Harp, usize), usize)> {
    let f = |s: &Harp| update_harp(state, *s);
    a.iter()
        .map(|s| ((f(s), i + 1), enharmonic_cost(state, f(s))))
        .collect::<Vec<((Harp, usize), usize)>>()
}

fn successors(
    state: Harp,
    i: usize,
    middle: &[Vec<Harp>],
    end: Harp,
) -> Vec<((Harp, usize), usize)> {
    if i < middle.len() {
        annotate_successors(&middle[i], state, i)
    } else if i == middle.len() {
        annotate_successors(&[end], state, i)
    } else {
        annotate_successors(&[], state, i)
    }
}

fn min_score_via_astar(
    start: Harp,
    middle: &[Vec<Harp>],
    end: Harp,
    // Option<(AstarSolution<State>, Score)>
) -> Option<(AstarSolution<(Harp, usize)>, usize)> {
    astar_bag(
        // Initial state is (start, 0)
        &(start, 0),
        // Given we are at (s, i), where can we go?
        |&(state, i)| successors(state, i, middle, end),
        // Heuristic giving a lower bound on the distance p to end
        |&(state, _)| num_changes(state, end, 0..=6),
        // success
        |&(_, i)| i > middle.len(),
    )
}

pub fn find_enharmonic_paths(
    start: Harp,
    middle: &[Vec<Harp>],
    end: Harp,
) -> (Vec<Vec<Harp>>, usize) {
    let (astar, pw_score) = min_score_via_astar(start, middle, end).unwrap();
    let mut out: Vec<Vec<Harp>> = Vec::with_capacity(astar.size_hint().0);
    for path in astar {
        out.push(path.into_iter().map(|x| x.0).collect());
    }
    (out, pw_score)
}

// Trims start and end state.
pub fn find_enharmonic_paths_(
    start: Harp,
    middle: &[Vec<Harp>],
    end: Harp,
) -> Vec<Vec<Harp>> {
    let (astar, _) = min_score_via_astar(start, middle, end).unwrap();
    let mut out: Vec<Vec<Harp>> = Vec::with_capacity(astar.size_hint().0);
    for mut path in astar {
        path.pop();
        out.push(path.into_iter().map(|x| x.0).skip(1).collect());
    }
    out
}
