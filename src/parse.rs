use crate::prelude::*;
use itertools::Itertools;
use nom::{
    character::complete::{
        char, line_ending, multispace0, one_of, space0, space1,
    },
    combinator::{all_consuming, opt},
    error::ParseError,
    multi::{count, many1, separated_list1},
    sequence::delimited,
    Finish, IResult,
};

// Allow but don't require space before and after, excludes newlines.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// matches whitespace containing a newline
fn newline(s: &str) -> IResult<&str, ()> {
    let (rem, _) = delimited(space0, line_ending, multispace0)(s)?;
    Ok((rem, ()))
}

// Accepts a pedal setting, returns the appropriate u8 representing it.
fn pedal_setting(s: &str) -> IResult<&str, Option<Accidental>> {
    let (rem, c) = one_of("~^-v")(s)?;
    let setting = match c {
        '~' => None,
        '^' => Some(Flat),
        '-' => Some(Natural),
        'v' => Some(Sharp),
        _ => unreachable!(),
    };
    Ok((rem, setting))
}

// Accepts 3 pedal settings, followed by |, followed by 4 settings.
fn diagram(s: &str) -> IResult<&str, Harp> {
    let (rem, left) = count(pedal_setting, 3)(s)?;
    let (rem, _) = char('|')(rem)?;
    let (rem, right) = count(pedal_setting, 4)(rem)?;
    let mut harp = [None; 7];
    for (i, c) in left.into_iter().enumerate() {
        harp[i] = c;
    }
    for (i, c) in right.into_iter().enumerate() {
        harp[i + 3] = c;
    }
    Ok((rem, harp))
}

// Accepts a pitch name, returns the name.
fn pitch(s: &str) -> IResult<&str, Name> {
    let (rem, c) = one_of("abcdefgABCDEFG")(s)?;
    let name = match c {
        'A' | 'a' => Name::A,
        'B' | 'b' => Name::B,
        'C' | 'c' => Name::C,
        'D' | 'd' => Name::D,
        'E' | 'e' => Name::E,
        'F' | 'f' => Name::F,
        'G' | 'g' => Name::G,
        _ => unreachable!(),
    };
    Ok((rem, name))
}

// Accepts a pitch modifier, returns the modifier.
fn modifier(s: &str) -> IResult<&str, Accidental> {
    let (rem, c) = one_of("fb♭n♮s#♯")(s)?;
    let modif = match c {
        'f' | 'b' | '♭' => Accidental::Flat,
        'n' | '♮' => Accidental::Natural,
        's' | '#' | '♯' => Accidental::Sharp,
        _ => unreachable!(),
    };
    Ok((rem, modif))
}

// Accepts a pitch name and, if present, modifier, returns a note.
fn note(s: &str) -> IResult<&str, Note> {
    let (rem, name) = pitch(s)?;
    let (rem, m) = opt(modifier)(rem)?;
    let modifier = match m {
        Some(x) => x,
        None => Accidental::Natural,
    };
    Ok((
        rem,
        Note {
            name,
            accidental: modifier,
        },
    ))
}

// Accepts any number of notes, delimited by any amount of space,
// all on the same line.
// "b#\tc  d \t" -> (" \t", [B#, C, D])
fn beat(s: &str) -> IResult<&str, Vec<Note>> {
    separated_list1(many1(space1), note)(s)
}

pub type Measure = Vec<Vec<Note>>;

// Accepts beats sepparated by at least a new line,
// possibly with extra whitespace.
// "b#\tc  d \t
// \t
// fb  | " -> ("  | ", [[B#, C, D], [Fb]])
fn measure(s: &str) -> IResult<&str, Measure> {
    separated_list1(newline, beat)(s)
}

fn music(s: &str) -> IResult<&str, Vec<Measure>> {
    delimited(
        opt(ws(char('|'))),
        separated_list1(ws(char('|')), measure),
        opt(ws(char('|'))),
    )(s)
}

fn strip_line(s: &str) -> &str {
    match s.split_once('$') {
        Some((head, _)) => head,
        None => s,
    }
}

fn strip_comments(s: &str) -> String {
    s.lines().map(strip_line).join("\n")
}

#[allow(clippy::type_complexity)]
fn parse_clean_file(
    s: &str,
) -> IResult<&str, (Option<Harp>, Vec<Measure>, Option<Harp>)> {
    let (rem, start) = opt(ws(diagram))(s)?;
    let (rem, body) = ws(music)(rem)?;
    let (rem, end) = opt(ws(diagram))(rem)?;
    Ok((rem, (start, body, end)))
}

#[allow(clippy::type_complexity)]
pub fn parse(
    s: &str,
) -> Result<(Option<Harp>, Vec<Measure>, Option<Harp>), String> {
    let t = strip_comments(s);
    let r = all_consuming(parse_clean_file)(&t).finish();
    match r {
        Ok((_, y)) => Ok(y),
        Err(x) => Err(x.to_string()),
    }
}
