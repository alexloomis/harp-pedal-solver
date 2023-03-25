use crate::base::{num_crossed, num_same, update_harp, Harp};
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

// What is the cost for each pedal change?
const PEDAL_COST: usize = 100;
// How much do we penalize simultaneous pedal changes on the same foot?
const DOUBLE_CHANGE_COST: usize = 40;
// How much do we penalize doubled strings (eg E# and F)?
const DOUBLE_STRING_COST: usize = 10;
// How much do we penalize crossed strings (eg E# and Fb)?
const CROSS_STRING_COST: usize = 120;
// How much do we penalize each beat that
// a string is different than the key signature?
#[allow(dead_code)]
const OUT_OF_KEY: usize = 0;

fn cost(start: Harp, finish: Harp) -> usize {
    let mut out = 0;
    let l_count = changes(start, finish, 0..=2);
    let r_count = changes(start, finish, 3..=6);
    out += PEDAL_COST * (l_count + r_count);
    if l_count > 1 {
        out += DOUBLE_CHANGE_COST * (l_count - 1);
    }
    if r_count > 1 {
        out += DOUBLE_CHANGE_COST * (r_count - 1);
    }
    out += DOUBLE_STRING_COST * num_same(finish);
    out += CROSS_STRING_COST * num_crossed(finish);
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
