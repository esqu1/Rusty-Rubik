use crate::cube::*;
use std::collections::HashMap;
use std::collections::HashSet;

use priority_queue::PriorityQueue;
// naive bfs

pub trait Solver {
    fn new(state: CubeState) -> Self;
    fn get_start_state(&self) -> &CubeState;
    fn solve(&self) -> MoveSequence;
}

pub struct BFSSolver {}

pub struct DFSSolver {}

pub struct AStarSolver {
    start_state: CubeState,
}

impl Solver for AStarSolver {
    fn new(state: CubeState) -> Self {
        AStarSolver { start_state: state }
    }

    fn get_start_state(&self) -> &CubeState {
        &self.start_state
    }

    fn solve(&self) -> MoveSequence {
        let mut queue = PriorityQueue::new();
        let mut visited = HashSet::<CubeState>::new();
        let mut come_from = HashMap::<CubeState, (CubeState, MoveInstance)>::new();
        let mut g_scores = HashMap::<CubeState, i32>::new();

        // TODO: need to compress cube state
        queue.push(self.get_start_state().clone(), 0);
        g_scores.insert(self.get_start_state().clone(), 0);
        while queue.len() > 0 {
            if let Some((current, priority)) = queue.pop() {
                if current == CubeState::default() {
                    // we found the solved state!
                    break;
                }
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current.clone());
                // iterate through all moves
                for m in ALL_MOVES.iter() {
                    let new_state = current.apply_move_instance(m);
                    let new_g_score = priority - 1;
                    let neighbor_g_score = g_scores.get(&new_state).unwrap_or(&std::i32::MIN);
                    if new_g_score > *neighbor_g_score {
                        come_from.insert(new_state.clone(), (current.clone(), *m));
                        g_scores.insert(new_state.clone(), new_g_score);
                    }
                    if let None = queue.get(&new_state) {
                        queue.push(new_state, priority - 1);
                    } else if let Some((_, p)) = queue.get(&new_state) {
                        if *p < priority - 1 {
                            queue.push(new_state, priority - 1);
                        }
                    }
                }
            }
        }
        // now reconstruct the path
        let mut curr = CubeState::default();
        let mut path = vec![];
        while curr != self.get_start_state().clone() {
            if let Some((c, m)) = come_from.get(&curr) {
                path.push(m.clone());
                curr = c.clone();
            }
        }
        path.reverse();
        path
    }
}

pub struct IDASolver {
    start_state: CubeState,
}

impl Solver for IDASolver {
    fn new(state: CubeState) -> Self {
        IDASolver { start_state: state }
    }

    fn get_start_state(&self) -> &CubeState {
        &self.start_state
    }

    fn solve(&self) -> MoveSequence {
        vec![]
    }
}
