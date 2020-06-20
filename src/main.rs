use clap::{App, Arg};

mod cube;
mod parser;
mod pruning;
mod solver;
mod tests;

use cube::CubeState;
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
        .arg(
            Arg::with_name("scramble")
                .short('s')
                .long("scramble")
                .takes_value(true)
                .about("Scramble sequence to solve."),
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
        if let Some(scramble) = matches.value_of("scramble") {
            let parsed_seq = parser::parse_scramble(&scramble).unwrap();
            println!("{:?}", parsed_seq);
            let solved = cube::CubeState::default();
            let new_state = solved.apply_move_instances(&parsed_seq);
            println!("{:?}", new_state);
            let new_state2 = new_state.clone();
            // load the pruning tables
            let corner_prune =
                std::fs::read("corners.pt").expect("Error reading corners pruning table");
            println!("Loaded corners pruning table.");
            let eo_prune = std::fs::read("edges_o.pt").expect("Error reading EO pruning table");
            println!("Loaded EO pruning table.");
            let ep_prune = std::fs::read("edges_p.pt").expect("Error reading EP pruning table");
            println!("Loaded EP pruning table.");
            let pruning_tables = PruningTables {
                corners: corner_prune,
                eo: eo_prune,
                ep: ep_prune,
            };

            let solver = solver::IDASolver::new(new_state, &pruning_tables);
            let solution = solver.solve();
            println!("{:?}", solution);
            println!("Verifying the above solution...");
            let maybe_solved = new_state2.apply_move_instances(&solution);
            if maybe_solved == CubeState::default() {
                println!("Successfully verified!");
            } else {
                println!("Uh oh...it's wrong...rip you.");
            }
        } else {
            println!("Missing scramble sequence; use `--help` for more info.");
        }
    }
}
