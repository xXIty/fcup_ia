//
// Custom modules import
// ======================
//
use crate::connect4::State;
use crate::connect4::Player;

// Input current state, return next state
pub fn minimax_recursive(state: &mut State, depth: i32) -> i32 {
    if depth == 0 || state.is_terminal() {
        return state.get_utility();
    }
    let mut v: i32;
    match state.get_player() {
        Player::MAX => {
            v = i32::MIN;
            for a in state.actions() {
                let mut new_state = state.clone();
                new_state.result(a);
                let min = minimax_recursive(&mut new_state, depth - 1);
                v = std::cmp::max(v, min);
            }
        }
        Player::MIN => {
            v = i32::MAX;
            for a in state.actions() {
                let mut new_state = state.clone();
                new_state.result(a);
                let max = minimax_recursive(&mut new_state, depth - 1);
                v = std::cmp::min(v, max);
            }
        }
    }
    return v;

}

pub fn minimax_solver(state: &mut State, depth: i32) -> usize {
    let mut action: usize = 0xdeadbeaf;
    let mut v: i32;
    match state.get_player() {
        Player::MAX => {
            v = i32::MIN;
            for a in state.actions() {
                let mut new_state = state.clone();
                new_state.result(a);
                let min = minimax_recursive(&mut new_state, depth - 1);
                println!("min aux -> {}", min);
                if min > v {
                   v = min;
                   action = a;
                }
            }
        }
        Player::MIN => {
            v = i32::MAX;
            for a in state.actions() {
                let mut new_state = state.clone();
                new_state.result(a);
                let max = minimax_recursive(&mut new_state, depth - 1);
                println!("max aux -> {}", max);
                if max < v {
                   v = max;
                   action = a;
                }
                
            }
        }
    }
    return action;
}












