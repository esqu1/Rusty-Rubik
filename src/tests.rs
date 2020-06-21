#[cfg(test)]
mod tests {
    use crate::cube::*;
    use crate::parser::*;
    use crate::pruning::*;
    use crate::solver::*;

    // PARSER TESTS
    #[test]
    fn parse_single_move() {
        assert_eq!(
            parse_scramble("U").unwrap()[0],
            MoveInstance {
                basemove: BaseMoveToken::U,
                dir: Direction::Normal,
            }
        );
    }

    #[test]
    fn parse_single_move_with_spaces() {
        assert_eq!(
            parse_scramble("U   \t").unwrap()[0],
            MoveInstance {
                basemove: BaseMoveToken::U,
                dir: Direction::Normal,
            }
        );
    }

    #[test]
    fn parse_multi_moves() {
        assert_eq!(
            parse_scramble("U2 F'").unwrap()[0],
            MoveInstance {
                basemove: BaseMoveToken::U,
                dir: Direction::Double,
            }
        );
        assert_eq!(
            parse_scramble("U2 F'").unwrap()[1],
            MoveInstance {
                basemove: BaseMoveToken::F,
                dir: Direction::Prime,
            }
        )
    }

    // CUBE STRUCTURE TESTS

    #[test]
    fn create_new_move_instance() {
        let move_instance = MoveInstance::new(BaseMoveToken::F, Direction::Prime);
        assert_eq!(move_instance.basemove, BaseMoveToken::F);
        assert_eq!(move_instance.dir, Direction::Prime)
    }

    #[test]
    fn index_of_solved_state() {
        let (c, eo, ep) = get_index_of_state(&CubeState::default());
        assert_eq!(c, 0);
        assert_eq!(eo, 0);
        assert_eq!(ep, 0);
    }

    // PRUNING TABLE TESTS
    #[test]
    #[ignore]
    fn pruning_table_of_solved_is_zero() {
        let tables = PruningTables::default_tables();
        assert_eq!(tables.eo[0], 0);
        assert_eq!(tables.ep[0], 0);
        assert_eq!(tables.corners[0], 0);
    }

    #[test]
    #[ignore]
    fn one_move_pruning_top() {
        let tables = PruningTables::default_tables();
        let solved = CubeState::default();
        let twisted =
            solved.apply_move_instance(&MoveInstance::new(BaseMoveToken::U, Direction::Normal));
        let (c, eo, ep) = get_index_of_state(&twisted);
        assert_eq!(tables.corners[c as usize], 1);
        assert_eq!(tables.eo[eo as usize], 0);
        assert_eq!(tables.ep[ep as usize], 1);
    }

    #[test]
    #[ignore]
    fn one_move_pruning_front() {
        let tables = PruningTables::default_tables();
        let solved = CubeState::default();
        let twisted =
            solved.apply_move_instance(&MoveInstance::new(BaseMoveToken::F, Direction::Normal));
        let (c, eo, ep) = get_index_of_state(&twisted);
        assert_eq!(tables.corners[c as usize], 1);
        assert_eq!(tables.eo[eo as usize], 1);
        assert_eq!(tables.ep[ep as usize], 1);
    }

    #[test]
    #[ignore]
    fn u_perm_optimal() {
        let tables = PruningTables::default_tables();
        let scramble = MoveSequence(parse_scramble("R U' R U R U R U' R' U' R2").unwrap());
        let solved = CubeState::default();
        let twisted = solved.apply_move_instances(&scramble);
        let solver = IDASolver::new(twisted, &tables);

        let solution = solver.solve();
        assert_eq!(solution.get_moves().len(), 9);
    }
}
