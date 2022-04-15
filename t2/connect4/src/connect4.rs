//
// Module name: connect_four
// Description: Implements the internals of the Connect Four game.
//

use std::fmt;
use dict::{Dict, DictIface};
use enum_map::{enum_map, Enum, EnumMap};

// Standard Board size
const  BOARD_WIDTH:  usize    =  7;                                      
const  BOARD_HIGHT:  usize    =  6;                                      
const  BOARD_SIZE:   usize    =  BOARD_HIGHT*BOARD_WIDTH;                

// Constants to do the state evaluation.
// Based on R. L. Rivest, Game Tree Searching by Min/Max Approximation, AI 34 [1988], pp. 77-96
const  EVAL_SEG_4:  [i32;  5]  =  [0, 1, 10, 50, 512];
const  EVAL_WIN:    i32        =  EVAL_SEG_4[4];
const  EVAL_TURN:   [i32;  2]  =  [16, -16];            


#[derive(Copy, Clone)]
enum Player {
    MAX = 0,
    MIN = 1,
}

#[derive(Enum, PartialEq, Copy, Clone)]
enum SlotType {
    MAX,
    MIN,
    EMPTY,
}

pub struct State {
    turn:  Player,
    board: [[SlotType; BOARD_WIDTH]; BOARD_HIGHT],
    heuristic: Option<i32>,
}

impl State {

    // Returns a game without any move done.
    pub fn new() -> State {
        State {
            turn:  Player::MAX,
            board: [[SlotType::EMPTY; BOARD_WIDTH]; BOARD_HIGHT],
            heuristic: None,
        }
    }
    
    // Calculates the heuristic of the state.
    pub fn get_heuristic(&mut self) -> i32 {

        if self.heuristic == None {
            self.set_heuristic();
        }

        return self.heuristic.unwrap();
    }

    fn set_heuristic(&mut self) {
        
        // Initialize evaluation with bonus from whos turn is.
        let mut eval: i32 = EVAL_TURN[self.turn as usize];

        // Check each segment of 4 slots
        'eval_state: for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_HIGHT {
                let eval_slot = self.eval_segments_of_4(row, col);
                if eval_slot == -EVAL_SEG_4[4] || eval_slot == EVAL_SEG_4[4] {
                    eval = eval_slot;
                    break 'eval_state;
                }
                else {
                    eval += eval_slot;
                }
            }
        }

        self.heuristic = Some(eval);
    }

    // Returns the evaluation of all possible segments of 4 slots from a given slot
    fn eval_segments_of_4(&self, row: usize, col: usize) -> i32 {
        let     horizontal_count = 0;
        let mut horizontal_type  = self.board[row][col];

        horizontal_count += (self.board[row][col] != SlotType::EMPTY) as i32;

        // Check if slot starts horizontal 4-segment
        if row < BOARD_WIDTH-3 {
            for i in 1..4 {
//                match horizontal_type {
//                    SlotType::EMPTY => {
//                    }
//                    - =>
            }
        }

        return 0;
    }
        
    // Returns true if the game is over.
    pub fn is_terminal(&mut self)  ->  bool  {
        if self.heuristic == None {
            self.set_heuristic();
        }
        return self.heuristic == Some(EVAL_WIN) || self.heuristic == Some(-EVAL_WIN);
    }

    // Returns the winner of the state.
    pub  fn  utility(&self)      ->  i32   {return 0;}

    // No se com retornar una llista de state amb tamany indeterminat 
    // pub  fn  successors(&self)   ->  
}

impl fmt::Display for State {


    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Choose which simbols to use for each player
        let slot_type_map = enum_map! {
            SlotType::MIN    =>  'X',
            SlotType::MAX    =>  'O',
            SlotType::EMPTY  =>  ' ',
        };

        // Empty output
        let mut output = String::new();

        // Build the board... preaty basic ;)
        for row in 0..BOARD_HIGHT {
            output.push_str(&"|");
            for column in 0..BOARD_WIDTH {
                output.push(slot_type_map[self.board[row][column]]);
            }
            output.push_str(&"|\n");
        }
        output.push_str(&" -------\n");

        // Print column numbers
        output.push_str(&" ");
        for row in 0..BOARD_WIDTH {
            output.push_str(&(row+1).to_string());
        }

        // Return the output built
        write!(f,"{}",output)
    }
}
