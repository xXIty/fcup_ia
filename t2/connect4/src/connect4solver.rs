//
// Custom modules import
// ======================
//
use crate::connect4::State;
use crate::connect4::Player;
use crate::treenode::Node;
use crate::treenode::NodeLink;
use std::io;
use std::cmp::{max, min};
use rand::Rng;

pub fn get_solver(s: &str, depth: u32, iter: u32) -> Box<dyn Solver> {
    match s {
        "INTERACTIVE"     =>  Box::new(Interactive    {  nodes_expanded: 0,               }),
        "MINMAX"          =>  Box::new(Minimax        {  nodes_expanded: 0, depth,        }),
        "ALPHA-BETA"      =>  Box::new(AlphaBeta      {  nodes_expanded: 0, depth,        }),
        "ALPHA-BETA-ORD"  =>  Box::new(AlphaBetaWOrd  {  nodes_expanded: 0, depth,        }),
        "MCTS"            =>  Box::new(MCTS           {  nodes_expanded: 0, depth, iter,  }),
        _                 =>  Box::new(Interactive    {  nodes_expanded: 0,               }),
    }
}

pub trait Solver {

    fn play(&mut self, s: &mut State);

    fn get_nodes_expanded(&self) -> u128;
}

pub struct Minimax {
    pub depth: u32,
    nodes_expanded: u128,
}
impl Solver for Minimax {

    fn get_nodes_expanded(&self) -> u128 { self.nodes_expanded }

    fn play(&mut self, s: &mut State) {
        let action = self.minimax_solver(s, self.depth);
        //println!("DEPTH: {} action {:x}",self.depth, action);
        s.result(action);
    }
}

impl Minimax {

    pub fn minimax_solver(&mut self, state: &mut State, depth: u32) -> usize {
        let mut action: usize = 0xdeadbeaf;
        let mut v: i32;
        match state.get_player() {
            Player::MAX => {
                v = i32::MIN;
                for a in state.actions() {
                    let mut new_state = state.clone();
                    new_state.result(a);
                    let min = self.minimax_recursive(&mut new_state, depth - 1);
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
                    let max = self.minimax_recursive(&mut new_state, depth - 1);
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

    // Input current state, return next state
    pub fn minimax_recursive(&mut self, state: &mut State, depth: u32) -> i32 {
        self.nodes_expanded += 1;
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
                    let min = self.minimax_recursive(&mut new_state, depth - 1);
                    v = max(v, min);
                }
            }
            Player::MIN => {
                v = i32::MAX;
                for a in state.actions() {
                    let mut new_state = state.clone();
                    new_state.result(a);
                    let max = self.minimax_recursive(&mut new_state, depth - 1);
                    v = std::cmp::min(v, max);
                }
            }
        }
        return v;
    }
}

pub struct AlphaBeta {
    pub depth: u32,
    nodes_expanded: u128,
}
impl Solver for AlphaBeta {

    fn get_nodes_expanded(&self) -> u128 { self.nodes_expanded }

    fn play(&mut self, s: &mut State) {
        let action = self.alpha_beta(s, self.depth);
        s.result(action);
    }
}

impl AlphaBeta {

    pub fn alpha_beta(&mut self, state: &mut State, depth:u32) -> usize {
        let mut action: usize = 0xdeadbeaf;
        let mut v: i32;
        match state.get_player() {
            Player::MAX => {
                v = i32::MIN;
                for a in state.actions() {
                    let mut new_state = state.clone();
                    new_state.result(a);
                    let min = self.alpha_beta_min_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
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
                    let max = self.alpha_beta_max_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
                    if max < v {
                       v = max;
                       action = a;
                    }
                    
                }
            }
        }
        return action;
    }

    fn alpha_beta_max_value(&mut self, state: &mut State, depth:u32, mut alpha: i32, beta:i32) -> i32 {
        self.nodes_expanded += 1;
        if depth == 0 || state.is_terminal() {
            return state.get_utility();
        }
        let mut v = i32::MIN;
        for a in state.actions() {
            let mut new_state = state.clone();
            new_state.result(a);
            v = max(v, self.alpha_beta_min_value(&mut new_state, depth - 1, alpha, beta));
            if v >= beta {
                return v;
            }
            alpha = max(v, alpha);
        }
        return v;
    }

    fn alpha_beta_min_value(&mut self, state: &mut State, depth:u32, alpha: i32, mut beta:i32) -> i32 {
        self.nodes_expanded += 1;
        if depth == 0 || state.is_terminal() {
            return state.get_utility();
        }
        let mut v = i32::MAX;
        for a in state.actions() {
            let mut new_state = state.clone();
            new_state.result(a);
            v = min(v, self.alpha_beta_max_value(&mut new_state, depth - 1, alpha, beta));
            if v <= alpha {
                return v;
            }
            beta = min(beta, v);
        }
        return v;
    }
}

pub struct AlphaBetaWOrd {
    pub depth: u32,
    nodes_expanded: u128,
}
impl Solver for AlphaBetaWOrd {

    fn get_nodes_expanded(&self) -> u128 { self.nodes_expanded }

    fn play(&mut self, s: &mut State) {
        let action = self.alpha_beta_w_ordering(s, self.depth);
        s.result(action);
    }
}

impl AlphaBetaWOrd {
    pub fn alpha_beta_w_ordering(&mut self, state: &mut State, depth:u32) -> usize {
        self.nodes_expanded += 1;
        let mut action: usize = 0xdeadbeaf;
        let mut v: i32;
        match state.get_player() {
            Player::MAX => {
                v = i32::MIN;
                for a in state.actions() {
                    let mut new_state = state.clone();
                    new_state.result(a);
                    let min = self.alpha_beta_w_ordering_min_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
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
                    let max = self.alpha_beta_w_ordering_max_value(&mut new_state, depth - 1, i32::MIN, i32::MAX);
                    if max < v {
                       v = max;
                       action = a;
                    }
                    
                }
            }
        }
        return action;
    }

    fn alpha_beta_w_ordering_max_value(&mut self, state: &mut State, depth:u32, mut alpha: i32, beta:i32) -> i32 {
        self.nodes_expanded += 1;
        if depth == 0 || state.is_terminal() {
            return state.get_utility();
        }
        let mut v = i32::MIN;
        
        // Generate all successor states
        let mut states_unordered = Vec::new();
        for a in state.actions() {
            let mut new_state = state.clone();
            new_state.result(a);
            states_unordered.push(new_state); 
        }
        // Order
        let mut states_ordered = State::order_list(states_unordered);
        states_ordered.reverse();

        // Do actuall algorithm
        for mut state in states_ordered {
            v = max(v, self.alpha_beta_w_ordering_min_value(&mut state, depth - 1, alpha, beta));
            if v >= beta {
                return v;
            }
            alpha = max(v, alpha);
        }
        return v;
    }

    fn alpha_beta_w_ordering_min_value(&mut self, state: &mut State, depth:u32, alpha: i32, mut beta:i32) -> i32 {
        self.nodes_expanded += 1;
        if depth == 0 || state.is_terminal() {
            return state.get_utility();
        }

        // Generate all successor states
        let mut states_unordered = Vec::new();
        for a in state.actions() {
            let mut new_state = state.clone();
            new_state.result(a);
            states_unordered.push(new_state); 
        }

        // Order
        let states_ordered = State::order_list(states_unordered);

        // Do actuall algorithm
        let mut v = i32::MAX;
        for mut state in states_ordered {
            v = min(v, self.alpha_beta_w_ordering_max_value(&mut state, depth - 1, alpha, beta));
            if v <= alpha {
                return v;
            }
            beta = min(beta, v);
        }
        return v;
    }
}


pub struct Interactive { nodes_expanded: u128, }
impl Solver for Interactive {

    fn get_nodes_expanded(&self) -> u128 { self.nodes_expanded }

    fn play(&mut self, s: &mut State) {
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

pub struct MCTS {
    pub depth: u32,
    pub iter: u32,
    nodes_expanded: u128,
}
impl Solver for MCTS {

    fn get_nodes_expanded(&self) -> u128 { self.nodes_expanded }

    fn play(&mut self, s: &mut State) {
        let root: NodeLink = Node::new(99, 0.0, 0.0, s.actions());
        let action = mcts_run(&root, s, self.depth, self.iter);
        s.result(action);
    }
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
    let mut a_b_w_ord = AlphaBetaWOrd { depth, nodes_expanded: 0, };
    match state.get_player() {
        Player::MAX => {
            a_b_w_ord.alpha_beta_w_ordering_min_value(state, depth - 1, i32::MIN, i32::MAX)
        }
        Player::MIN => {
            a_b_w_ord.alpha_beta_w_ordering_max_value(state, depth - 1, i32::MIN, i32::MAX)
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

