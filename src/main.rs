use clap::{App, Arg};

mod cube;
mod parser;
mod pruning;
mod solver;
mod tests;

use pruning::*;
use solver::Solver;
use std::thread;

fn main() {
    let matches = App::new("Rsubik")
        .version("0.0.1")
        .author("Brandon Lin <blin1283@gmail.com>")
        .arg(
            Arg::with_name("pruning")
                .short('p')
                .long("pruning")
                .takes_value(false)
                .about("Flag to say whether to generate pruning tables."),
        )
        .get_matches();

    if matches.is_present("pruning") {
        let corners_prune = thread::spawn(|| {
            generate_pruning_table_corners(String::from("corners.pt"));
            println!("Corners pruning table finished!");
        });
        let eo_prune = thread::spawn(|| {
            generate_pruning_table_eo(String::from("edges_o.pt"));
            println!("EO pruning table finished!");
        });
        let ep_prune = thread::spawn(|| {
            generate_pruning_table_ep(String::from("edges_p.pt"));
            println!("EP pruning table finished!");
        });
        corners_prune.join().unwrap();
        eo_prune.join().unwrap();
        ep_prune.join().unwrap();
        println!("Done generating pruning tables!");
    } else {
        let scramble = std::env::args().nth(1).unwrap_or(String::from("U2 F"));
        let parsed_seq = parser::parse_scramble(&scramble).unwrap();
        let solved = cube::CubeState::default();
        let new_state = solved.apply_move_instances(&parsed_seq);
        let solver: solver::AStarSolver = solver::Solver::new(new_state);
        println!("{:?}", solver.solve());
    }
}
