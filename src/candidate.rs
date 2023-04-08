use itertools::Itertools;
use log::{debug, error, info, trace, warn};

use crate::{
    cost::pedal_cost_both,
    prelude::*,
    solve::{
        find_spellings,
        // possible_pedals,
        possible_pedals_,
    },
};

pub fn find_candidates(input: &MusicInput) -> Vec<Candidate> {
    info!("Managing enharmonic spellings...");
    let spellings = find_spellings(input);
    let mut with_spelling: Vec<CandidateBuilder> =
        Vec::with_capacity(spellings.len());
    for s in spellings {
        let mut candidate = CandidateBuilder::new();
        candidate.set_diagram(update_harp(
            [1; 7],
            update_harp(
                input.goal,
                update_harp(full_initial(&s), input.diagram),
            ),
        ));
        candidate.set_destination(update_harp(
            update_harp([1; 7], update_harps(input.diagram, &s)),
            input.goal,
        ));
        candidate.set_spelling(s);
        with_spelling.push(candidate);
    }

    info!("Breaking up simultaneous pedal changes...");
    let mut with_pedals = Vec::with_capacity(with_spelling.len());
    for c in with_spelling {
        let mut candidates = vec![];
        for p in
            // possible_pedals(c.diagram.unwrap(), c.spelling.as_ref().unwrap())
            possible_pedals_(&c)
        {
            trace!("Found possibility: {:?}", p);
            let mut new = c.clone();
            new.set_pedals(p);
            candidates.push(new);
        }
        with_pedals.push(candidates);
    }
    let candidates: Vec<CandidateBuilder> =
        with_pedals.into_iter().flatten().collect_vec();
    let mut out = Vec::with_capacity(candidates.len());
    for mut c in candidates {
        let cost = pedal_cost_both(c.pedals.as_ref().unwrap());
        c.set_cost(cost);
        if let Some(new) = c.try_init() {
            out.push(new)
        }
    }
    out
}
