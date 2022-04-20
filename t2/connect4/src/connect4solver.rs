//
// Custom modules import
// ======================
//
use crate::connect4::State;
use crate::connect4::Player;

// Input current state, return next state
pub fn minimax_decision(state: &mut State, depth: i32) -> i32 {
    match state.get_player() {
        Player::MAX => max_value(state, depth),
        Player::MIN => min_value(state, depth),
    }

}

fn max_value(state: &State, depth: i32) -> i32 {
    if depth == 0 || state.is_terminal() {
        return state.get_utility();
    }
    let mut v: i32 = i32::MIN;
    for s in state.successors() {
        let min = min_value(&s,  depth - 1);
        v = std::cmp::max(v, min);
    }
    return v;
}

fn min_value(state: &State, depth: i32) -> i32 {
    if depth == 0 || state.is_terminal() {
        return state.get_utility();
    }
    let mut v: i32 = i32::MAX;
    for s in state.successors() {
        let max = max_value(&s,  depth - 1);
        v = std::cmp::min(v, max);
    }
    return v;

}



