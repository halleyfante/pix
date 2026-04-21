use crate::lexer::Lexer;
use crate::parser::ast::{ColorValue, Point, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_mirror() {
    let mut lexer = Lexer::new("mirror (0, 4) to (7, 4) {\n    pixel (1, 1) with #FF0000\n}");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Mirror {
                from: Point { x: 0, y: 4 },
                to: Point { x: 7, y: 4 },
                statements: vec![Statement::Pixel {
                    point: Point { x: 1, y: 1 },
                    color: ColorValue::Hexadecimal("FF0000".to_string()),
                }],
            }],
        }
    );
}

#[test]
fn parse_empty_mirror() {
    let mut lexer = Lexer::new("mirror (0, 0) to (8, 8) {}");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Mirror {
                from: Point { x: 0, y: 0 },
                to: Point { x: 8, y: 8 },
                statements: vec![],
            }],
        }
    );
}
