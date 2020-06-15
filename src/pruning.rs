use crate::cube::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

const MAX_DEPTH: i8 = 9;

pub fn generate_pruning_table_corners(filename: String) -> Result<bool, &'static str> {
    let mut map = HashMap::<[i8; 8], i8>::new();
    let solved = CubeState::default();
    let mut queue = VecDeque::new();
    let mut curr_depth = 0;
    queue.push_back((solved, 0));
    while queue.len() > 0 {
        if let Some((current, depth)) = queue.pop_front() {
            if depth > curr_depth {
                println!("Building pruning table for depth {}...", depth);
                curr_depth = depth;
            }
            map.insert(current.cp, depth);
            if depth < MAX_DEPTH {
                for m in ALL_MOVES.iter() {
                    let new_state = current.apply_move_instance(m);
                    if let None = map.get(&new_state.cp) {
                        queue.push_back((new_state, depth + 1));
                    }
                }
            }
        }
    }
    let bytes = bincode::serialize(&map).unwrap();
    let mut file = File::create(filename).expect("Unable to create file.");
    file.write(bytes.as_slice())
        .expect("Unable to write to file.");
    Ok(true)
}
