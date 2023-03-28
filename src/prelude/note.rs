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
            panic!("Empty string given as note name. John Cage, is that you?")
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
