/**
pub struct CubeState {
}
is an example for a 3x3

have a macro to generate these states?
*/
use std::collections::HashSet;

pub trait PuzzleMove {

}

pub trait Puzzle<M: PuzzleMove> {

    // Initializes this puzzle in its solved state.
    fn new() -> Self;

    fn get_all_moves() -> HashSet<M>;

    // TODO: Megaminx has different scrambling and actual solving moves

    // Generally, for puzzles like NxN puzzles, all available moves are allowed.
    // However, for bandaging puzzles such as the Bandaged Cube and Square-1, certain
    // moves will restrict others from being allowed.
    fn get_allowed_moves(&self) -> HashSet<M>;

    fn make_move(&self, m: PuzzleMove)
    fn make_move_mut(&mut self)
}

pub struct Cube3 {
    pub cp: [u8; 8],
    pub co: [i8; 8],
    pub ep: [u8; 12],
    pub eo: [i8; 12],
}

pub struct Cube4 {
    pub cp: [u8; 8],
    pub co: [i8; 8],
    pub wings1: [u8, 24],
    pub xcenters1: [u8, 24],
}

pub struct Cube5 {
    // corners are always the same...
    pub wings1: [u8, 24],
    pub midges_ep: [u8, 12],
    pub midges_eo: [i8, 12],
    pub xcenters1: [u8, 24],
    pub pluscenters1: [u8, 24],
}

pub struct Cube6 {
    //...
    pub wings1: [u8, 24],
    pub wings2: [u8, 24],
    // ...
}

pub struct Cube335 {
    pub cp: [u8, 8],
    // actually in cuboids, where 90 degree rotation isn't allowed, orientation doesn't
    // matter
    pub
}


// macro_rules! define_cube {
//     ($dim:literal) => {
//         pub struct Cube$dim {
//             pub cp: [u8; 8],
//             pub co: [i8; 8],

//         }
//     }
// }

pub enum CubeAxis {
    U,
    D,
    L,
    R,
    F,
    B,
}

// rotations?

pub struct CubeMove {
    pub axis: CubeAxis,
    pub depth: i8,
    pub rotation: i8,
}

// move actions?

impl Puzzle<CubeMove> for Cube3 {

    pub fn new() -> Self {
        Cube3 {}
    }

    pub fn get_all_moves() -> HashSet<Cube3Move> {
        let mut all_moves = HashSet::new();
        all_moves.insert()
    }
}


// I want:
// let cube = RubiksCube::new();
// cube.make_move_mut("R");
//
// in python:
// cube = Cube3("U D")
