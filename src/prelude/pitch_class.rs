use crate::prelude::{read_note, Modifier, Name, Note};
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
