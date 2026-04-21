use crate::lexer::Lexer;
use crate::parser::ast::{Expression, Operator};
use crate::parser::parse::Parser;

#[test]
fn parse_number() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, Expression::Number(42));
}

#[test]
fn parse_coordinate_x() {
    let mut lexer = Lexer::new("x");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, Expression::CoordinateX);
}

#[test]
fn parse_coordinate_y() {
    let mut lexer = Lexer::new("y");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, Expression::CoordinateY);
}

#[test]
fn parse_addition() {
    let mut lexer = Lexer::new("x + 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateX),
            operator: Operator::Add,
            right: Box::new(Expression::Number(3)),
        }
    );
}

#[test]
fn parse_subtraction() {
    let mut lexer = Lexer::new("y - 1");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateY),
            operator: Operator::Subtract,
            right: Box::new(Expression::Number(1)),
        }
    );
}

#[test]
fn parse_multiplication() {
    let mut lexer = Lexer::new("2 * x");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::Number(2)),
            operator: Operator::Multiply,
            right: Box::new(Expression::CoordinateX),
        }
    );
}

#[test]
fn parse_division() {
    let mut lexer = Lexer::new("x / 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateX),
            operator: Operator::Divide,
            right: Box::new(Expression::Number(2)),
        }
    );
}

#[test]
fn parse_exponentiation() {
    let mut lexer = Lexer::new("x ^ 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateX),
            operator: Operator::Power,
            right: Box::new(Expression::Number(2)),
        }
    );
}

#[test]
fn parse_exponentiation_right_associative() {
    let mut lexer = Lexer::new("2 ^ 3 ^ 4");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::Number(2)),
            operator: Operator::Power,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Number(3)),
                operator: Operator::Power,
                right: Box::new(Expression::Number(4)),
            }),
        }
    );
}

#[test]
fn parse_equal() {
    let mut lexer = Lexer::new("x = 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateX),
            operator: Operator::Equal,
            right: Box::new(Expression::Number(3)),
        }
    );
}

#[test]
fn parse_less_than() {
    let mut lexer = Lexer::new("x < 8");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateX),
            operator: Operator::LessThan,
            right: Box::new(Expression::Number(8)),
        }
    );
}

#[test]
fn parse_greater_than() {
    let mut lexer = Lexer::new("y > 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateY),
            operator: Operator::GreaterThan,
            right: Box::new(Expression::Number(0)),
        }
    );
}

#[test]
fn parse_and() {
    let mut lexer = Lexer::new("x = 3 and y = 5");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::CoordinateX),
                operator: Operator::Equal,
                right: Box::new(Expression::Number(3)),
            }),
            operator: Operator::And,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::CoordinateY),
                operator: Operator::Equal,
                right: Box::new(Expression::Number(5)),
            }),
        }
    );
}

#[test]
fn parse_or() {
    let mut lexer = Lexer::new("x = 0 or y = 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::CoordinateX),
                operator: Operator::Equal,
                right: Box::new(Expression::Number(0)),
            }),
            operator: Operator::Or,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::CoordinateY),
                operator: Operator::Equal,
                right: Box::new(Expression::Number(0)),
            }),
        }
    );
}

#[test]
fn parse_not() {
    let mut lexer = Lexer::new("not x = 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::UnaryNot {
            operand: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::CoordinateX),
                operator: Operator::Equal,
                right: Box::new(Expression::Number(3)),
            }),
        }
    );
}

#[test]
fn parse_parenthesized_expression() {
    let mut lexer = Lexer::new("(x - 8) ^ 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::CoordinateX),
                operator: Operator::Subtract,
                right: Box::new(Expression::Number(8)),
            }),
            operator: Operator::Power,
            right: Box::new(Expression::Number(2)),
        }
    );
}

#[test]
fn parse_multiplication_before_addition() {
    let mut lexer = Lexer::new("y - 2 * x");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateY),
            operator: Operator::Subtract,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Number(2)),
                operator: Operator::Multiply,
                right: Box::new(Expression::CoordinateX),
            }),
        }
    );
}

#[test]
fn parse_circle_condition() {
    let mut lexer = Lexer::new("(x - 8) ^ 2 + (y - 8) ^ 2 < 16");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
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
        }
    );
}

#[test]
fn parse_less_than_or_equal() {
    let mut lexer = Lexer::new("x <= 8");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateX),
            operator: Operator::LessThanOrEqual,
            right: Box::new(Expression::Number(8)),
        }
    );
}

#[test]
fn parse_greater_than_or_equal() {
    let mut lexer = Lexer::new("y >= 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse_expression().unwrap();

    assert_eq!(
        expression,
        Expression::BinaryOperation {
            left: Box::new(Expression::CoordinateY),
            operator: Operator::GreaterThanOrEqual,
            right: Box::new(Expression::Number(0)),
        }
    );
}

#[test]
fn parse_empty_expression() {
    let mut parser = Parser::new(vec![]);
    let error = parser.parse_expression().unwrap_err();

    assert_eq!(error.message, "unexpected end of input");
}
