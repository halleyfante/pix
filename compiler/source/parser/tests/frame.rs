use crate::lexer::Lexer;
use crate::parser::ast::{Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_frame() {
    let mut lexer = Lexer::new("frame 200");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Frame { delay: 200 }],
        }
    );
}

#[test]
fn parse_frame_missing_delay() {
    let mut lexer = Lexer::new("frame");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "unexpected end of input");
}
