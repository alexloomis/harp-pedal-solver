use crate::cli::CONST;
use crate::cost::{astar_cost, astar_heuristic};
use crate::prelude::*;
use itertools::Itertools;
use pathfinding::directed::astar::{astar_bag, AstarSolution};
use std::iter;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Change {
    pub note: Note,
    pub time: usize,
}

impl Change {
    pub fn new(note: Note) -> Change {
        Change { note, time: 0 }
    }

    pub fn advance(&mut self) {
        self.time += 1
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct AstarState {
    pub beat: usize,
    pub pedals: Harp,
    // (last note, how long ago it was)
    pub last_left: Option<Change>,
    pub last_right: Option<Change>,
    pub early: [bool; 7],
}

fn advance_memory(m: Option<Change>) -> Option<Change> {
    if let Some(mut change) = m {
        if change.time >= CONST.forget_after {
            None
        } else {
            change.advance();
            Some(change)
        }
    } else {
        None
    }
}

impl AstarState {
    pub fn new(pedals: Harp) -> AstarState {
        AstarState {
            beat: 0,
            pedals,
            last_left: None,
            last_right: None,
            early: [false; 7],
        }
    }

    pub fn set_early(&mut self, name: Name) {
        self.early[name_to_usize(name)] = true;
    }

    // Unset notes that are no longer early
    pub fn unset_early(&mut self, target: Harp) {
        for (i, b) in self.early.iter_mut().enumerate() {
            *b = *b && target[i].is_none();
        }
    }

    pub fn advance(
        &mut self,
        left: Option<Note>,
        right: Option<Note>,
        target: Harp,
    ) {
        self.beat += 1;
        match left {
            Some(note) => {
                set_pedal(&mut self.pedals, note);
                self.last_left = Some(Change::new(note));
                self.set_early(note.name);
            }
            None => {
                self.last_left = advance_memory(self.last_left);
            }
        }
        match right {
            Some(note) => {
                set_pedal(&mut self.pedals, note);
                self.last_right = Some(Change::new(note));
                self.set_early(note.name);
            }
            None => {
                self.last_right = advance_memory(self.last_right);
            }
        }
        self.unset_early(target);
    }
}

// What changes can we make with our left foot?
// None means changing nothing is an option, empty means there are no options.
fn left_targets(state: AstarState, target: Harp) -> Vec<Option<Note>> {
    let l_changes = harp_changes(state.pedals, target, 0..=2);
    match &l_changes[..] {
        // Can change a single pedal, if undetermined
        [] => {
            let mut new_lefts = vec![];
            new_lefts.push(None);
            for (j, n) in target[0..=2].iter().enumerate() {
                if n.is_none() {
                    for new in [Some(Flat), Some(Natural), Some(Sharp)] {
                        if state.pedals[j] != new {
                            new_lefts.push(idx_to_note(j, new));
                        }
                    }
                }
            }
            new_lefts
        }
        // Must change the given pedal
        [n] => {
            vec![Some(*n)]
        }
        _ => {
            vec![]
        }
    }
}

fn right_targets(state: AstarState, target: Harp) -> Vec<Option<Note>> {
    let r_changes = harp_changes(state.pedals, target, 3..=6);
    match &r_changes[..] {
        // Can change a single pedal, if undetermined
        [] => {
            let mut new_rights = vec![];
            new_rights.push(None);
            for (j, n) in target[3..=6].iter().enumerate() {
                if n.is_none() {
                    for new in [Some(Flat), Some(Natural), Some(Sharp)] {
                        if state.pedals[j] != new {
                            new_rights.push(idx_to_note(j + 3, new));
                        }
                    }
                }
            }
            new_rights
        }
        // Must change the given pedal
        [n] => {
            vec![Some(*n)]
        }
        _ => {
            vec![]
        }
    }
}

// let left_is_early = l_changes.len() > 1;
// let right_is_early = r_changes.len() > 1;
fn get_targets(state: AstarState, target: Harp) -> Vec<AstarState> {
    let mut out: Vec<AstarState> = vec![];
    let l_changes = left_targets(state, target);
    let r_changes = right_targets(state, target);
    for (left, right) in l_changes.into_iter().cartesian_product(r_changes) {
        let mut new_state = state;
        new_state.advance(left, right, target);
        out.push(new_state);
    }
    out
}

fn target_costs(
    state: AstarState,
    targets: &[Harp],
) -> Vec<(AstarState, usize)> {
    let mut out = vec![];
    for target in targets {
        out.append(
            &mut get_targets(state, *target)
                .into_iter()
                .map(|t| (t, astar_cost(state, t)))
                .collect_vec(),
        )
    }
    out
}

fn succ(
    state: AstarState,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Vec<(AstarState, usize)> {
    let i = state.beat;
    if i < mid.len() {
        target_costs(state, &mid[i])
    } else if i == mid.len() {
        target_costs(state, &[end])
    } else {
        vec![]
    }
}

fn min_score_via_astar(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Option<(AstarSolution<AstarState>, usize)> {
    astar_bag(
        // Initial state
        &AstarState::new(start),
        // Given we are at state, where can we go?
        |&state| succ(state, mid, end),
        // Heuristic giving a lower bound on the distance p to end
        |&state| astar_heuristic(state, end),
        // success
        |&state| state.beat > mid.len(),
    )
}

fn possible_starts(state: Harp) -> Vec<Harp> {
    let mut choices =
        iter::repeat(vec![Some(Flat), Some(Natural), Some(Sharp)])
            .take(7)
            .collect_vec();
    for (i, m) in state.iter().enumerate() {
        if m.is_some() {
            choices[i] = vec![*m];
        }
    }
    let mut out = Vec::with_capacity(choices.iter().map(|v| v.len()).product());
    for v in choices
        .into_iter()
        .map(|c| c.into_iter())
        .multi_cartesian_product()
    {
        let mut new = [None; 7];
        for (i, c) in v.iter().enumerate() {
            new[i] = *c;
        }
        out.push(new)
    }
    out
}

pub fn find_solutions(
    start: Harp,
    mid: &[Vec<Harp>],
    end: Harp,
) -> Vec<Vec<AstarState>> {
    let mut best_score = usize::MAX;
    let mut best_choice = vec![];
    for s in possible_starts(start) {
        if let Some((astar, score)) = min_score_via_astar(s, mid, end) {
            if score < best_score {
                best_score = score;
                best_choice = astar.into_iter().collect_vec();
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
    out
}
