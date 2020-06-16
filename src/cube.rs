use strum_macros::EnumString;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CubeState {
    pub cp: [i8; 8],
    pub co: [i8; 8],
    pub ep: [i8; 12],
    pub eo: [i8; 12],
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Normal,
    Prime,
    Double,
}

#[derive(PartialEq, Eq, EnumString, Debug, Clone, Copy)]
pub enum BaseMoveToken {
    U,
    D,
    L,
    R,
    F,
    B,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MoveInstance {
    pub basemove: BaseMoveToken,
    pub dir: Direction,
}

pub type MoveSequence = Vec<MoveInstance>;

impl MoveInstance {
    pub fn new(basemove : BaseMoveToken, dir: Direction) -> MoveInstance {
        MoveInstance {
            basemove,
            dir
        }
    }
}

// corners: UBL UBR UFR UFL DFL DFR DBR DBL
// edges: UB UR UF UL BL BR FR FL DF DR DB DL

pub struct Move {
    pub cp_change: [i8; 8], // a[i] gives the position i goes to
    pub co_change: [i8; 8],
    pub ep_change: [i8; 12],
    pub eo_change: [i8; 12],
}

#[macro_export]
macro_rules! cube_move {
    ($basemove: ident, $dir:ident) => {
        {
            MoveInstance {
                basemove: BaseMoveToken::$basemove,
                dir: Direction::$dir,
            }
        }
    }
}

macro_rules! apply_permutation {
    ($og_state: expr, $delta: expr) => {{
        if $og_state.len() != $delta.len() {
            panic!("Size mismatch in applying permutation");
        } else {
            let mut new_array = $og_state.clone();
            for i in 0..$og_state.len() {
                new_array[i] = $og_state[$delta[i] as usize];
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



// bitvector: [UDLRFB] x [_'2] e.g. U, U', U2, etc
// pub fn get_allowed_post_moves(moves: MoveSequence) -> u32 {
//     // depends on the last two moves
//     if moves.len() == 0 {
//         0
//     } else if moves.len() <= 1 {
//         let Some(sole_move) = moves.iter().next();


//     } else {
//         let last_two = moves.iter().rev().take(2);
//     }
// }



impl Default for CubeState {
    fn default() -> CubeState {
        CubeState {
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            co: [0, 0, 0, 0, 0, 0, 0, 0],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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

impl CubeState {
    // TODO: change move to move instance
    pub fn apply_basemove(&self, m: &BaseMoveToken) -> Self {
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

    pub fn apply_move_instance(&self, m: &MoveInstance) -> Self {
        let num_turns = match &m.dir {
            Direction::Normal => 1,
            Direction::Prime => 3, 
            Direction::Double => 2,
        };
        (0..num_turns).fold(self.clone(), |acc, _| acc.apply_basemove(&m.basemove))
    }

    pub fn apply_move_instances(&self, moves: &Vec<MoveInstance>) -> Self {
        moves.iter().fold(self.clone(), |acc, mov| acc.apply_move_instance(&mov))
    }

    // pub fn random() -> Self {

    // }
}

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

pub const MOVE_U: Move = Move {
    cp_change: [1, 2, 3, 0, 4, 5, 6, 7],
    co_change: [0, 0, 0, 0, 0, 0, 0, 0],
    ep_change: [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const MOVE_D: Move = Move {
    cp_change: [0, 1, 2, 3, 5, 6, 7, 4],
    co_change: [0, 0, 0, 0, 0, 0, 0, 0],
    ep_change: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const MOVE_R: Move = Move {
    cp_change: [0, 6, 1, 3, 4, 2, 5, 7],
    co_change: [0, -1, 1, 0, 0, -1, 1, 0],
    ep_change: [0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const MOVE_L: Move = Move {
    cp_change: [3, 1, 2, 4, 7, 5, 6, 0],
    co_change: [1, 0, 0, -1, 1, 0, 0, -1],
    ep_change: [0, 1, 2, 7, 3, 5, 6, 11, 8, 9, 10, 4],
    eo_change: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const MOVE_F: Move = Move {
    cp_change: [0, 1, 5, 2, 3, 4, 6, 7],
    co_change: [0, 0, -1, 1, -1, 1, 0, 0],
    ep_change: [0, 1, 6, 3, 4, 5, 8, 2, 7, 9, 10, 11],
    eo_change: [0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0],
};

pub const MOVE_B: Move = Move {
    cp_change: [7, 0, 2, 3, 4, 5, 1, 6],
    co_change: [-1, 1, 0, 0, 0, 0, -1, 1],
    ep_change: [4, 1, 2, 3, 10, 0, 6, 7, 8, 9, 5, 11],
    eo_change: [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0],
};

