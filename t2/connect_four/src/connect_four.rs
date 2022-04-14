//
// Module name: connect_four
// Description: Implements the internals of the Connect Four game.
//

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
