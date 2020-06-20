use crate::cube::*;
use std::io::Write;

/**
 * A struct holding pruning information for certain subgroups of the
 * Rubik's Cube.
 *
 * Each pruning table provides a lower bound on how many moves are
 * needed to transform a given state into the solved state within each subgroup.
 * These tables are obtained from `pruning.rs`.
 */
pub struct PruningTables {
    /// A pruning table representing the subgroup of corner permutation and orientation.
    pub corners: Vec<u8>,
    /// A pruning table representing the subgroup of edge orientation.
    pub eo: Vec<u8>,
    /// A pruning table representing the subgroup of edge permutation.
    pub ep: Vec<u8>,
}

impl PruningTables {
    pub fn default_tables() -> Self {
        let corners = std::fs::read("corners.pt").unwrap();
        let edges_o = std::fs::read("edges_o.pt").unwrap();
        let edges_p = std::fs::read("edges_p.pt").unwrap();
        PruningTables {
            corners,
            eo: edges_o,
            ep: edges_p,
        }
    }

    /// Computes a lower bound on the number of moves needed to
    /// solve the given state, based on the pruning table values.
    pub fn compute_h_value(&self, state: &CubeState) -> u8 {
        let (corners, eo, ep) = get_index_of_state(&state);
        std::cmp::max(
            self.corners[corners as usize],
            std::cmp::max(self.eo[eo as usize], self.ep[ep as usize]),
        )
    }
}

fn iddfs(
    starting_state: &CubeState,
    depth: u8,
    mut bv: &mut [u8],
    prop_func: &dyn Fn(&CubeState) -> usize,
    tag: String,
) {
    if depth < 1 {
        panic!("Depth must be positive");
    }
    for d in 1..depth {
        println!("Building {} pruning table for depth {}...", tag, d);
        iddfs_search(&starting_state, d, d, &mut bv, 0, &prop_func);
    }
}

fn iddfs_search(
    state: &CubeState,
    original_depth: u8,
    d: u8,
    mut bv: &mut [u8],
    allowed_moves: u8,
    prop_func: &dyn Fn(&CubeState) -> usize,
) {
    if d == 0 {
        // cool, we're at depth d now.
        let index = prop_func(state);
        if index > 0 && bv[index] == 0 {
            bv[index] = original_depth;
        }
    } else {
        for m in ALL_MOVES
            .iter()
            .filter(|mo| (1 << get_basemove_pos(mo.basemove)) & allowed_moves == 0)
        {
            let new_state = state.apply_move_instance(m);
            let new_allowed_moves = get_allowed_post_moves(allowed_moves, Some(m.basemove));
            iddfs_search(
                &new_state,
                original_depth,
                d - 1,
                &mut bv,
                new_allowed_moves,
                &prop_func,
            );
        }
    }
}

fn write_table(table: &[u8], filename: String) {
    let mut file = std::fs::File::create(filename).expect("Unable to create file.");
    file.write(table).expect("Unable to write to file.");
}

pub fn generate_pruning_table_corners(filename: String) -> bool {
    let solved = CubeState::default();
    let mut table = vec![0 as u8; 88179840];
    iddfs(
        &solved,
        9,
        &mut table,
        &|state: &CubeState| {
            let (corner, _, _) = get_index_of_state(state);
            corner as usize
        },
        String::from("corners"),
    );
    write_table(&*table, filename);
    true
}

pub fn generate_pruning_table_eo(filename: String) -> bool {
    let solved = CubeState::default();
    let mut table = vec![0 as u8; 2048];
    iddfs(
        &solved,
        8,
        &mut table,
        &|state| {
            let (_, index, _) = get_index_of_state(&state);
            index as usize
        },
        String::from("EO"),
    );
    write_table(&*table, filename);
    true
}

pub fn generate_pruning_table_ep(filename: String) -> bool {
    let solved = CubeState::default();
    let mut table = vec![0 as u8; 479001600];
    iddfs(
        &solved,
        9,
        &mut table,
        &|state| {
            let (_, _, index) = get_index_of_state(&state);
            index as usize
        },
        String::from("EP"),
    );
    write_table(&*table, filename);
    true
}
