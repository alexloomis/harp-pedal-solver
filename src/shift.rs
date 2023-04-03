use crate::prelude::*;
use crate::util::*;
use itertools::Itertools;
use log::trace;
use trees::{Forest, Tree};

// Is this pedal setting compatable with this bar?
fn can_change(music: &[Note], change: Note) -> bool {
    music
        .iter()
        .filter(|n| n.name == change.name)
        .all(|n| n.modifier == change.modifier)
}

fn grow_tree(tree: &mut Tree<Option<Note>>, forest: Forest<Option<Note>>) {
    for branch in forest {
        tree.push_back(branch);
    }
}

// Add branch with root None and branches
fn add_none(
    forest: &mut Forest<Option<Note>>,
    remaining: &[(Vec<Note>, Vec<Note>)],
) {
    let mut tree = Tree::new(None);
    grow_tree(&mut tree, change_builder(remaining, vec![]));
    forest.push_back(tree);
}

fn make_children(
    forest: &mut Forest<Option<Note>>,
    music: &[Note],
    acc: &[Note],
    remaining: &[(Vec<Note>, Vec<Note>)],
) {
    for (i, change) in acc.iter().enumerate() {
        // If we can't add it, we can't push it back further either,
        // so we don't create a branch for it.
        if can_change(music, *change) {
            let mut tree = Tree::new(Some(*change));
            let mut new_acc = acc.to_owned();
            new_acc.remove(i);
            grow_tree(&mut tree, change_builder(remaining, new_acc));
            forest.push_back(tree);
        }
    }
}

// If we expect grandchildren, kill barren children.
fn kill_barren_children(forest: &mut Forest<Option<Note>>) {
    for mut branch in forest.iter_mut() {
        if branch.has_no_child() {
            branch.detach();
        };
    }
}

// Pushes pedals *later*, so pedals and music need to be reversed.
// music_changes[i] are the pedals and music we currently have at time N-i.
// Returns changes the right way around again.
// TODO: make private, change tests to use shift
pub fn change_builder(
    music_changes: &[(Vec<Note>, Vec<Note>)],
    mut acc: Vec<Note>, // surplus changes getting passed on
) -> Forest<Option<Note>> {
    let mut forest = Forest::<Option<Note>>::new();
    if let Some(((new_music, new_changes), remaining)) =
        music_changes.split_first()
    {
        acc.extend(new_changes);
        // If we owe more changes than we have slots, there are no solutions.
        if acc.len() > remaining.len() + 1 {
            trace!("killing branch with acc {acc:?}");
            return forest;
        }
        if acc.is_empty() {
            add_none(&mut forest, remaining);
        } else {
            make_children(&mut forest, new_music, &acc, remaining);
        }
        // If we expect grandchildren, kill barren children.
        if !remaining.is_empty() {
            kill_barren_children(&mut forest);
        }
    }
    forest
}

pub fn shift(
    music: &[Vec<Note>],
    changes: &[Vec<Note>],
) -> Vec<Vec<Option<Note>>> {
    let mut vec = vec![];
    for i in (0..music.len()).rev() {
        vec.push((music[i].to_vec(), changes[i].to_vec()));
    }
    unravel_paths(change_builder(&vec, vec![]))
}

fn shift_pedals_builder(
    music: &[Harp],
    pedals: &[Vec<Note>],
) -> Vec<Vec<Option<Note>>> {
    let mut vec = vec![];
    for i in (0..music.len()).rev() {
        vec.push((harp_to_notes(music[i]).to_vec(), pedals[i].to_vec()));
    }
    unravel_paths(change_builder(&vec, vec![]))
}

pub fn shift_pedals(
    music: &[Harp],
    pedals: Vec<(Vec<Note>, Vec<Note>)>,
) -> Vec<Pedals> {
    let mut l = Vec::with_capacity(pedals.len());
    let mut r = Vec::with_capacity(pedals.len());
    for (left, right) in pedals {
        l.push(left);
        r.push(right);
    }
    let lefts = shift_pedals_builder(music, &l);
    let rights = shift_pedals_builder(music, &r);
    lefts
        .into_iter()
        .cartesian_product(rights)
        .map(|(v_l, v_r)| v_l.into_iter().zip(v_r).collect_vec())
        .collect_vec()
}

fn accumulate(total: usize, acc: usize, new: usize) -> (usize, usize) {
    let here = usize::from(acc + new > 0);
    (total + here, acc + new - here)
}

// How many shifts are needed?
fn num_shifts_builder(pedals: &[Vec<Note>]) -> (usize, usize) {
    pedals
        .iter()
        .map(|m| m.len())
        .fold((0, 0), |(x, y), z| accumulate(x, y, z))
}

// Returns usize::MAX if shifts not possible.
pub fn num_shifts(pedals: &[Vec<Note>]) -> usize {
    let (total, acc) = num_shifts_builder(pedals);
    if acc > 0 {
        usize::MAX
    } else {
        total
    }
}
