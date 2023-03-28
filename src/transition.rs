use crate::cli::CLI;
use crate::prelude::{num_crossed, num_same, update_harp, Harp};
use pathfinding::directed::astar::{astar_bag, AstarSolution};

// How many pedal changes to go between two states?
fn changes<R>(start: Harp, finish: Harp, range: R) -> usize
where
    R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>,
{
    let mut out = 0;
    for i in range {
        // If both start[i] and finish[i] are defined, and they differ
        if start[i] * finish[i] != 0 && start[i] != finish[i] {
            out += 1;
        }
    }
    out
}

fn cost(start: Harp, finish: Harp) -> usize {
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

// Given possible successors `a`, what are the possible states,
// the new index, and what is the transition cost?
fn annotate_successors(
    a: &[Harp],
    state: Harp,
    i: usize,
) -> Vec<((Harp, usize), usize)> {
    let f = |s: &Harp| update_harp(state, *s);
    a.iter()
        .map(|s| ((f(s), i + 1), cost(state, f(s))))
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
        |&(state, _)| changes(state, end, 0..=6),
        // success
        |&(_, i)| i > middle.len(),
    )
}

pub fn find_paths(
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
