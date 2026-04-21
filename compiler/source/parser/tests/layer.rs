use crate::lexer::Lexer;
use crate::parser::ast::{ColorValue, Expression, Operator, Point, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_layer() {
    let mut lexer = Lexer::new("layer background {\n    pixel (0, 0) with #FF0000\n}");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Layer {
                name: "background".to_string(),
                statements: vec![Statement::Pixel {
                    point: Point { x: 0, y: 0 },
                    color: ColorValue::Hexadecimal("FF0000".to_string()),
                }],
            }],
        }
    );
}

#[test]
fn parse_empty_layer() {
    let mut lexer = Lexer::new("layer empty {}");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Layer {
                name: "empty".to_string(),
                statements: vec![],
            }],
        }
    );
}

#[test]
fn parse_layer_missing_name() {
    let mut lexer = Lexer::new("layer { }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected layer name, found LeftBrace");
}

#[test]
fn parse_layer_with_multiple_statements() {
    let mut lexer = Lexer::new("layer foreground {\n    draw x = 0 with #FF0000\n    clear\n}");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Layer {
                name: "foreground".to_string(),
                statements: vec![
                    Statement::Draw {
                        condition: Expression::BinaryOperation {
                            left: Box::new(Expression::CoordinateX),
                            operator: Operator::Equal,
                            right: Box::new(Expression::Number(0)),
                        },
                        color: ColorValue::Hexadecimal("FF0000".to_string()),
                    },
                    Statement::Clear,
                ],
            }],
        }
    );
}
