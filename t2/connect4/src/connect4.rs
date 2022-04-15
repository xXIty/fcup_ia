//
// Module name: connect_four
// Description: Implements the internals of the Connect Four game.
//

use std::fmt;

const BOARD_WIDTH: usize = 7;
const BOARD_HIGHT: usize = 6;
const BOARD_SIZE:  usize = BOARD_HIGHT*BOARD_WIDTH;

enum Player {
    MAX,
    MIN,
}

pub struct State {
    // - Game board
    // - Player turn
    turn:  Player,
    board: [char; BOARD_SIZE],
}

impl State {

    // Returns a game without any move done.
    pub fn new() -> State {
        State {
            turn:  Player::MAX,
            board: ['-'; BOARD_SIZE],
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
        return 0;
    }

    // No se com retornar una llista de state amb tamany indeterminat 
    // pub  fn  successors(&self)   ->  
}

impl fmt::Display for State {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Empty output
        let mut output = String::new();

        // Build the board... preaty basic ;)
        for row in 0..BOARD_HIGHT {
            output.push_str(&"|");
            for column in 0..BOARD_WIDTH {
                let mut pos = ' ';
                if self.board[row*BOARD_HIGHT+column] != '-' {
                    let pos = self.board[row*BOARD_HIGHT+column];
                }
                output.push(pos);
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
