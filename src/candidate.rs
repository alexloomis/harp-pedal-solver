use log::info;

use crate::{
    prelude::*,
    solve::{get_pedal_changes, get_spellings},
};

pub fn find_candidates(
    input: &MusicInput,
) -> Result<Vec<Candidate>, Vec<usize>> {
    info!("Managing enharmonic spellings...");
    let (spellings, cost) = get_spellings(input)?;
    let average_cost = cost / input.music.len();
    let mut candidates: Vec<CandidateBuilder> =
        Vec::with_capacity(spellings.len());
    for s in spellings {
        let mut candidate = CandidateBuilder::new();
        candidate.set_diagram(update_harp(
            [Some(Flat); 7],
            update_harp(
                input.goal,
                update_harp(full_initial(&s), input.diagram),
            ),
        ));
        candidate.set_destination(update_harp(
            update_harp([Some(Flat); 7], update_harps(input.diagram, &s)),
            input.goal,
        ));
        candidate.set_spelling(s);
        candidate.set_pedals(get_pedal_changes(&candidate));
        candidate.refine_spelling(input);
        candidates.push(candidate);
    }

    let mut out = Vec::with_capacity(candidates.len());
    for mut c in candidates {
        c.set_cost(average_cost);
        if let Some(new) = c.try_init() {
            out.push(new)
        }
    }
    Ok(out)
}
