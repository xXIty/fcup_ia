//
// Custom modules import
// ======================
//
use crate::connect4::State;

// Input current state, return next state
pub fn minimax_decision(state: &mut State, depth: i32) -> i32 {
    let v = max_value(state, depth);
    return v;

}

fn max_value(state: &State, depth: i32) -> i32 {
    if state.is_terminal(depth) {
        return state.get_utility();
    }
    let mut v: i32 = i32::MIN;
    let mut v_vec: Vec<i32> = Vec::new();
    for s in state.successors() {
        let min = min_value(&s, depth);
        v = std::cmp::max(v, min);
        v_vec.push(min);
    }
    println!("MAX_VALUE FOR STATE: depth {}\n{}",state.depth, &state);
    for min in v_vec {
        print!("{} ", min);
    }
    print!("\n");
    println!("MAX_VALUE -> {}", v);
    return v;
}

fn min_value(state: &State, depth: i32) -> i32 {
    if state.is_terminal(depth) {
        return state.get_utility();
    }
    let mut v: i32 = i32::MAX;
    let mut v_vec: Vec<i32> = Vec::new();
    for s in state.successors() {
        let max = max_value(&s, depth);
        v = std::cmp::min(v, max);
        v_vec.push(max);
    }
    println!("MIN_VALUE FOR STATE: depth {}\n{}",state.depth, &state);
    for max in v_vec {
        print!("{} ", max);
    }
    print!("\n");
    println!("MIN_VALUE -> {}", v);
    return v;

}



