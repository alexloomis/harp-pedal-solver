use crate::base::*;

#[derive(Clone)]
pub struct State {
    pub left: Option<Name>,
    pub right: Option<Name>,
    pub harp: Pedals,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub new_pedal: Name,
    pub new_pitch: Option<Height>,
}

#[derive(Clone)]
pub struct Plan {
    pub instructions: Vec<(Option<Instruction>, Option<Instruction>)>,
}

// Can make state a mutable reference
fn unverified_update_state(
    state: State,
    l_instr: Option<Instruction>,
    r_instr: Option<Instruction>,
) -> State {
    let mut new_harp = state.harp;
    let new_left = l_instr.map_or(state.left, |x| Some(x.new_pedal));
    let new_right = r_instr.map_or(state.right, |x| Some(x.new_pedal));
    l_instr.and_then(|u| {
        u.new_pitch.and_then(|v| new_harp.insert(u.new_pedal, v))
    });
    r_instr.and_then(|u| {
        u.new_pitch.and_then(|v| new_harp.insert(u.new_pedal, v))
    });
    State {
        left: new_left,
        right: new_right,
        harp: new_harp,
    }
}
