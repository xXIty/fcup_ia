//
// Module name: connect_four
// Description: Implements the internals of the Connect Four game.
//

use std::fmt;
use enum_map::{enum_map, Enum, EnumMap};

const BOARD_WIDTH: usize = 7;
const BOARD_HIGHT: usize = 6;
const BOARD_SIZE:  usize = BOARD_HIGHT*BOARD_WIDTH;

enum Player {
    MAX,
    MIN,
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
}

impl State {

    // Returns a game without any move done.
    pub fn new() -> State {
        State {
            turn:  Player::MAX,
            board: [[SlotType::EMPTY; BOARD_WIDTH]; BOARD_HIGHT],
        }
    }
        
    // Returns true if the game is over.
    pub  fn  is_terminal(&self)  ->  bool  {
        return self.heuristic() == 512 || self.heuristic() == -512
    }

    // Returns the winner of the state.
    pub  fn  utility(&self)      ->  i32   {return 0;}

    // Returns the heuristic value of the state.
    pub fn heuristic(&self) -> i32 {
        let mut eval: i32 = 0;
        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_HIGHT {
            }
        }
        return 0;
    }

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
