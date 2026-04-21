use crate::lexer::Lexer;
use crate::parser::ast::{Point, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_move() {
    let mut lexer = Lexer::new("move (0, 0) to (3, 3) at (5, 5)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Move {
                from: Point { x: 0, y: 0 },
                to: Point { x: 3, y: 3 },
                destination: Point { x: 5, y: 5 },
            }],
        }
    );
}
