#[cfg(test)]
mod tests {
    use crate::cube::*;
    use crate::parser::*;

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
    fn pruning_table_of_solved_is_zero() {
        let corners = std::fs::read("corners.pt").unwrap();
        let edges_o = std::fs::read("edges_o.pt").unwrap();
        let edges_p = std::fs::read("edges_p.pt").unwrap();
        assert_eq!(edges_o[0], 0);
        assert_eq!(edges_p[0], 0);
        assert_eq!(corners[0], 0);
    }

    #[test]
    fn one_move_pruning_top() {
        let corners = std::fs::read("corners.pt").unwrap();
        let edges_o = std::fs::read("edges_o.pt").unwrap();
        let edges_p = std::fs::read("edges_p.pt").unwrap();
        let solved = CubeState::default();
        let twisted =
            solved.apply_move_instance(&MoveInstance::new(BaseMoveToken::U, Direction::Normal));
        let (c, eo, ep) = get_index_of_state(&twisted);
        assert_eq!(corners[c as usize], 1);
        assert_eq!(edges_o[eo as usize], 0);
        assert_eq!(edges_p[ep as usize], 1);
    }

    #[test]
    fn one_move_pruning_front() {
        let corners = std::fs::read("corners.pt").unwrap();
        let edges_o = std::fs::read("edges_o.pt").unwrap();
        let edges_p = std::fs::read("edges_p.pt").unwrap();
        let solved = CubeState::default();
        let twisted =
            solved.apply_move_instance(&MoveInstance::new(BaseMoveToken::F, Direction::Normal));
        let (c, eo, ep) = get_index_of_state(&twisted);
        assert_eq!(corners[c as usize], 1);
        assert_eq!(edges_o[eo as usize], 1);
        assert_eq!(edges_p[ep as usize], 1);
    }

    #[test]
    fn te() {
        use crate::cube::BaseMoveToken::*;
        use crate::cube::Direction::*;
        let seq = vec![
            MoveInstance {
                basemove: F,
                dir: Prime,
            },
            MoveInstance {
                basemove: U,
                dir: Normal,
            },
            MoveInstance {
                basemove: F,
                dir: Prime,
            },
        ];
        println!("{:b}", allowed_moves_after_seq(&seq));
        //6180
    }
}
