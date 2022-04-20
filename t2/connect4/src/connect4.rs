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
const  UTILITY_TURN:   i32     =  16;            
const  UTILITY_SEG_4: [i32; 5] =  [0, 1, 10, 50, 512];
const  UTILITY_WIN:    i32     =  UTILITY_SEG_4[4];


#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Player {
    MAX =  1,
    MIN = -1,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum CellType {
    Token(Player),
    EMPTY,
}


#[derive(Copy, Clone, Debug)]
pub struct State {
    turn:       Player,                      
    board:      [[CellType; BOARD_WIDTH]; BOARD_HIGHT],
    utility:  Option<i32>,                 
    pub depth: i32,
}

impl State {

    // Returns a game without any move done.
    pub fn new() -> State {
        State {
            turn:       Player::MAX,
            board:      [[CellType::EMPTY; BOARD_WIDTH]; BOARD_HIGHT],
            utility:  Some(UTILITY_TURN * (Player::MAX as i32)),
            depth: 0,
        }
    }

    pub fn get_player(&self) -> Player {
        self.turn
    }

    // Calculates the utility of the state.
    pub fn get_utility(&self) -> i32 {
        return self.utility.unwrap();
    }

    pub fn set_utility(&mut self) {


        // Initialize evaluation with bonus from whos turn is.
        let mut utility_total: i32 = UTILITY_TURN * (self.turn as i32);

        // Check each segment of 4 cells
        'calculate_utility: for row in (0..BOARD_HIGHT).rev() {

            for col in 0..BOARD_WIDTH {

                // Calculate horizontal 4-segment
                let utility_cell = self.utility_seg_4_horizontal(row, col);
                if utility_cell.abs() == UTILITY_WIN {
                    utility_total = utility_cell;
                    break 'calculate_utility;
                }
                else {
                    utility_total += utility_cell;
                }

                // Calculate vertical 4-segment
                let utility_cell = self.utility_seg_4_vertical(row, col);
                if utility_cell.abs() == UTILITY_WIN {
                    utility_total = utility_cell;
                    break 'calculate_utility;
                }
                else {
                    utility_total += utility_cell;
                }

                // Calculate diagonal-up 4-segment
                let utility_cell = self.utility_seg_4_diag_up(row, col);
                if utility_cell.abs() == UTILITY_WIN {
                    utility_total = utility_cell;
                    break 'calculate_utility;
                }
                else {
                    utility_total += utility_cell;
                }

                // Calculate diagonal-down 4-segment
                let utility_cell = self.utility_seg_4_diag_down(row, col);
                if utility_cell.abs() == UTILITY_WIN {
                    utility_total = utility_cell;
                    break 'calculate_utility;
                }
                else {
                    utility_total += utility_cell;
                }
            }
        }
        self.utility = Some(utility_total);
    }

    fn utility_seg_4_horizontal(&self, row: usize, col: usize) -> i32 {

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

        let  type_horizontal: i32    =  State::cell_type_to_utility(type_horizontal);
        return type_horizontal * UTILITY_SEG_4[count_horizontal];
    }

    fn utility_seg_4_vertical(&self, row: usize, col: usize) -> i32 {

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
        let  type_vertical: i32  =  State::cell_type_to_utility(type_vertical);
        return UTILITY_SEG_4[count_vertical] * type_vertical;
    }

    fn utility_seg_4_diag_up(&self, row: usize, col: usize) -> i32 {

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
        let  type_diag_up: i32     =  State::cell_type_to_utility(type_diag_up);
        return UTILITY_SEG_4[count_diag_up] * type_diag_up;
    }

    fn utility_seg_4_diag_down(&self, row: usize, col: usize) -> i32 {

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
        let  type_diag_down: i32   =  State::cell_type_to_utility(type_diag_down);
        return UTILITY_SEG_4[count_diag_down] * type_diag_down;
    }

    fn cell_type_to_utility(cell_type: CellType) -> i32 {
        match cell_type {
            CellType::EMPTY                =>   0,
            CellType::Token(Player::MAX)  =>   1,
            CellType::Token(Player::MIN)  =>  -1,
        }
    }

        
    // Returns true if the game is over.
    pub fn is_terminal(&self)  ->  bool  {
        let mut terminal: bool = true;
        for col in 0..BOARD_WIDTH {
            if self.board[0][col] == CellType::EMPTY {
                terminal = false;
                break;
            }
        }
        terminal |= self.utility.unwrap().abs() == UTILITY_WIN;
        return terminal;
    }

    

    // Add token from whoes player turn it is to the column specified by parameter
    // and switch turn.
    pub fn result(&mut self, column: usize) -> bool {

        let mut return_val: bool = false;

        // Try to drop token in given column.
        if column < BOARD_WIDTH {
            for row in (0..BOARD_HIGHT).rev() {
                if self.board[row][column] == CellType::EMPTY {
                    self.board[row][column] = CellType::Token(self.turn);
                    self.turn = match self.turn {
                        Player::MAX => Player::MIN,
                        Player::MIN => Player::MAX,
                    };
                    self.depth = self.depth + 1;
                    return_val = true;
                    self.set_utility();
                    break;
                }
                else {
                    continue;
                }
            }
        }
        return return_val;
    }

    pub fn actions(&self) -> Vec<usize> {

        let mut acts : Vec<usize> = Vec::new(); 
        for col in 0..BOARD_WIDTH {
            match self.board[0][col] {
                CellType::EMPTY => acts.push(col),
                _ => continue,
            };
            
        }
        return acts;         

    }
}

impl fmt::Display for State {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // Choose which simbols to use for each player
        let cell_type_map: HashMap<CellType, char>  = HashMap::from([
            (CellType::Token(Player::MIN)    ,  'X'),
            (CellType::Token(Player::MAX)    ,  'O'),
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

        // Print utility
        if self.utility != None {
            let utility_text = format!("Heuristic of position: {}.\n",
                                         &self.utility.unwrap().to_string());
            output.push_str(&utility_text);
        }

        // Inform of the winner
        if self.utility.unwrap().abs() == UTILITY_WIN {
            let winner = match self.turn {
                Player::MAX => CellType::Token(Player::MIN),
                Player::MIN => CellType::Token(Player::MAX),
            };
            let winner_text = format!("Player with {}'s wins!!!", cell_type_map[&winner]);
            output.push_str(&winner_text);
        }
        // Print whose turn it is.
        else {
            let turn_text = format!("It is now {}'s turn.\n", 
                                    cell_type_map[&CellType::Token(self.turn)]);
            output.push_str(&turn_text);
        }

        // Return the output built
        write!(f,"{}",output)
    }
}
