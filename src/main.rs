extern crate strum;
extern crate strum_macros;

use clap::{App, Arg};

mod cube;
mod parser;
mod pruning;
mod solver;

use solver::Solver;
use pruning::generate_pruning_table_corners;

fn main() {
    let matches = App::new("Rsubik")
        .version("0.0.1")
        .author("Brandon Lin <blin1283@gmail.com>")
        .arg(
            Arg::with_name("pruning")
                .short('p')
                .long("pruning")
                .takes_value(false)
                .help("Flag to say whether to generate pruning tables."),
        )
        .get_matches();

    let pruning = matches.is_present("pruning");

    if pruning {
        generate_pruning_table_corners(String::from("test.pt"));
    } else {
        let scramble = std::env::args().nth(1).unwrap_or(String::from("U2 F"));
        let parsed_seq = parser::parse_scramble(&scramble).unwrap();
        let solved = cube::CubeState::default();
        let new_state = solved.apply_move_instances(&parsed_seq);
        let solver: solver::AStarSolver = solver::Solver::new(new_state);
        println!("{:?}", solver.solve());
    }
}
