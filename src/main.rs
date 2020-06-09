extern crate strum;
extern crate strum_macros;

mod cube;
mod parser;

fn main() {
    let scramble = std::env::args().nth(1).unwrap_or(String::from("U2 F D'"));
    let parsed_seq = parser::parse_scramble(&scramble).unwrap();
    let solved = cube::CubeState::default();
    let new_state = solved.apply_move_instances(&parsed_seq);
    println!("{:?}", new_state);
}
