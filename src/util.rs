use trees::Forest;

// Unravels paths through a forest,
// each returned path is from a terminal node to a root.
// Stop trying to implement for &Forest, it can't be made into an iterator.
pub fn unravel_paths<T: Copy>(forest: Forest<T>) -> Vec<Vec<T>> {
    // There are as many paths as terminal nodes.
    let mut out = Vec::new();
    for mut branch in forest {
        let val = *branch.root().data();
        let paths = unravel_paths(branch.abandon());
        // If there's nowhere else to go,
        // the remainder of the list is the current value.
        // Otherwise, it's the path followed by the current value.
        if paths.is_empty() {
            out.push(vec![val]);
        } else {
            for mut v in paths {
                v.push(val);
                out.push(v);
            }
        }
    }
    out
}

// // If we expect grandchildren, kill barren children.
// fn kill_barren_children<T>(forest: &mut Forest<T>) {
//     for mut branch in forest.iter_mut() {
//         if branch.has_no_child() {
//             branch.detach();
//         };
//     }
// }
