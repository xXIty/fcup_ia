//
// External dependencies imports
// =============================
//
use std::io;
use clap::{Arg, Command}; // Command line parser
                          
//
// Custom modules import
// ======================
//
mod connect4;
mod connect4solver;

#[derive(Debug)]
enum Algorithm {
    User,
    Minimax(i32),
    AlphaBeta,
    MCTS,
}

fn main() {

    // Easy command line argument parser using Clap
    // ==============================================
    //
    Command::new("Connect Four")
        .author("Jordi Garcia & Miquel roset")
        .about("Program to study different adversarial algorithms applied to game of Connect four.")
        .arg(
            Arg::new("depth")
                .short          (  'd'      )
                .long           (  "depth"  )
                .id             (  "depth"  )
                .required       (  false    )
                .takes_value    (  true     )
                .default_value  (  "6"      )
                .help           ( "Sets the search depth for the used algorithm." )
        )
        .arg(
            Arg::new("human_starts")
                .short        (  's'             )
                .long         (  "human_starts"  )
                .required     (  false           )
                .takes_value  (  false           )
                .help         ( "Lets you start the game" )
        )
        .arg(
            Arg::new("algorithm")
                .short            (  'a'            )
                .long             (  "algorithm"    )
                .id               (  "algorithm"    )
                .required         (  false          )
                .takes_value      (  true           )
                .default_value    (  "MINMAX"       )
                .possible_values  (  ["MINMAX",     
                                     "ALPHA-BETA",  
                                     "MCTS"]        )
                .help             ( "Algorithm to play against" ) 
        )
        .after_help("With which depth can you beat me? ;))")
        .get_matches();


    // Variable initializations
    // ==========================
    //
    let mut state = connect4::State::new();
    let depth: i32 = 2;
    let p1 = Algorithm::User;
    let p2 = Algorithm::Minimax(depth);

    while !state.is_terminal() {
        println!("{}",state);
        p1.decide_and_run(&mut state);
        p2.decide_and_run(&mut state);

    }
    println!("{}",state);
}

impl Algorithm {

    fn decide_and_run(&self, s: &mut connect4::State) {
        println!("algo:  {:?}", *self);
        match self {
            Algorithm::User => {
                let  mut  move_valid  =  false;
                
                while !move_valid {

                    // Read column to drop token
                    let  mut  move_request =  String::new();
                    println!("Enter column number where you want to drop your token:");
                    io::stdin().read_line(&mut move_request)
                        .expect("Failed to read line.");

                    // Try to convert column number to usize
                    let move_request: usize = match move_request.trim().parse() {
                        Ok(num) => num,
                        Err(_)  => continue,
                    };

                    // Try to make a move over the state
                    move_valid = s.result(move_request);
                }
            }
            Algorithm::Minimax(depth) => {
                println!("DEPTH: {}",depth);
                connect4solver::minimax_solver(s, *depth);
                s.result(0);
            }
            Algorithm::AlphaBeta => { 
                println!("not implemented");
                s.result(0);
            }

            Algorithm::MCTS => { 
                println!("not implemented");
                s.result(0);
            }
        }


    }

}

