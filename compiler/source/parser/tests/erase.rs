use crate::lexer::Lexer;
use crate::parser::ast::{Expression, Operator, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_erase_simple_condition() {
    let mut lexer = Lexer::new("erase y = 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Erase {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::CoordinateY),
                    operator: Operator::Equal,
                    right: Box::new(Expression::Number(0)),
                },
            }],
        }
    );
}

#[test]
fn parse_erase_compound_condition() {
    let mut lexer = Lexer::new("erase x > 0 and x < 4 and y > 0 and y < 4");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Erase {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::BinaryOperation {
                            left: Box::new(Expression::BinaryOperation {
                                left: Box::new(Expression::CoordinateX),
                                operator: Operator::GreaterThan,
                                right: Box::new(Expression::Number(0)),
                            }),
                            operator: Operator::And,
                            right: Box::new(Expression::BinaryOperation {
                                left: Box::new(Expression::CoordinateX),
                                operator: Operator::LessThan,
                                right: Box::new(Expression::Number(4)),
                            }),
                        }),
                        operator: Operator::And,
                        right: Box::new(Expression::BinaryOperation {
                            left: Box::new(Expression::CoordinateY),
                            operator: Operator::GreaterThan,
                            right: Box::new(Expression::Number(0)),
                        }),
                    }),
                    operator: Operator::And,
                    right: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::CoordinateY),
                        operator: Operator::LessThan,
                        right: Box::new(Expression::Number(4)),
                    }),
                },
            }],
        }
    );
}

#[test]
fn parse_erase_missing_condition() {
    let mut lexer = Lexer::new("erase");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "unexpected end of input");
}
