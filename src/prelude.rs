use itertools::Itertools;

pub use crate::prelude::harp::*;
pub use crate::prelude::note::*;
pub use crate::prelude::pitch_class::*;

pub use note::Accidental::*;

// The ways of storing notes are
// Note - a single note suitable for human readability,
// or if enharmonics should be treated differently.
// PichClass - a single note suitible for numeric manipulation,
// or if enharmonics should be treated identically.
// Harp - A collection of upto one note per scale degree.

pub mod harp;
pub mod note;
pub mod pitch_class;

pub type Pedals = Vec<Vec<Note>>;

pub fn unzip_pedals(pedals: &Pedals) -> (Pedals, Pedals) {
    let lefts = pedals
        .iter()
        .map(|v| v.iter().filter(|n| n.is_left()).copied().collect_vec())
        .collect_vec();
    let rights = pedals
        .iter()
        .map(|v| v.iter().filter(|n| n.is_right()).copied().collect_vec())
        .collect_vec();
    (lefts, rights)
}

pub struct MusicInput {
    pub diagram: Harp,
    // presets, other pitches
    pub music: Vec<(Vec<Note>, Vec<PitchClass>)>,
    pub goal: Harp,
}

#[derive(Clone, Debug)]
pub struct CandidateBuilder {
    pub diagram: Option<Harp>,
    pub destination: Option<Harp>,
    pub spelling: Option<Vec<Harp>>,
    // Should be one longer than spelling, last is required changes for goal.
    pub pedals: Option<Pedals>,
    pub cost: Option<usize>,
}

impl CandidateBuilder {
    pub fn new() -> CandidateBuilder {
        CandidateBuilder {
            diagram: None,
            destination: None,
            spelling: None,
            pedals: None,
            cost: None,
        }
    }

    pub fn set_diagram(&mut self, diagram: Harp) {
        self.diagram = Some(diagram);
    }

    pub fn set_destination(&mut self, destination: Harp) {
        self.destination = Some(destination);
    }

    pub fn set_spelling(&mut self, spelling: Vec<Harp>) {
        self.spelling = Some(spelling);
    }

    pub fn set_pedals(&mut self, pedals: Pedals) {
        self.pedals = Some(pedals);
    }

    pub fn set_cost(&mut self, cost: usize) {
        self.cost = Some(cost);
    }

    pub fn refine_spelling(&mut self, input: &MusicInput) {
        for s in self.spelling.iter_mut() {
            // For each beat
            for (i, m) in input.music.iter().enumerate() {
                // For each pitch
                for (j, d) in s[i].iter_mut().enumerate() {
                    if let Some(note) = idx_to_note(j, *d) {
                        if !m.0.contains(&note)
                            && !m.1.contains(&note_to_pc(note))
                        {
                            *d = None;
                        }
                    }
                }
            }
        }
    }

    pub fn try_init(self) -> Option<Candidate> {
        Some(Candidate {
            diagram: self.diagram?,
            destination: self.destination?,
            spelling: self.spelling?,
            pedals: self.pedals?,
            cost: self.cost?,
        })
    }
}

impl Default for CandidateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Candidate {
    pub diagram: Harp,
    pub destination: Harp,
    pub spelling: Vec<Harp>,
    // Should be one longer than spelling, last is required changes for goal.
    pub pedals: Pedals,
    pub cost: usize,
}
