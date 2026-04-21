use crate::lexer::Lexer;
use crate::parser::ast::{Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_clear() {
    let mut lexer = Lexer::new("clear");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Clear],
        }
    );
}
