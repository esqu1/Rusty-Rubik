#[cfg(test)]
mod tests {

    // PARSER TESTS
    #[test]
    fn parse_single_move() {
        assert_eq!(
            crate::parser::parse_scramble("U").unwrap()[0],
            crate::cube::MoveInstance {
                basemove: crate::cube::BaseMoveToken::U,
                dir: crate::cube::Direction::Normal,
            }
        );
    }

    #[test]
    fn parse_single_move_with_spaces() {
        assert_eq!(
            crate::parser::parse_scramble("U   \t").unwrap()[0],
            crate::cube::MoveInstance {
                basemove: crate::cube::BaseMoveToken::U,
                dir: crate::cube::Direction::Normal,
            }
        );
    }

    #[test]
    fn parse_multi_moves() {
        assert_eq!(
            crate::parser::parse_scramble("U2 F'").unwrap()[0],
            crate::cube::MoveInstance {
                basemove: crate::cube::BaseMoveToken::U,
                dir: crate::cube::Direction::Double,
            }
        );
        assert_eq!(
            crate::parser::parse_scramble("U2 F'").unwrap()[1],
            crate::cube::MoveInstance {
                basemove: crate::cube::BaseMoveToken::F,
                dir: crate::cube::Direction::Prime,
            }
        )
    }

    #[test]
    fn index_of_solved_state() {
        let (c, e) = crate::cube::get_index_of_state(&crate::cube::CubeState::default());
        assert_eq!(c, 0);
        assert_eq!(e, 0); 
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
