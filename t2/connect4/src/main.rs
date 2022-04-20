//
// External dependencies imports
// =============================
//
use clap::{Arg, Command}; // Command line parser
                          
//
// Custom modules import
// ======================
//
mod connect4;
mod connect4solver;

fn main() {

    // Easy command line argument parser using Clap
    // ==============================================
    //
    let command = Command::new("Connect Four")
        .author("Jordi Garcia & Miquel roset")
        .about("Program to study different adversarial algorithms applied to game of Connect four.")
        .arg(
            Arg::new("depth")
                .long           (  "d1"  )
                .id             (  "depth1"  )
                .required       (  false    )
                .takes_value    (  true     )
                .default_value  (  "6"      )
                .help           ( "Sets the search depth for the 1st used algorithm." )
        )
        .arg(
            Arg::new("depth")
                .long           (  "d2"  )
                .id             (  "depth2"  )
                .required       (  false    )
                .takes_value    (  true     )
                .default_value  (  "6"      )
                .help           ( "Sets the search depth for the 2nd used algorithm." )
        )
        .arg(
            Arg::new("player1")
                .long             (  "p1"           )
                .value_name       (  "type"         )
                .required         (  false          )
                .takes_value      (  true           )
                .default_value    (  "MINMAX"       )
                .possible_values  (  ["INTERACTIVE",
                                     "MINMAX",
                                     "MINMAX",     
                                     "ALPHA-BETA",  
                                     "MCTS"]        )
                .help             ( "Algorithm to play against" ) 
        )
        .arg(
            Arg::new("player2")
                .long             (  "p2"           )
                .value_name       (  "type"         )
                .required         (  false          )
                .takes_value      (  true           )
                .default_value    (  "MINMAX"       )
                .possible_values  (  ["INTERACTIVE",
                                     "MINMAX",     
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
//
//    let p1 = connect4solver::Algorithm::User;
//    let p2 = connect4solver::Algorithm::User;
//    
    let depth1: u32 = command.value_of("depth1").unwrap().parse().unwrap();
    let depth2: u32 = command.value_of("depth2").unwrap().parse().unwrap();
    let p1 = connect4solver::get_solver(command.value_of("player1").unwrap(), depth1);
    let p2 = connect4solver::get_solver(command.value_of("player2").unwrap(), depth2);

    while !state.is_terminal() {
        println!("{}",state);
        match state.get_player() {
            connect4::Player::MAX=> {
                p1.play(&mut state);
            }
            connect4::Player::MIN=> {
                p2.play(&mut state);
            }
        }
    }
    println!("{}",state);
}

