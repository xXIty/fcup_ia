//
// Module name: connect_four
// Description: Implements the internals of the Connect Four game.
//

use std::fmt;
use std::collections::HashMap;

// Standard Board size
const  BOARD_WIDTH:  usize    =  7;                                      
const  BOARD_HIGHT:  usize    =  6;                                      
const  BOARD_SIZE:   usize    =  BOARD_HIGHT*BOARD_WIDTH;                

// Constants to do the state evaluation.
// Based on R. L. Rivest, Game Tree Searching by Min/Max Approximation, AI 34 [1988], pp. 77-96
const  HEURISTIC_TURN:   i32     =  16;            
const  HEURISTIC_SEG_4: [i32; 5] =  [0, 1, 10, 50, 512];
const  HEURISTIC_WIN:    i32     =  HEURISTIC_SEG_4[4];

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Player {
    MAX =  1,
    MIN = -1,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum CellType {
    Player(Player),
    EMPTY,
}

pub struct State {
    turn:       Player,                      
    board:      [[CellType; BOARD_WIDTH]; BOARD_HIGHT],
    heuristic:  Option<i32>,                 
}

impl State {

    // Returns a game without any move done.
    pub fn new() -> State {
        State {
            turn:       Player::MAX,
            board:      [[CellType::EMPTY; BOARD_WIDTH]; BOARD_HIGHT],
            heuristic:  Some(HEURISTIC_TURN * (Player::MAX as i32)),
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
        let mut heuristic_total: i32 = HEURISTIC_TURN * (self.turn as i32);

        // Check each segment of 4 cells
        'calculate_heuristic: for row in (0..BOARD_HIGHT).rev() {

            // Acumulate heuristics of the row
            let mut heuristic_row: i32 = 0;

            for col in 0..BOARD_WIDTH {

                // Calculate horizontal 4-segment
                let heuristic_cell = self.heuristic_seg_4_horizontal(row, col);
                if heuristic_cell.abs() == HEURISTIC_WIN {
                    heuristic_total = heuristic_cell;
                    break 'calculate_heuristic;
                }
                else {
                    heuristic_row += heuristic_cell;
                }

                // Calculate vertical 4-segment
                let heuristic_cell = self.heuristic_seg_4_vertical(row, col);
                if heuristic_cell.abs() == HEURISTIC_WIN {
                    heuristic_total = heuristic_cell;
                    break 'calculate_heuristic;
                }
                else {
                    heuristic_row += heuristic_cell;
                }

                // Calculate diagonal-up 4-segment
                let heuristic_cell = self.heuristic_seg_4_diag_up(row, col);
                if heuristic_cell.abs() == HEURISTIC_WIN {
                    heuristic_total = heuristic_cell;
                    break 'calculate_heuristic;
                }
                else {
                    heuristic_row += heuristic_cell;
                }

                // Calculate diagonal-down 4-segment
                let heuristic_cell = self.heuristic_seg_4_diag_down(row, col);
                if heuristic_cell.abs() == HEURISTIC_WIN {
                    heuristic_total = heuristic_cell;
                    break 'calculate_heuristic;
                }
                else {
                    heuristic_row += heuristic_cell;
                }
            }

            heuristic_total += heuristic_row;

            // Check if it is necessary to keep moving up in the board
            if heuristic_row == 0 {
                break 'calculate_heuristic;
            }
        }
        self.heuristic = Some(heuristic_total);
    }

    fn heuristic_seg_4_horizontal(&self, row: usize, col: usize) -> i32 {

        let mut type_horizontal = CellType::EMPTY;
        let mut count_horizontal :usize = 0;
        
        // Check for horizontal 4-segment (right direction) and count MAX's or MIN's.
        if col < BOARD_WIDTH-3 {
            for new_col in col..col+4 {

                let cell: CellType = self.board[row][new_col];

                if cell == CellType::EMPTY {
                    continue;
                }
                else if type_horizontal == CellType::EMPTY || cell == type_horizontal {
                    type_horizontal = cell;
                    count_horizontal += 1;
                }
                else {
                    count_horizontal = 0;
                    break;
                }
            }
        }

        let  type_horizontal: i32    =  State::cell_type_to_heuristic(type_horizontal);
        return type_horizontal * HEURISTIC_SEG_4[count_horizontal];
    }

    fn heuristic_seg_4_vertical(&self, row: usize, col: usize) -> i32 {

        let  mut  type_vertical    =  CellType::EMPTY;
        let  mut  count_vertical    :usize  =  0;

        // Check for vertical 4-segment (up direction) and count MAX's or MIN's.
        if row > 3 {
            for new_row in (row-3..=row).rev() {

                let cell: CellType = self.board[new_row][col];

                if cell == CellType::EMPTY {
                    continue;
                }
                else if type_vertical == CellType::EMPTY || cell == type_vertical {
                    type_vertical = cell;
                    count_vertical += 1;
                }
                else {
                    count_vertical = 0;
                    break;
                }
            }
        }
        let  type_vertical: i32  =  State::cell_type_to_heuristic(type_vertical);
        return HEURISTIC_SEG_4[count_vertical] * type_vertical;
    }

    fn heuristic_seg_4_diag_up(&self, row: usize, col: usize) -> i32 {

        let  mut  type_diag_up     =  CellType::EMPTY;
        let  mut  count_diag_up :usize  =  0;

        // Check for up-diagonal 4-segment (right direction) and count MAX's or MIN's.
        if row > 3 && col < BOARD_WIDTH-3 {
            for cell_incr in 0..4 {

                let new_row = row - cell_incr;
                let new_col = col + cell_incr;
                let cell: CellType = self.board[new_row][new_col];

                if cell == CellType::EMPTY {
                    continue;
                }
                else if type_diag_up == CellType::EMPTY || cell == type_diag_up {
                    type_diag_up = cell;
                    count_diag_up += 1;
                }
                else {
                    count_diag_up = 0;
                    break;
                }
            }
        }
        let  type_diag_up: i32     =  State::cell_type_to_heuristic(type_diag_up);
        return HEURISTIC_SEG_4[count_diag_up] * type_diag_up;
    }

    fn heuristic_seg_4_diag_down(&self, row: usize, col: usize) -> i32 {

        let  mut  type_diag_down   =  CellType::EMPTY;
        let  mut  count_diag_down   :usize  =  0;

        // Check for down-diagonal 4-segment (right direction) and count MAX's or MIN's.
        if row < BOARD_HIGHT-3 && col < BOARD_WIDTH-3 {
            for cell_incr in 0..4 {

                let new_row = row + cell_incr;
                let new_col = col + cell_incr;
                let cell: CellType = self.board[new_row][new_col];

                if cell == CellType::EMPTY {
                    continue;
                }
                else if type_diag_down == CellType::EMPTY || cell == type_diag_down {
                    type_diag_down = cell;
                    count_diag_down += 1;
                }
                else {
                    count_diag_down = 0;
                    break;
                }
            }
        }
        let  type_diag_down: i32   =  State::cell_type_to_heuristic(type_diag_down);
        return HEURISTIC_SEG_4[count_diag_down] * type_diag_down;
    }

    fn cell_type_to_heuristic(cell_type: CellType) -> i32 {
        match cell_type {
            CellType::EMPTY                =>   0,
            CellType::Player(Player::MAX)  =>   1,
            CellType::Player(Player::MIN)  =>  -1,
        }
    }

        
    // Returns true if the game is over.
    pub fn is_terminal(&mut self)  ->  bool  {
        if self.heuristic == None {
            self.set_heuristic();
        }
        return self.heuristic.unwrap().abs() == HEURISTIC_WIN;
    }


    // Add token from whoes player turn it is to the column specified by parameter
    // and switch turn.
    pub fn result(&mut self, column: usize) -> bool {

        let mut return_val: bool = false;

        // Try to drop token in given column.
        if column < BOARD_WIDTH {
            for row in (0..BOARD_HIGHT).rev() {
                if self.board[row][column] == CellType::EMPTY {
                    self.board[row][column] = CellType::Player(self.turn);
                    self.turn = match self.turn {
                        Player::MAX => Player::MIN,
                        Player::MIN => Player::MAX,
                    };
                    return_val = true;
                    self.set_heuristic();
                    break;
                }
                else {
                    continue;
                }
            }
        }
        return return_val;
    }

    // TODO WORK
    // ===================
    //
    
    // Returns the winner of the state.
    // pub  fn  utility(&self)      ->  i32   {return 0;}


    // No se com retornar una llista de state amb tamany indeterminat 
    // pub  fn  successors(&self)   ->  
}

impl fmt::Display for State {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // Choose which simbols to use for each player
        let cell_type_map: HashMap<CellType, char>  = HashMap::from([
            (CellType::Player(Player::MIN)    ,  'X'),
            (CellType::Player(Player::MAX)    ,  'O'),
            (CellType::EMPTY                  ,  ' '),
        ]);

        // Empty output
        let mut output = String::new();

        // Build the board... preaty basic ;)
        for row in 0..BOARD_HIGHT {
            output.push_str(&"|");
            for column in 0..BOARD_WIDTH {
                output.push(cell_type_map[&self.board[row][column]]);
            }
            output.push_str(&"|\n");
        }
        output.push_str(&" -------\n");

        // Print column numbers
        output.push_str(&" ");
        for row in 0..BOARD_WIDTH {
            output.push_str(&(row).to_string());
        }
        output.push('\n');

        // Print heuristic
        if self.heuristic != None {
            let heuristic_text = format!("Heuristic of position: {}.\n",
                                         &self.heuristic.unwrap().to_string());
            output.push_str(&heuristic_text);
        }

        // Inform of the winner
        if self.heuristic.unwrap().abs() == HEURISTIC_WIN {
            let winner = match self.turn {
                Player::MAX => CellType::Player(Player::MIN),
                Player::MIN => CellType::Player(Player::MAX),
            };
            let winner_text = format!("Player with {}'s wins!!!", cell_type_map[&winner]);
            output.push_str(&winner_text);
        }
        // Print whose turn it is.
        else {
            let turn_text = format!("It is now {}'s turn.\n", 
                                    cell_type_map[&CellType::Player(self.turn)]);
            output.push_str(&turn_text);
        }

        // Return the output built
        write!(f,"{}",output)
    }
}
