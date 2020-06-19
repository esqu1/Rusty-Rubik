use crate::cube::*;
use std::io::Write;


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
        iddfs_search(&starting_state, d, d, &mut bv, &prop_func);
    }
}

fn iddfs_search(
    state: &CubeState,
    original_depth: u8,
    d: u8,
    mut bv: &mut [u8],
    prop_func: &dyn Fn(&CubeState) -> usize,
) {
    if d == 0 {
        // cool, we're at depth d now. 
        let index = prop_func(state);
        if bv[index] != 0 {
            bv[index] = original_depth;
        }
    } else {
        for m in ALL_MOVES.iter() {
            let new_state = state.apply_move_instance(m);
            iddfs_search(&new_state, original_depth, d - 1, &mut bv, &prop_func);
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
            let (corner, _) = get_index_of_state(state);
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
        9,
        &mut table,
        &|state| {
            let index = get_index_of_orientation(&state.eo, 2);
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
            let index = get_index_of_permutation(&state.ep);
            index as usize
        },
        String::from("EP"),
    );
    write_table(&*table, filename);
    true
}
// pub fn generate_pruning_table_corners(filename: String) -> Result<bool, &'static str> {
//     let mut table = vec![0 as u8; 88179840];
//     let solved = CubeState::default();
//     let mut queue = VecDeque::new();
//     let mut curr_depth = 0;
//     queue.push_back((solved, 0));
//     while queue.len() > 0 {
//         if let Some((current, depth)) = queue.pop_front() {
//             if depth > curr_depth {
//                 println!("Building corners pruning table for depth {}...", depth);
//                 curr_depth = depth;
//             }
//             let (corner, _) = get_index_of_state(&current);
//             table[corner as usize] = depth;
//             if depth < 6 {
//                 for m in ALL_MOVES.iter() {
//                     let new_state = current.apply_move_instance(m);
//                     let (c, _) = get_index_of_state(&new_state);
//                     if table[c as usize] == 0 {
//                         queue.push_back((new_state, depth + 1));
//                     }
//                 }
//             }
//         }
//     }

//     let mut file = File::create(filename).expect("Unable to create file.");
//     file.write(&*table).expect("Unable to write to file.");
//     Ok(true)
// }

// pub fn generate_pruning_table_ep(filename: String) -> Result<bool, &'static str> {
//     let mut table = vec![0 as u8; 479001600];
//     let solved = CubeState::default();
//     let mut queue = VecDeque::new();
//     let mut curr_depth = 0;
//     queue.push_back((solved, 0));
//     while queue.len() > 0 {
//         if let Some((current, depth)) = queue.pop_front() {
//             if depth > curr_depth {
//                 println!("Building EP pruning table for depth {}...", depth);
//                 curr_depth = depth;
//             }
//             let index = get_index_of_permutation(&current.ep);
//             table[index as usize] = depth;
//             if depth < 7 {
//                 for m in ALL_MOVES.iter() {
//                     let new_state = current.apply_move_instance(m);
//                     let i = get_index_of_permutation(&new_state.ep);
//                     if table[i as usize] == 0 {
//                         queue.push_back((new_state, depth + 1));
//                     }
//                 }
//             }
//         }
//     }
//     let mut file = File::create(filename).expect("Unable to create file.");
//     file.write(&*table).expect("Unable to write to file.");
//     Ok(true)
// }

// pub fn generate_pruning_table_eo(filename: String) -> Result<bool, &'static str> {
//     let mut table = vec![0 as u8; 2048];
//     let solved = CubeState::default();
//     let mut queue = VecDeque::new();
//     let mut curr_depth = 0;
//     queue.push_back((solved, 0));
//     while queue.len() > 0 {
//         if let Some((current, depth)) = queue.pop_front() {
//             if depth > curr_depth {
//                 println!("Building EO pruning table for depth {}...", depth);
//                 curr_depth = depth;
//             }
//             let index = get_index_of_orientation(&current.eo, 2);
//             table[index as usize] = depth;
//             if depth < MAX_DEPTH {
//                 for m in ALL_MOVES.iter() {
//                     let new_state = current.apply_move_instance(m);
//                     let i = get_index_of_orientation(&new_state.eo, 2);
//                     if table[i as usize] == 0 {
//                         queue.push_back((new_state, depth + 1));
//                     }
//                 }
//             }
//         }
//     }
//     let mut file = File::create(filename).expect("Unable to create file.");
//     file.write(&*table).expect("Unable to write to file.");
//     Ok(true)
// }
