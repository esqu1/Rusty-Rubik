//! A module providing functions to interact with the
//! structure and state of the Rubik's Cube.
//!
//! The state of the Rubik's Cube is internally represented
//! by four properties of the cube: corner permutation, corner
//! orientation, edge permutation, and edge orientation. A tuple
//! of these four properties (with correct parity relations)
//! uniquely determines the state of the cube.

use strum_macros::EnumString;

/// An enum for the faces of the Rubik's Cube.
///
/// - U: top face
/// - D: bottom face
/// - L: left face
/// - R: right face
/// - F: front face
/// - B: back face
#[derive(PartialEq, Eq, EnumString, Debug, Clone, Copy)]
pub enum BaseMoveToken {
    U,
    D,
    L,
    R,
    F,
    B,
}

impl std::fmt::Display for BaseMoveToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents the direction which to turn a face. `Prime` represents
/// a counter-clockwise rotation of a face, and `Double` represents
/// a 180 degree rotation of a face.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Normal,
    Prime,
    Double,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Normal => write!(f, ""),
            Direction::Prime => write!(f, "'"),
            Direction::Double => write!(f, "2"),
        }
    }
}

/// An instantiation of a certain face equipped with a direction.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MoveInstance {
    pub basemove: BaseMoveToken,
    pub dir: Direction,
}

impl MoveInstance {
    pub fn new(basemove: BaseMoveToken, dir: Direction) -> Self {
        Self { basemove, dir }
    }

    pub fn invert(&self) -> Self {
        Self::new(
            self.basemove,
            match self.dir {
                Direction::Normal => Direction::Prime,
                Direction::Prime => Direction::Normal,
                x => x,
            },
        )
    }
}

impl std::fmt::Display for MoveInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.basemove, self.dir)
    }
}

/// A struct representing sequences of moves, used for representing
/// scramble sequences and solution sequences.
pub struct MoveSequence(pub Vec<MoveInstance>);

impl MoveSequence {
    pub fn get_moves(&self) -> &Vec<MoveInstance> {
        &self.0
    }
    pub fn get_moves_mut(&mut self) -> &mut Vec<MoveInstance> {
        &mut self.0
    }

    pub fn invert(&self) -> Self {
        let mut moves = vec![];
        for m in self.get_moves().iter().rev() {
            moves.push(m.invert());
        }
        MoveSequence(moves)
    }
}

impl std::fmt::Display for MoveSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strs = vec![];
        for m in self.get_moves().iter() {
            strs.push(m.to_string());
        }
        write!(f, "{}", strs.join(" "))
    }
}

/// A struct representing a commutator, taking the form of a tuple.
///
/// If the first element of this tuple is $A$, and the second is $B$, then
/// the commutator represented by this is $[A,B] = ABA^{-1}B^{-1}$.
///
/// One can create a Commutator object as such:
/// ```
/// use rusty_rubik::cube::*;
/// use rusty_rubik::cube_move;
///
/// fn main() {
///     let a = MoveSequence(vec![
///         cube_move!(R, Normal),
///         cube_move!(U, Prime),
///         cube_move!(R, Prime),
///     ]);
///     let b = MoveSequence(vec![cube_move!(D, Normal)]);
///
///     // commutator representing [R U' R', D] = R U' R' D R U R' D'
///     let comm = Commutator(a,b);
///     
/// }
/// ```
pub struct Commutator(pub MoveSequence, pub MoveSequence);

/// A struct representing a conjugate, taking the form of a tuple.
///
/// If the first element of this tuple is $C$, and the second is a commutator $B$,
/// then the conjugate represented by this is $[C: B] = CBC^{-1}$.
///
/// One can create a Conjugate object as such:
///
/// ```
/// use rusty_rubik::cube::*;
/// use rusty_rubik::cube_move;
///
/// fn main() {
///     let c = MoveSequence(vec![
///         cube_move!(R, Normal),
///     ]);
///     let a = MoveSequence(vec![
///         cube_move!(R, Normal),
///         cube_move!(D, Normal),
///         cube_move!(R, Prime),
///     ]);
///     let b = MoveSequence(vec![
///         cube_move!(U, Double),
///     ]);
///
///     // conjugate representing [R: [R D R', U2]] = R2 D R' U2 R D' R' U2 R'
///     let comm = Commutator(a,b);
///     
/// }
/// ```
pub struct Conjugate(pub MoveSequence, pub Commutator);

/// An internal set of permutation vectors representing what action
/// is done to a configuration of the Rubik's Cube when a move is applied.
///
/// The order of the corners and edges is as follows:
/// - Corners: UBL UBR UFR UFL DFL DFR DBR DBL
/// - Edges: UB UR UF UL BL BR FR FL DF DR DB DL
struct Move {
    pub cp_change: [u8; 8], // a[i] gives the position that i goes to
    pub co_change: [i8; 8],
    pub ep_change: [u8; 12],
    pub eo_change: [i8; 12],
}

/// A shorthand macro that can be used to construct MoveInstances.
///
/// ```
/// use rusty_rubik::cube::*;
/// use rusty_rubik::cube_move;
///
/// fn main() {
///     let r_prime: MoveInstance = cube_move!(R, Prime);
///     let u2: MoveInstance = cube_move!(U, Double);
/// }
/// ```
#[macro_export]
macro_rules! cube_move {
    ($basemove: ident, $dir:ident) => {{
        MoveInstance {
            basemove: BaseMoveToken::$basemove,
            dir: Direction::$dir,
        }
    }};
}

macro_rules! apply_permutation {
    ($og_state: expr, $delta: expr) => {{
        if $og_state.len() != $delta.len() {
            panic!("Size mismatch in applying permutation");
        } else {
            let mut new_array = $og_state.clone();
            for i in 0..$og_state.len() {
                new_array[$delta[i] as usize] = $og_state[i];
            }
            new_array
        }
    }};
}

macro_rules! apply_orientation {
    ($og_state: expr, $delta: expr, $num_orientations: expr) => {{
        if $og_state.len() != $delta.len() {
            panic!("Size mismatch in applying orientation");
        } else {
            let mut new_array = $og_state.clone();
            for i in 0..$og_state.len() {
                new_array[i] = (($og_state[i] + $delta[i] + $num_orientations) % $num_orientations);
                if new_array[i] == 2 {
                    new_array[i] = -1;
                }
            }
            new_array
        }
    }};
}

pub(crate) fn get_basemove_pos(token: BaseMoveToken) -> u8 {
    match token {
        BaseMoveToken::U => 5,
        BaseMoveToken::D => 4,
        BaseMoveToken::L => 3,
        BaseMoveToken::R => 2,
        BaseMoveToken::F => 1,
        BaseMoveToken::B => 0,
    }
}

fn get_antipode(token: BaseMoveToken) -> BaseMoveToken {
    match token {
        BaseMoveToken::U => BaseMoveToken::D,
        BaseMoveToken::D => BaseMoveToken::U,
        BaseMoveToken::L => BaseMoveToken::R,
        BaseMoveToken::R => BaseMoveToken::L,
        BaseMoveToken::F => BaseMoveToken::B,
        BaseMoveToken::B => BaseMoveToken::F,
    }
}

// bitvector: [UDLRFB], 0 means it's allowed
pub(crate) fn get_allowed_post_moves(prev_bv: u8, last_move: Option<BaseMoveToken>) -> u8 {
    if let Some(lm) = last_move {
        let antipode = get_antipode(lm);
        if prev_bv & (1 << get_basemove_pos(antipode)) != 0 {
            // then the antipode was already applied
            (1 << get_basemove_pos(lm)) + (1 << get_basemove_pos(antipode))
        } else {
            1 << get_basemove_pos(lm)
        }
    } else {
        0
    }
}

/// Determines which moves are allowed after the given move sequence,
/// to speed up solver methods.
///
/// This is to avoid double rotations of faces (e.g. R R') and
/// excessive rotations of antipodal faces (e.g. R L R can be simplified
/// to R2 L).
// TODO: refactor into struct method
pub fn allowed_moves_after_seq(moves: &MoveSequence) -> u8 {
    let sol = moves.get_moves();
    match sol.len() {
        0 => 0,
        1 => {
            let last_move = sol[sol.len() - 1];
            1 << get_basemove_pos(last_move.basemove)
        }
        _ => {
            let last_move = sol[sol.len() - 1];
            let second_to_last = sol[sol.len() - 2];
            if get_antipode(last_move.basemove) == second_to_last.basemove {
                (1 << get_basemove_pos(last_move.basemove))
                    + (1 << get_basemove_pos(second_to_last.basemove))
            } else {
                1 << get_basemove_pos(last_move.basemove)
            }
        }
    }
}

/// The underlying struct for representing a configuration of the Rubik's Cube.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CubeState {
    pub cp: [u8; 8],
    pub co: [i8; 8],
    pub ep: [u8; 12],
    pub eo: [i8; 12],
}

impl Default for CubeState {
    fn default() -> CubeState {
        CubeState {
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            co: [0 as i8; 8],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eo: [0 as i8; 12],
        }
    }
}

fn get_move_matrix(mov: &BaseMoveToken) -> Move {
    match mov {
        BaseMoveToken::U => MOVE_U,
        BaseMoveToken::D => MOVE_D,
        BaseMoveToken::L => MOVE_L,
        BaseMoveToken::R => MOVE_R,
        BaseMoveToken::F => MOVE_F,
        BaseMoveToken::B => MOVE_B,
    }
}

fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

// range:
// corners: [0, 8! - 1]
// edges: [0, 12! - 1]
fn get_index_of_permutation(perm: &[u8]) -> u32 {
    // 2 bytes suffice for 12!
    let mut fin = 0;
    for i in 0..perm.len() {
        let mut res = 0;
        for j in (i + 1)..perm.len() {
            if perm[j] < perm[i] {
                res += 1;
            }
        }
        fin += res * factorial((perm.len() - i - 1) as u32);
    }
    fin as u32
}

// range:
// corners: [0, 3^7 - 1]
// edges: [0, 2^11 - 1]
fn get_index_of_orientation(ori: &[i8], num_orientations: u8) -> u16 {
    let mut result = 0;
    for (i, val) in ori.iter().enumerate() {
        if i == ori.len() - 1 {
            break;
        }
        let pos = (val + num_orientations as i8) % num_orientations as i8;
        result += pos as u16;
        if i != ori.len() - 2 {
            result *= num_orientations as u16;
        }
    }
    result
}

/// Returns a triple representing a compressed representation of a Rubik's
/// Cube configuration.
///
/// The elements of the triple consist of a corner index, an
/// edge orientation (EO) index, and an edge permutation (EP) index.
// TODO: refactor into struct method
pub fn get_index_of_state(state: &CubeState) -> (u32, u16, u64) {
    let cp_index = get_index_of_permutation(&state.cp);
    let co_index = get_index_of_orientation(&state.co, 3);
    let corner_index = cp_index * u32::pow(3, 7) + (co_index as u32);
    let ep_index = get_index_of_permutation(&state.ep) as u64;
    let eo_index = get_index_of_orientation(&state.eo, 2);
    (corner_index, eo_index, ep_index)
}

impl CubeState {
    fn apply_basemove(&self, m: &BaseMoveToken) -> Self {
        let mov = get_move_matrix(m);
        let oriented_corners = apply_orientation!(&self.co, &mov.co_change, 3);
        let oriented_edges = apply_orientation!(&self.eo, &mov.eo_change, 2);
        CubeState {
            cp: apply_permutation!(&self.cp, &mov.cp_change),
            co: apply_permutation!(oriented_corners, &mov.cp_change),
            ep: apply_permutation!(&self.ep, &mov.ep_change),
            eo: apply_permutation!(oriented_edges, &mov.ep_change),
        }
    }

    /// Applies a move to a Rubik's Cube configuration.
    pub fn apply_move_instance(&self, m: &MoveInstance) -> Self {
        let num_turns = match &m.dir {
            Direction::Normal => 1,
            Direction::Prime => 3,
            Direction::Double => 2,
        };
        (0..num_turns).fold(self.clone(), |acc, _| acc.apply_basemove(&m.basemove))
    }

    /// Applies a sequence of moves, in order to a Rubik's Cube configuration.
    pub fn apply_move_instances(&self, moves: &MoveSequence) -> Self {
        moves
            .get_moves()
            .iter()
            .fold(self.clone(), |acc, mov| acc.apply_move_instance(&mov))
    }

    // pub fn random() -> Self {

    // }
}

/// A vector of all allowed moves on a Rubik's Cube.
pub const ALL_MOVES: [MoveInstance; 18] = [
    cube_move!(U, Normal),
    cube_move!(U, Prime),
    cube_move!(U, Double),
    cube_move!(D, Normal),
    cube_move!(D, Prime),
    cube_move!(D, Double),
    cube_move!(L, Normal),
    cube_move!(L, Prime),
    cube_move!(L, Double),
    cube_move!(R, Normal),
    cube_move!(R, Prime),
    cube_move!(R, Double),
    cube_move!(F, Normal),
    cube_move!(F, Prime),
    cube_move!(F, Double),
    cube_move!(B, Normal),
    cube_move!(B, Prime),
    cube_move!(B, Double),
];

const MOVE_U: Move = Move {
    cp_change: [1, 2, 3, 0, 4, 5, 6, 7],
    co_change: [0, 0, 0, 0, 0, 0, 0, 0],
    ep_change: [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

const MOVE_D: Move = Move {
    cp_change: [0, 1, 2, 3, 5, 6, 7, 4],
    co_change: [0, 0, 0, 0, 0, 0, 0, 0],
    ep_change: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

const MOVE_R: Move = Move {
    cp_change: [0, 6, 1, 3, 4, 2, 5, 7],
    co_change: [0, -1, 1, 0, 0, -1, 1, 0],
    ep_change: [0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

const MOVE_L: Move = Move {
    cp_change: [3, 1, 2, 4, 7, 5, 6, 0],
    co_change: [1, 0, 0, -1, 1, 0, 0, -1],
    ep_change: [0, 1, 2, 7, 3, 5, 6, 11, 8, 9, 10, 4],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

const MOVE_F: Move = Move {
    cp_change: [0, 1, 5, 2, 3, 4, 6, 7],
    co_change: [0, 0, -1, 1, -1, 1, 0, 0],
    ep_change: [0, 1, 6, 3, 4, 5, 8, 2, 7, 9, 10, 11],
    eo_change: [0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0],
};

const MOVE_B: Move = Move {
    cp_change: [7, 0, 2, 3, 4, 5, 1, 6],
    co_change: [-1, 1, 0, 0, 0, 0, -1, 1],
    ep_change: [4, 1, 2, 3, 10, 0, 6, 7, 8, 9, 5, 11],
    eo_change: [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0],
};
