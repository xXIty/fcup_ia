//
// External dependencies imports
// =============================
//
use clap::{Arg, Command}; // Command line parser
use std::time::Instant;
                          
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
    let args = Command::new("Connect Four")
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
                                     "ALPHA-BETA-O",
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
                                     "ALPHA-BETA-O",
                                     "MCTS"]        )
                .help             ( "Algorithm to play against" ) 
        )
        .arg(
            Arg::new("quiet")
                .short            (  'q'            )
                .long             (  "quiet"        )
                .value_name       (  "quiet"        )
                .required         (  false          )
                .takes_value      (  false          )
                .help             ( "Output statistic information only." ) 
        )
        .after_help("With which depth can you beat me? ;))")
        .get_matches();


    // Variable initializations
    // ==========================
    //

    let mut game_round: u128 = 0;
    let mut state = connect4::State::new();

    let output_quiet: bool = args.is_present("quiet");

    let solve_method1 = args.value_of("player1").unwrap();
    let solve_method2 = args.value_of("player2").unwrap();

    let depth1: u32 = args.value_of("depth1").unwrap().parse().unwrap();
    let depth2: u32 = args.value_of("depth2").unwrap().parse().unwrap();

    let p1 = connect4solver::get_solver(solve_method1, depth1);
    let p2 = connect4solver::get_solver(solve_method2, depth2);

    let mut p1_turn_time_sum: u128 = 0;
    let mut p2_turn_time_sum: u128 = 0;

    // Gameplay
    // =========
    //
    while !state.is_terminal() {

        // Verbose output (not quiet)
        if !output_quiet {
            println!("{}",state);
        }

        // Play a turn and do timing
        match state.get_player() {
            connect4::Player::MAX=> {
                let p1_turn_start = Instant::now();
                p1.play(&mut state);
                let p1_turn_elapsed = p1_turn_start.elapsed().as_micros();
                p1_turn_time_sum += p1_turn_elapsed;
            }
            connect4::Player::MIN=> {
                let p2_turn_start = Instant::now();
                p2.play(&mut state);
                let p2_turn_elapsed = p2_turn_start.elapsed().as_micros();
                p2_turn_time_sum += p2_turn_elapsed;
            }
        }
        game_round += 1;
    }

    // Calculate mean turn time elapsed per player
    let p1_turn_time_mean: u128 = p1_turn_time_sum / game_round;
    let p2_turn_time_mean: u128 = p2_turn_time_sum / game_round;
    
    println!("Game statistics!");
    println!("Mean time spent per turn.");
    println!("# {} {} {}", solve_method1, depth1, p1_turn_time_mean);
    println!("# {} {} {}", solve_method2, depth2, p2_turn_time_mean);

    if !output_quiet {
        println!("{}",state);
    }
}

