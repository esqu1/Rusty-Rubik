mod cube;
mod parser;

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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
