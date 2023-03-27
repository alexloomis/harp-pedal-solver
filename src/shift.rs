use crate::prelude::*;
use crate::util::*;
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
    grow_tree(&mut tree, change_builder(remaining, &[]));
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
            grow_tree(&mut tree, change_builder(remaining, &new_acc));
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
    acc: &[Note], // surplus changes getting passed on
) -> Forest<Option<Note>> {
    let mut forest = Forest::<Option<Note>>::new();
    let mut acc_ = acc.to_owned();
    if let Some(((new_music, new_changes), remaining)) =
        music_changes.split_first()
    {
        acc_.extend(new_changes);
        // If we owe more changes than we have slots, there are no solutions.
        if acc_.len() > remaining.len() + 1 {
            println!("killing branch with acc {acc_:?}");
            return forest;
        }
        if acc_.is_empty() {
            add_none(&mut forest, remaining);
        } else {
            make_children(&mut forest, new_music, &acc_, remaining);
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
        vec.push((music[i].clone(), changes[i].clone()));
    }
    unravel_paths(change_builder(&vec, &[]))
}

fn accumulate(total: usize, acc: usize, new: usize) -> (usize, usize) {
    let here = usize::from(acc + new > 0);
    (total + here, acc + new - here)
}

// How many shifts are needed?
fn num_shifts_(pedals: &[Vec<Note>]) -> (usize, usize) {
    pedals
        .iter()
        .map(|m| m.len())
        .fold((0, 0), |(x, y), z| accumulate(x, y, z))
}

// Returns usize::MAX if shifts not possible.
pub fn num_shifts(pedals: &[Vec<Note>]) -> usize {
    let (total, acc) = num_shifts_(pedals);
    if acc > 0 {
        usize::MAX
    } else {
        total
    }
}
