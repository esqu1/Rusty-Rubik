extern crate strum;
extern crate strum_macros;

mod cube;
mod parser;
mod solver;

use solver::Solver;

fn main() {
    let scramble = std::env::args().nth(1).unwrap_or(String::from("U2 F"));
    let parsed_seq = parser::parse_scramble(&scramble).unwrap();
    let solved = cube::CubeState::default();
    let new_state = solved.apply_move_instances(&parsed_seq);
    let solver : solver::AStarSolver = solver::Solver::new(new_state);
    println!("{:?}", solver.solve());
}
