pub use crate::base::harp::*;
pub use crate::base::note::*;
pub use crate::base::pitch_class::*;

// The ways of storing notes are
// Note - a single note suitable for human readability
// PichClass - a single note suitible for numeric manipulation
// Harp - upto one note per scale degree

pub mod note {
    use enum_iterator::Sequence;
    use std::fmt;

    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Sequence)]
    pub enum Name {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }

    impl fmt::Display for Name {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Name::A => write!(f, "A"),
                Name::B => write!(f, "B"),
                Name::C => write!(f, "C"),
                Name::D => write!(f, "D"),
                Name::E => write!(f, "E"),
                Name::F => write!(f, "F"),
                Name::G => write!(f, "G"),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Modifier {
        Flat,
        Sharp,
        Natural,
    }

    impl fmt::Display for Modifier {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Modifier::Flat => write!(f, "♭"),
                Modifier::Natural => write!(f, "♮"),
                Modifier::Sharp => write!(f, "♯"),
            }
        }
    }

    pub fn pedal_symbol(modifier: Modifier) -> char {
        match modifier {
            Modifier::Flat => '^',
            Modifier::Natural => '-',
            Modifier::Sharp => 'v',
        }
    }

    pub fn pedal_symbol_opt(modifier: Option<Modifier>) -> char {
        match modifier {
            Some(x) => pedal_symbol(x),
            None => '~',
        }
    }

    #[derive(Copy, Clone)]
    pub struct Note {
        pub name: Name,
        pub modifier: Modifier,
    }

    impl fmt::Debug for Note {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}{}", self.name, self.modifier)
        }
    }

    impl fmt::Display for Note {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}{}", self.name, self.modifier)
        }
    }

    pub fn read_note(string: &str) -> Note {
        let mut chars = string.chars();
        let name = match chars.next() {
            Some('A') => Name::A,
            Some('B') => Name::B,
            Some('C') => Name::C,
            Some('D') => Name::D,
            Some('E') => Name::E,
            Some('F') => Name::F,
            Some('G') => Name::G,
            None => {
                panic!(
                    "Empty string given as note name. John Cage, is that you?"
                )
            }
            Some(x) => panic!("Invalid note name {x}"),
        };
        let modifier = match chars.next() {
            Some('b' | 'f' | '♭') => Modifier::Flat,
            Some('n' | '♮') | None => Modifier::Natural,
            Some('s' | '#' | '♯') => Modifier::Sharp,
            Some(x) => panic!("Invalid modifier {x}"),
        };
        Note { name, modifier }
    }
}

pub mod pitch_class {
    use crate::base::{read_note, Modifier, Name, Note};
    // 0 is Ab, 1 is A, etc.
    pub type PitchClass = u8;

    fn name_to_u8(name: Name) -> u8 {
        match name {
            Name::A => 0,
            Name::B => 2,
            Name::C => 3,
            Name::D => 5,
            Name::E => 7,
            Name::F => 8,
            Name::G => 10,
        }
    }

    fn modifier_to_u8(modifier: Modifier) -> u8 {
        match modifier {
            Modifier::Flat => 0,
            Modifier::Natural => 1,
            Modifier::Sharp => 2,
        }
    }

    pub fn note_to_pc(note: Note) -> PitchClass {
        (name_to_u8(note.name) + modifier_to_u8(note.modifier)) % 12
    }

    pub fn pc_to_notes(pc: PitchClass) -> Vec<Note> {
        match pc % 12 {
            0 => vec![read_note("Gs"), read_note("Af")],
            1 => vec![read_note("An")],
            2 => vec![read_note("As"), read_note("Bf")],
            3 => vec![read_note("Cf"), read_note("Bn")],
            4 => vec![read_note("Bs"), read_note("Cn")],
            5 => vec![read_note("Cs"), read_note("Df")],
            6 => vec![read_note("Dn")],
            7 => vec![read_note("Ds"), read_note("Ef")],
            8 => vec![read_note("Ff"), read_note("En")],
            9 => vec![read_note("Es"), read_note("Fn")],
            10 => vec![read_note("Fs"), read_note("Gf")],
            11 => vec![read_note("Gn")],
            _ => panic!("unreachable state, pc mod 12 should be in 0..11"),
        }
    }

    // Chooses natural if possible, flat otherwise.
    pub fn pc_to_note(pc: PitchClass) -> Note {
        pc_to_notes(pc).pop().unwrap()
    }
}

pub mod harp {
    use crate::base::{note_to_pc, pedal_symbol_opt, Modifier, Name, Note};
    use itertools::Itertools;

    // Index 0 is D, 1 is C, etc. (see pedal_to_u8)
    // Value 0 is unassigned, 1 is flat, 2 is natural, 3 is sharp, others undefined
    pub type Harp = [u8; 7];

    pub fn pedal_diagram(harp: Harp) -> String {
        let mut out = String::from("");
        for (i, h) in harp.into_iter().enumerate() {
            if i == 3 {
                out.push('|');
            }
            out.push(pedal_symbol_opt(u8_to_modifier(h)));
        }
        out
    }

    // Define pedals
    fn name_to_usize(name: Name) -> usize {
        match name {
            Name::D => 0,
            Name::C => 1,
            Name::B => 2,
            Name::E => 3,
            Name::F => 4,
            Name::G => 5,
            Name::A => 6,
        }
    }

    fn usize_to_name(u: usize) -> Name {
        match u {
            0 => Name::D,
            1 => Name::C,
            2 => Name::B,
            3 => Name::E,
            4 => Name::F,
            5 => Name::G,
            6 => Name::A,
            x => panic!("Invalid pedal index {x}"),
        }
    }

    fn modifier_to_u8(modifier: Modifier) -> u8 {
        match modifier {
            Modifier::Flat => 1,
            Modifier::Natural => 2,
            Modifier::Sharp => 3,
        }
    }

    fn u8_to_modifier(u: u8) -> Option<Modifier> {
        match u {
            0 => None,
            1 => Some(Modifier::Flat),
            2 => Some(Modifier::Natural),
            3 => Some(Modifier::Sharp),
            x => panic!("Invalid modifier {x}"),
        }
    }

    pub fn notes_to_harp(notes: &[Note]) -> Harp {
        let mut out = [0; 7];
        for note in notes {
            out[name_to_usize(note.name)] = modifier_to_u8(note.modifier);
        }
        out
    }

    pub fn harp_notes<R>(harp: Harp, range: R) -> Vec<Note>
    where
        R: std::ops::RangeBounds<usize> + std::iter::IntoIterator<Item = usize>,
    {
        // Only really need capacity range.len()
        let mut out = Vec::with_capacity(7);
        for i in range {
            if let Some(h) = u8_to_modifier(harp[i]) {
                out.push(Note {
                    name: usize_to_name(i),
                    modifier: h,
                });
            }
        }
        out
    }

    pub fn harp_to_notes(harp: Harp) -> Vec<Note> {
        harp_notes(harp, 0..=6)
    }

    pub fn update_harp(state: Harp, changes: Harp) -> Harp {
        let mut out = state;
        for i in 0..=6 {
            if changes[i] != 0 {
                out[i] = changes[i];
            }
        }
        out
    }

    pub fn num_crossed(state: Harp) -> usize {
        let mut out = 0;
        // If C is flat and B is sharp
        if state[1] == 1 && state[2] == 3 {
            out += 1;
        }
        // If E is sharp and C is flat
        if state[3] == 3 && state[2] == 1 {
            out += 1;
        }
        out
    }

    pub fn num_same(state: Harp) -> usize {
        let pitches = harp_to_notes(state);
        // If performance ever becomes an issue,
        // replace unique() with a specialized filter
        pitches.len() - pitches.iter().map(|n| note_to_pc(*n)).unique().count()
    }

    pub fn unset_seen(harps: &[Harp]) -> Vec<Harp> {
        let mut state = harps[0];
        let mut out = Vec::with_capacity(harps.len());
        out.push(harps[0]);
        for harp in harps[1..].iter() {
            let mut new = [0; 7];
            for j in 0..=6 {
                // If the value at j is set and is new
                if harp[j] != 0 && state[j] != harp[j] {
                    new[j] = harp[j];
                }
            }
            out.push(new);
            state = update_harp(state, new);
        }
        out
    }

    // pub const LEFT: [Name; 3] = [Name::D, Name::C, Name::B];
    // pub const RIGHT: [Name; 4] = [Name::E, Name::F, Name::G, Name::A];
}
