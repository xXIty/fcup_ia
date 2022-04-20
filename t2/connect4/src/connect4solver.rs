//
// Custom modules import
// ======================
//
use crate::connect4::State;
use crate::connect4::Player;
use std::io;
use std::cmp::{max,min};

pub fn get_solver(s: &str, depth: u32) -> Box<dyn Solver> {
    match s {
        "INTERACTIVE"  =>  Box::new(Interactive  {                 }),
        "MINMAX"       =>  Box::new(Minimax      {  depth:  depth  }),
        "ALPHA-BETA"   =>  Box::new(AlphaBeta    {  depth:  depth  }),
        _              =>  Box::new(Interactive  {                 }),
    }
}

pub trait Solver {
    fn play(&self, s: &mut State);
}

pub struct Minimax {
    pub depth: u32,
}
impl Solver for Minimax {
    fn play(&self, s: &mut State) {
        let action = minimax_solver(s, self.depth);
        println!("DEPTH: {} action {:x}",self.depth, action);
        s.result(action);
    }
}

pub struct AlphaBeta {
    pub depth: u32,
}
impl Solver for AlphaBeta {
    fn play(&self, s: &mut State) {
        let action = alpha_beta(s, self.depth);
        println!("DEPTH: {} action {:x}",self.depth, action);
        s.result(action);
    }
}

pub struct Interactive {}
impl Solver for Interactive {
    fn play(&self, s: &mut State) {
        let  mut  move_valid  =  false;
        
        while !move_valid {

            // Read column to drop token
            let  mut  action =  String::new();
            println!("Enter column number where you want to drop your token:");
            io::stdin().read_line(&mut action)
                .expect("Failed to read line.");

            // Try to convert column number to usize
            let action: usize = match action.trim().parse() {
                Ok(num) => num,
                Err(_)  => continue,
            };

            // Try to make a move over the state
            move_valid = s.result(action);
        }
    }
}

//#[derive(Debug)]
//pub enum Algorithm {
//    User,
//    Minimax(i32),
//    AlphaBeta,
//    MCTS,
//}

//impl Algorithm {
//
//    pub fn decide_and_run(&self, s: &mut State) {
//        println!("algo:  {:?}", *self);
//        match self {
//            Player::User => {
//                let  mut  move_valid  =  false;
//                
//                while !move_valid {
//
//                    // Read column to drop token
//                    let  mut  action =  String::new();
//                    println!("Enter column number where you want to drop your token:");
//                    io::stdin().read_line(&mut action)
//                        .expect("Failed to read line.");
//
//                    // Try to convert column number to usize
//                    let action: usize = match action.trim().parse() {
//                        Ok(num) => num,
//                        Err(_)  => continue,
//                    };
//
//                    // Try to make a move over the state
//                    move_valid = s.result(action);
//                }
//            }
//            Player::Minimax(depth) => {
//                let action = minimax_solver(s, *depth);
//                println!("DEPTH: {} action {:x}",depth, action);
//                s.result(action);
//            }
//            Player::AlphaBeta => { 
//                println!("not implemented");
//                s.result(0);
//            }
//
//            Player::MCTS => { 
//                println!("not implemented");
//                s.result(0);
//            }
//        }
//
//
//    }
//
//}

// Input current state, return next state
pub fn minimax_recursive(state: &mut State, depth: u32) -> i32 {
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
                v = max(v, min);
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

pub fn minimax_solver(state: &mut State, depth: u32) -> usize {
    let mut action: usize = 0xdeadbeaf;
    let mut v: i32;
    match state.get_player() {
        Player::MAX => {
            v = i32::MIN;
            for a in state.actions() {
                let mut new_state = state.clone();
                new_state.result(a);
                let min = minimax_recursive(&mut new_state, depth - 1);
                //println!("min aux -> {}", min);
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
                //println!("max aux -> {}", max);
                if max < v {
                   v = max;
                   action = a;
                }
                
            }
        }
    }
    return action;
}

fn alpha_beta_max_value(state: &mut State, depth:u32, mut alpha: i32, beta:i32) -> i32 {
    if depth == 0 || state.is_terminal() {
        return state.get_utility();
    }
    let mut v = i32::MIN;
    for a in state.actions() {
        let mut new_state = state.clone();
        new_state.result(a);
        v = max(v, alpha_beta_min_value(&mut new_state, depth - 1, alpha, beta));
        if v >= beta {
            return v;
        }
        alpha = max(v, alpha);
    }
    return v;
}

fn alpha_beta_min_value(state: &mut State, depth:u32, alpha: i32, mut beta:i32) -> i32 {
    if depth == 0 || state.is_terminal() {
        return state.get_utility();
    }
    let mut v = i32::MAX;
    for a in state.actions() {
        let mut new_state = state.clone();
        new_state.result(a);
        v = min(v, alpha_beta_max_value(&mut new_state, depth - 1, alpha, beta));
        if v >= beta {
            return v;
        }
        beta = min(beta, v);
    }
    return v;
}

pub fn alpha_beta(state: &mut State, depth:u32) -> usize {
    let mut action: usize = 0xdeadbeaf;
    let mut v: i32;
    match state.get_player() {
        Player::MAX => {
            v = i32::MIN;
            for a in state.actions() {
                let mut new_state = state.clone();
                new_state.result(a);
                let min = alpha_beta_max_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
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
                let max = alpha_beta_min_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
                if max < v {
                   v = max;
                   action = a;
                }
                
            }
        }
    }
    return action;
}

