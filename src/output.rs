use itertools::Itertools;

// move to prelude
use crate::parse::Measure;
use crate::prelude::*;

fn pedal_markup(diagram: Harp) -> String {
    let mut out = String::with_capacity(35);
    out.push_str("<>^\\markup { \\harp-pedal \"");
    out.push_str(&pedal_diagram(diagram));
    out.push_str("\" }");
    out
}

fn add_measure(contents: &mut Vec<String>, measure: Measure) {
    let mut new_line = String::from("");
    for beat in measure {
        let len = beat.len();
        if len > 1 {
            new_line.push_str("<< ");
        }
        for note in beat {
            new_line.push_str(&note.name.to_string().to_lowercase());
            match note.accidental {
                Accidental::Flat => {
                    new_line.push_str("es ");
                }
                Accidental::Natural => {
                    new_line.push(' ');
                }
                Accidental::Sharp => {
                    new_line.push_str("is ");
                }
            }
        }
        if len > 1 {
            new_line.push_str(">> ");
        } else if len == 0 {
            new_line.push_str("r ");
        }
    }
    new_line.push_str("\\bar \"|\"");
    contents.push(new_line)
}

fn make_ly_treble(treble: Vec<Measure>, start: Harp, end: Harp) -> String {
    let mut lines: Vec<String> = vec![
        "treble = \\fixed c' {".to_string(),
        "    \\clef \"treble\" \\key c \\major".to_string(),
        "    \\override Staff.TimeSignature.stencil = ##f".to_string(),
        "    \\cadenzaOn".to_string(),
        pedal_markup(start),
    ];
    for measure in treble {
        add_measure(&mut lines, measure);
    }
    lines.push(pedal_markup(end));
    lines.push("}".to_string());
    lines.join("\n")
}

fn make_ly_pedals_l(changes: Vec<Vec<Note>>) -> String {
    let mut out = "pedalsL = { ".to_string();
    for beat in changes {
        out.push_str("s ");
        for change in beat {
            out.push_str("_\"");
            out.push_str(&change.to_string());
            out.push_str("\" ");
        }
    }
    out.push('}');
    out
}

fn make_ly_pedals_r(changes: Vec<Vec<Note>>) -> String {
    let mut out = "pedalsR = { ".to_string();
    for beat in changes {
        out.push_str("s ");
        for change in beat {
            out.push_str("_\"");
            out.push_str(&change.to_string());
            out.push_str("\" ");
        }
    }
    out.push('}');
    out
}

pub fn make_ly_file_(
    treble: Vec<Measure>,
    start: Harp,
    end: Harp,
    changes: &Pedals,
) -> String {
    let (lefts, rights) = unzip_pedals(changes);
    let mut lines: Vec<String> =
        vec!["\\version \"2.22.0\"",
        "",
        "\\header {",
        "    tagline = ##f",
        "}",
        "", 
        "\\layout {",
        "    \\context {",
        "        \\Dynamics",
        "        \\override TextScript.font-shape = #'upright",
        "        \\override VerticalAxisGroup.nonstaff-nonstaff-spacing = #'((minimum-distance . 2.5))",
        "    }",
        "}",
        "",].iter().map(|s| s.to_string()).collect_vec();
    lines.push(make_ly_treble(treble, start, end));
    lines.push("".to_string());
    lines.push(make_ly_pedals_r(rights));
    lines.push("".to_string());
    lines.push(make_ly_pedals_l(lefts));
    lines.push("".to_string());
    lines.push("\\new Staff <<".to_string());
    lines.push("    \\new Voice \\treble".to_string());
    lines.push("    \\new Dynamics \\pedalsR".to_string());
    lines.push("    \\new Dynamics \\pedalsL".to_string());
    lines.push(">>".to_string());
    lines.join("\n")
}
