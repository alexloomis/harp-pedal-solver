use crate::parse::NoteRequest::*;
use crate::prelude::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, multispace0, multispace1, one_of},
    combinator::{all_consuming, map, opt, value},
    error::ParseError,
    multi::{count, many0, separated_list1},
    sequence::{delimited, preceded},
    Finish, IResult,
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum NoteRequest {
    This(Note),
    Any(Note),
    Rest,
}

pub struct Parsed {
    pub start: Option<Harp>,
    pub this_any: Vec<Vec<(Vec<Note>, Vec<PitchClass>)>>,
    pub end: Option<Harp>,
}

// Allow but don't require space before and after, includes newlines.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
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

fn rest(s: &str) -> IResult<&str, NoteRequest> {
    value(Rest, one_of("rR"))(s)
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
        'f' | 'b' | '♭' => Flat,
        'n' | '♮' => Natural,
        's' | '#' | '♯' => Sharp,
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
        None => Natural,
    };
    Ok((
        rem,
        Note {
            name,
            accidental: modifier,
        },
    ))
}

// A note that can be respelled
fn any_note(s: &str) -> IResult<&str, NoteRequest> {
    map(note, Any)(s)
}

// A note that cannot be respelled
fn this_note(s: &str) -> IResult<&str, NoteRequest> {
    map(preceded(char('*'), note), This)(s)
}

fn note_request(s: &str) -> IResult<&str, NoteRequest> {
    alt((rest, this_note, any_note))(s)
}

// Accepts any number of note requests, delimited by any amount of space,
// all on the same line. "b#\tc  d \t" -> (" \t", [B#, C, D])
fn beat(s: &str) -> IResult<&str, Vec<NoteRequest>> {
    delimited(char('['), many0(ws(note_request)), char(']'))(s)
}

pub type Measure = Vec<Vec<NoteRequest>>;

// Accepts beats sepparated by at least a new line,
// possibly with extra whitespace.
// "b#\tc  d \t
// \t
// fb  | " -> ("  | ", [[B#, C, D], [Fb]])
fn measure(s: &str) -> IResult<&str, Measure> {
    separated_list1(multispace1, beat)(s)
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
// Parse an already processed file.
fn parse_clean_file(
    s: &str,
) -> IResult<&str, (Option<Harp>, Vec<Measure>, Option<Harp>)> {
    let (rem, start) = opt(ws(diagram))(s)?;
    let (rem, body) = ws(music)(rem)?;
    let (rem, end) = opt(ws(diagram))(rem)?;
    Ok((rem, (start, body, end)))
}

#[allow(clippy::type_complexity)]
fn pre_parse(
    s: &str,
) -> Result<(Option<Harp>, Vec<Measure>, Option<Harp>), String> {
    let t = strip_comments(s);
    let r = all_consuming(parse_clean_file)(&t).finish();
    match r {
        Ok((_, y)) => Ok(y),
        Err(x) => Err(x.to_string()),
    }
}

// List of rest, this, and any, to list of (this, any)
fn split_requests(
    requests: Vec<Measure>,
) -> Vec<Vec<(Vec<Note>, Vec<PitchClass>)>> {
    let mut out = Vec::with_capacity(requests.len());
    for measure in requests {
        let mut measure_contents = Vec::with_capacity(measure.len());
        for beat in measure {
            let mut this = Vec::with_capacity(beat.len());
            let mut any = Vec::with_capacity(beat.len());
            for req in beat {
                match req {
                    This(n) => this.push(n),
                    Any(n) => any.push(note_to_pc(n)),
                    Rest => (),
                }
            }
            measure_contents.push((this, any));
        }
        out.push(measure_contents);
    }
    out
}

pub fn parse(s: &str) -> Result<Parsed, String> {
    match pre_parse(s) {
        Ok((start, mid, end)) => Ok(Parsed {
            start,
            this_any: split_requests(mid),
            end,
        }),
        Err(x) => Err(x),
    }
}
