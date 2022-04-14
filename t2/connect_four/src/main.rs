//
// External dependencies imports
// =============================
//
use clap::{Arg, Command}; // Command line parser
                          
//
// Custom modules import
// ======================
//
mod connect_four;

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
    let state = connect_four::State::new();
    
}
