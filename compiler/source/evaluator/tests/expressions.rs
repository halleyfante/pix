use crate::evaluator::evaluate::Evaluator;
use crate::evaluator::instruction::Value;
use crate::parser::ast::{Expression, Operator};

fn number(value: u32) -> Expression {
    Expression::Number(value)
}

fn binary(left: Expression, operator: Operator, right: Expression) -> Expression {
    Expression::BinaryOperation {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    }
}

fn not(operand: Expression) -> Expression {
    Expression::UnaryNot {
        operand: Box::new(operand),
    }
}

#[test]
fn evaluate_number_literal() {
    let expression = number(42);
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(42));
}

#[test]
fn evaluate_coordinate_x() {
    let result = Evaluator::evaluate_expression(&Expression::CoordinateX, 5, 10).unwrap();
    assert_eq!(result, Value::Number(5));
}

#[test]
fn evaluate_coordinate_y() {
    let result = Evaluator::evaluate_expression(&Expression::CoordinateY, 5, 10).unwrap();
    assert_eq!(result, Value::Number(10));
}

#[test]
fn evaluate_addition() {
    let expression = binary(number(3), Operator::Add, number(4));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(7));
}

#[test]
fn evaluate_subtraction() {
    let expression = binary(number(10), Operator::Subtract, number(3));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(7));
}

#[test]
fn evaluate_subtraction_negative_result() {
    let expression = binary(number(3), Operator::Subtract, number(10));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(-7));
}

#[test]
fn evaluate_multiplication() {
    let expression = binary(number(3), Operator::Multiply, number(4));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(12));
}

#[test]
fn evaluate_division() {
    let expression = binary(number(10), Operator::Divide, number(3));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(3));
}

#[test]
fn evaluate_power() {
    let expression = binary(number(2), Operator::Power, number(10));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Number(1024));
}

#[test]
fn evaluate_equal_true() {
    let expression = binary(number(5), Operator::Equal, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn evaluate_equal_false() {
    let expression = binary(number(5), Operator::Equal, number(3));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn evaluate_less_than() {
    let expression = binary(number(3), Operator::LessThan, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn evaluate_greater_than() {
    let expression = binary(number(5), Operator::GreaterThan, number(3));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn evaluate_less_than_or_equal() {
    let expression = binary(number(3), Operator::LessThanOrEqual, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));

    let expression = binary(number(5), Operator::LessThanOrEqual, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));

    let expression = binary(number(6), Operator::LessThanOrEqual, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn evaluate_greater_than_or_equal() {
    let expression = binary(number(5), Operator::GreaterThanOrEqual, number(3));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));

    let expression = binary(number(5), Operator::GreaterThanOrEqual, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));

    let expression = binary(number(4), Operator::GreaterThanOrEqual, number(5));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn evaluate_and() {
    let expression = binary(
        binary(number(1), Operator::Equal, number(1)),
        Operator::And,
        binary(number(2), Operator::Equal, number(2)),
    );
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn evaluate_or() {
    let expression = binary(
        binary(number(1), Operator::Equal, number(2)),
        Operator::Or,
        binary(number(2), Operator::Equal, number(2)),
    );
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn evaluate_not_true() {
    let expression = not(binary(number(1), Operator::Equal, number(1)));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn evaluate_not_false() {
    let expression = not(binary(number(1), Operator::Equal, number(2)));
    let result = Evaluator::evaluate_expression(&expression, 0, 0).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn evaluate_expression_with_coordinates() {
    // x + y at position (3, 4) = 7
    let expression = binary(Expression::CoordinateX, Operator::Add, Expression::CoordinateY);
    let result = Evaluator::evaluate_expression(&expression, 3, 4).unwrap();
    assert_eq!(result, Value::Number(7));
}

#[test]
fn evaluate_nested_expression() {
    // (x * 2) + (y * 3) at position (2, 3) = 4 + 9 = 13
    let expression = binary(
        binary(Expression::CoordinateX, Operator::Multiply, number(2)),
        Operator::Add,
        binary(Expression::CoordinateY, Operator::Multiply, number(3)),
    );
    let result = Evaluator::evaluate_expression(&expression, 2, 3).unwrap();
    assert_eq!(result, Value::Number(13));
}

#[test]
fn evaluate_complex_boolean_expression() {
    // (x > 2) and (y < 8) at position (5, 3) = true and true = true
    let expression = binary(
        binary(Expression::CoordinateX, Operator::GreaterThan, number(2)),
        Operator::And,
        binary(Expression::CoordinateY, Operator::LessThan, number(8)),
    );
    let result = Evaluator::evaluate_expression(&expression, 5, 3).unwrap();
    assert_eq!(result, Value::Boolean(true));
}
