use crate::prelude::*;
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Accidental {
    Flat,
    Sharp,
    Natural,
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Flat => write!(f, "♭"),
            Natural => write!(f, "♮"),
            Sharp => write!(f, "♯"),
        }
    }
}

pub fn pedal_symbol(modifier: Accidental) -> char {
    match modifier {
        Flat => '^',
        Natural => '-',
        Sharp => 'v',
    }
}

pub fn pedal_symbol_opt(modifier: Option<Accidental>) -> char {
    match modifier {
        Some(x) => pedal_symbol(x),
        None => '~',
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Note {
    pub name: Name,
    pub accidental: Accidental,
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.accidental)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.accidental)
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
            panic!("Empty string given as note name. John Cage, is that you?")
        }
        Some(x) => panic!("Invalid note name {x}"),
    };
    let modifier = match chars.next() {
        Some('b' | 'f' | '♭') => Flat,
        Some('n' | '♮') | None => Natural,
        Some('s' | '#' | '♯') => Sharp,
        Some(x) => panic!("Invalid modifier {x}"),
    };
    Note {
        name,
        accidental: modifier,
    }
}

impl Note {
    pub fn is_left(&self) -> bool {
        (self.name == Name::D) | (self.name == Name::C) | (self.name == Name::B)
    }

    pub fn is_right(&self) -> bool {
        !self.is_left()
    }
}
