//
// Module name: connect_four
// Description: Implements the internals of the Connect Four game.
//

use std::fmt;
use enum_map::{enum_map, Enum, EnumMap};

// Standard Board size
const  BOARD_WIDTH:  usize    =  7;                                      
const  BOARD_HIGHT:  usize    =  6;                                      
const  BOARD_SIZE:   usize    =  BOARD_HIGHT*BOARD_WIDTH;                

// Constants to do the state evaluation.
// Based on R. L. Rivest, Game Tree Searching by Min/Max Approximation, AI 34 [1988], pp. 77-96
const  EVAL_TURN:   [i32;  2]  =  [16, -16];            
const  EVAL_SEG_4:  [i32;  5]  =  [0, 1, 10, 50, 512];
const  EVAL_WIN:    i32        =  EVAL_SEG_4[4];

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
            heuristic: Some(0),
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

        // Store exclusive MAX or MIN apparitions
        let  mut  type_diag_down   =  SlotType::EMPTY;
        let  mut  type_diag_up     =  SlotType::EMPTY;
        let  mut  type_horizontal  =  SlotType::EMPTY;
        let  mut  type_vertical    =  SlotType::EMPTY;

        // Store number of MAX or MIN apparitions
        let  mut  count_diag_down   :usize  =  0;
        let  mut  count_diag_up     :usize  =  0;
        let  mut  count_horizontal  :usize  =  0;
        let  mut  count_vertical    :usize  =  0;

        // Check for horizontal 4-segment (right direction) and count MAX's or MIN's.
        if col < BOARD_WIDTH-3 {
            for new_col in col..col+4 {
                // Try to start counting MAX's or MIN's.
                if type_horizontal == SlotType::EMPTY {
                    type_horizontal   = self.board[row][new_col];
                    count_horizontal += (self.board[row][new_col] != SlotType::EMPTY) as usize;
                }
                // Increase MAX's or MIN's counter.
                else if type_horizontal == self.board[row][new_col] {
                    count_horizontal += 1;
                }
                // MAX and MIN's are mixed => segment without value.
                else if type_horizontal != self.board[row][new_col] {
                    count_horizontal = 0;
                    break;
                }
                // EMPTY slot.
                else {
                    continue;
                }
            }
        }
        
        // Check for vertical 4-segment (up direction) and count MAX's or MIN's.
        if row > 3 {
            for new_row in row..row-4 {
                // Try to start counting MAX's or MIN's.
                if type_vertical == SlotType::EMPTY {
                    count_vertical += (self.board[new_row][col] != SlotType::EMPTY) as usize;
                    type_vertical   = self.board[new_row][col];
                }
                // Increase MAX's or MIN's counter.
                else if type_vertical == self.board[new_row][col] {
                    count_vertical += 1;
                }
                // MAX and MIN's are mixed => segment without value.
                else if type_vertical != self.board[new_row][col] {
                    count_vertical = 0;
                    break;
                }
                // EMPTY slot.
                else {
                    continue;
                }
            }
        }

        // Check for up-diagonal 4-segment (right direction) and count MAX's or MIN's.
        if row > 3 && col < BOARD_WIDTH-3 {
            for slot_incr in 0..4 {

                let new_row = row - slot_incr;
                let new_col = col + slot_incr;

                // Try to start counting MAX's or MIN's.
                if type_diag_up == SlotType::EMPTY {
                    count_diag_up += (self.board[new_row][new_col] != SlotType::EMPTY) as usize;
                    type_diag_up   = self.board[new_row][new_col];
                }
                // Increase MAX's or MIN's counter.
                else if type_diag_up == self.board[new_row][new_col] {
                    count_diag_up += 1;
                }
                // MAX and MIN's are mixed => segment without value.
                else if type_diag_up != self.board[new_row][new_col] {
                    count_diag_up = 0;
                    break;
                }
                // EMPTY slot.
                else {
                    continue;
                }
            }
        }

        // Check for down-diagonal 4-segment (right direction) and count MAX's or MIN's.
        if row < BOARD_HIGHT-3 && col < BOARD_WIDTH-3 {
            for slot_incr in 0..4 {

                let new_row = row + slot_incr;
                let new_col = col + slot_incr;

                // Try to start counting MAX's or MIN's.
                if type_diag_down == SlotType::EMPTY {
                    count_diag_down += (self.board[new_row][new_col] != SlotType::EMPTY) as usize;
                    type_diag_down   = self.board[new_row][new_col];
                }
                // Increase MAX's or MIN's counter.
                else if type_diag_down == self.board[new_row][new_col] {
                    count_diag_down += 1;
                }
                // MAX and MIN's are mixed => segment without value.
                else if type_diag_down != self.board[new_row][new_col] {
                    count_diag_down = 0;
                    break;
                }
                // EMPTY slot.
                else {
                    continue;
                }
            }
        }
        return EVAL_SEG_4  [  count_horizontal  ]  +
               EVAL_SEG_4  [  count_vertical    ]  +
               EVAL_SEG_4  [  count_diag_down   ]  +
               EVAL_SEG_4  [  count_diag_up     ]  ;
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
        output.push('\n');

        // Print heuristic
        if self.heuristic != None {
            output.push_str("Heuristic of position: ".into());
            output.push_str(&self.heuristic.unwrap().to_string());
        }

        // Return the output built
        write!(f,"{}",output)
    }
}
