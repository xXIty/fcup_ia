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

    while !state.is_terminal() {
        println!("{}",state);

        let  mut  move_valid  =  false;

        while !move_valid {

            // Read column to drop token
            let  mut  move_request =  String::new();
            println!("Enter column number where you want to drop your token:");
            io::stdin().read_line(&mut move_request)
                .expect("Failed to read line.");

            // Try to convert column number to u8
            let move_request: usize = match move_request.trim().parse() {
                Ok(num) => num,
                Err(_)  => continue,
            };

            println!("Starting to move");

            move_valid = state.make_move(move_request);
        }
    }
}
