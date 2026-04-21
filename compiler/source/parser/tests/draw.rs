use crate::lexer::Lexer;
use crate::parser::ast::{ColorValue, Expression, Operator, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_draw_with_hex_color() {
    let mut lexer = Lexer::new("draw y = 0 with #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Draw {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::CoordinateY),
                    operator: Operator::Equal,
                    right: Box::new(Expression::Number(0)),
                },
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            }],
        }
    );
}

#[test]
fn parse_draw_with_complex_condition() {
    let mut lexer = Lexer::new("draw (x - 8) ^ 2 + (y - 8) ^ 2 < 16 with #00FF00");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Draw {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::BinaryOperation {
                            left: Box::new(Expression::BinaryOperation {
                                left: Box::new(Expression::CoordinateX),
                                operator: Operator::Subtract,
                                right: Box::new(Expression::Number(8)),
                            }),
                            operator: Operator::Power,
                            right: Box::new(Expression::Number(2)),
                        }),
                        operator: Operator::Add,
                        right: Box::new(Expression::BinaryOperation {
                            left: Box::new(Expression::BinaryOperation {
                                left: Box::new(Expression::CoordinateY),
                                operator: Operator::Subtract,
                                right: Box::new(Expression::Number(8)),
                            }),
                            operator: Operator::Power,
                            right: Box::new(Expression::Number(2)),
                        }),
                    }),
                    operator: Operator::LessThan,
                    right: Box::new(Expression::Number(16)),
                },
                color: ColorValue::Hexadecimal("00FF00".to_string()),
            }],
        }
    );
}

#[test]
fn parse_draw_missing_with() {
    let mut lexer = Lexer::new("draw y = 0 #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected With, found HexadecimalColor(\"FF0000\")");
}

#[test]
fn parse_draw_missing_color() {
    let mut lexer = Lexer::new("draw y = 0 with");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "unexpected end of input");
}
