use crate::lexer::Lexer;
use crate::parser::ast::{Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_grid_statement() {
    let mut lexer = Lexer::new("grid 16 by 16");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Grid {
                width: 16,
                height: 16,
            }],
        }
    );
}

#[test]
fn parse_grid_different_dimensions() {
    let mut lexer = Lexer::new("grid 32 by 64");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Grid {
                width: 32,
                height: 64,
            }],
        }
    );
}

#[test]
fn parse_grid_missing_width() {
    let mut lexer = Lexer::new("grid by 16");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected number, found By");
}

#[test]
fn parse_grid_missing_by() {
    let mut lexer = Lexer::new("grid 16 16");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected By, found Number(16)");
}

#[test]
fn parse_grid_missing_height() {
    let mut lexer = Lexer::new("grid 16 by");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "unexpected end of input");
}
