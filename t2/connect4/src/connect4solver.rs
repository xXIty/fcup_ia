//
// Custom modules import
// ======================
//
use crate::connect4::State;
use crate::connect4::Player;
use crate::treenode::Node;
use crate::treenode::NodeLink;
use std::io;
use rand::Rng;


use std::cmp::{max,min};

pub fn get_solver(s: &str, depth: u32, iter: u32) -> Box<dyn Solver> {
    match s {
        "INTERACTIVE"  =>  Box::new(Interactive  {                 }),
        "MINMAX"       =>  Box::new(Minimax      {  depth:  depth  }),
        "ALPHA-BETA"   =>  Box::new(AlphaBeta    {  depth:  depth  }),
        "MCTS"         =>  Box::new(MCTS         {  depth:  depth,
                                                    iter:   iter}),
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
        //println!("DEPTH: {} action {:x}",self.depth, action);
        s.result(action);
    }
}

pub struct MCTS {
    pub depth: u32,
    pub iter: u32,
}
impl Solver for MCTS {
    fn play(&self, s: &mut State) {
        let root: NodeLink = Node::new(99, 0.0, 0.0, s.actions());
        let action = mcts_run(&root, s, self.depth, self.iter);
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
        if v <= alpha {
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
                let min = alpha_beta_min_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
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
                let max = alpha_beta_max_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
                if max < v {
                   v = max;
                   action = a;
                }
                
            }
        }
    }
    return action;
}

fn mcts_run(root: &NodeLink, s: &State, depth: u32, iter: u32) ->  usize {

    let mut state = s.clone();

    for i in 0..iter {
        println!("ITER: {}", i);
        mcts_iter(root, &mut state, depth);
    }
    let node = &mcts_selection(root);
    return node.borrow().data.action;

}

fn mcts_iter(node: &NodeLink, s: &mut State, depth: u32) {

    let mut leaf: bool = node.borrow().is_leaf();
    
    // Selection
    println!("SELECTION");
    while !leaf {
        let node = &mcts_selection(node);
        println!("sefdfdl: {}",node.borrow().data.unexpanded.len());
        s.result(node.borrow().data.action);
        leaf = node.borrow().is_leaf();
    }


    // Expand
    println!("EXPAND");
    let node: NodeLink = mcts_expand(node, s);

    // Rollout
    println!("ROLLOUTt");
    let v: i32 = mcts_rollout(s, depth);

    println!("BACKPROP");
    mcts_backpropagate(&node, v);

}


fn mcts_selection(node: &NodeLink) -> NodeLink {
    let mut max: f32 = f32::MIN;
    let mut next: usize = 0;
    let node_v = node.borrow();
    for i in 0..node_v.children.len() {
        let ucb = ucb(&node_v.children[i], node_v.data.t);
        if ucb >= max {
            max = ucb;
            next = i;
        }
    };
    return node_v.children[next].clone(); 
}


fn ucb(node: &NodeLink, t:f32) -> f32 {
    let node_v = node.borrow();

    let reward: f32 = node_v.data.t / node_v.data.n;
    let constant: f32 = f32::sqrt(2.0);

    let f: f32 = f32::sqrt(f32::ln(t/node_v.data.n as f32));

    return reward + constant * f;

}

fn mcts_random_expand(node: &NodeLink) -> usize {
    let mut node_v = node.borrow_mut();
    let lasts = node_v.data.unexpanded.len();
    println!("QUEDEN: {}", lasts);
    let randi;
    if lasts == 1 {
        randi = 0;
    } else {
        randi = rand::thread_rng().gen_range(0..node_v.data.unexpanded.len());
    }
    let a = node_v.data.unexpanded[randi];
    node_v.data.unexpanded.remove(randi);

    return a;

}


fn mcts_expand(node: &NodeLink, s: &mut State) -> NodeLink {


    let a = mcts_random_expand(node);


    s.result(a);

    let new_node = Node::new(a, 0.0, 0.0, s.actions());

    let new_node = Node::add_child(node, new_node);
     
    return new_node;
}

fn mcts_rollout(state: &mut State, depth: u32) -> i32 {
    match state.get_player() {
        Player::MAX => {
            alpha_beta_min_value(state, depth - 1, i32::MIN, i32::MAX)
        }
        Player::MIN => {
            alpha_beta_max_value(state, depth - 1, i32::MIN, i32::MAX)
        }
    }
}


fn mcts_backpropagate(node: &NodeLink, v: i32) {
    {
    let node_v = node.borrow();
    match &node_v.parent { 
        None => {},
        Some(x) => {
            let parent = x.upgrade().unwrap();
            mcts_backpropagate(&parent, v);
            
        }
    }; 
    }
    let mut node_v = node.borrow_mut();
    node_v.data.n += 1.0; 
    node_v.data.t += v as f32; 
}

