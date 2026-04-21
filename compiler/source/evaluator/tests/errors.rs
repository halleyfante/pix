use crate::evaluator::evaluate::Evaluator;
use crate::evaluator::instruction::EvaluateError;
use crate::parser::ast::{ColorValue, Expression, Operator, Program, Statement};

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

#[test]
fn error_missing_grid() {
    let program = Program {
        statements: vec![],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "missing grid statement".to_string(),
        }
    );
}

#[test]
fn error_duplicate_grid() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Grid {
                width: 20,
                height: 20,
            },
        ],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "duplicate grid statement".to_string(),
        }
    );
}

#[test]
fn error_grid_zero_width() {
    let program = Program {
        statements: vec![Statement::Grid {
            width: 0,
            height: 10,
        }],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "grid dimensions must be greater than zero".to_string(),
        }
    );
}

#[test]
fn error_grid_zero_height() {
    let program = Program {
        statements: vec![Statement::Grid {
            width: 10,
            height: 0,
        }],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "grid dimensions must be greater than zero".to_string(),
        }
    );
}

#[test]
fn error_draw_condition_not_boolean() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Draw {
                condition: Expression::CoordinateX,
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            },
        ],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "draw condition must be a boolean expression".to_string(),
        }
    );
}

#[test]
fn error_erase_condition_not_boolean() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Erase {
                condition: binary(Expression::CoordinateX, Operator::Add, number(1)),
            },
        ],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "erase condition must be a boolean expression".to_string(),
        }
    );
}

#[test]
fn error_division_by_zero() {
    let expression = binary(number(10), Operator::Divide, number(0));
    let result = Evaluator::evaluate_expression(&expression, 0, 0);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "division by zero".to_string(),
        }
    );
}

#[test]
fn error_not_requires_boolean() {
    let expression = Expression::UnaryNot {
        operand: Box::new(number(5)),
    };
    let result = Evaluator::evaluate_expression(&expression, 0, 0);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "operator 'not' requires a boolean operand".to_string(),
        }
    );
}

#[test]
fn error_add_requires_numbers() {
    let expression = binary(
        binary(number(1), Operator::Equal, number(1)),
        Operator::Add,
        number(5),
    );
    let result = Evaluator::evaluate_expression(&expression, 0, 0);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "operator '+' requires number operands".to_string(),
        }
    );
}

#[test]
fn error_duplicate_layer_name() {
    let program = Program {
        statements: vec![
            Statement::Grid { width: 8, height: 8 },
            Statement::Layer {
                name: "fundo".to_string(),
                statements: vec![],
            },
            Statement::Layer {
                name: "fundo".to_string(),
                statements: vec![],
            },
        ],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "duplicate layer name 'fundo'".to_string(),
        }
    );
}

#[test]
fn error_duplicate_layer_name_nested_in_mirror() {
    let program = Program {
        statements: vec![
            Statement::Grid { width: 8, height: 8 },
            Statement::Layer {
                name: "fundo".to_string(),
                statements: vec![],
            },
            Statement::Mirror {
                from: crate::parser::ast::Point { x: 0, y: 0 },
                to: crate::parser::ast::Point { x: 7, y: 7 },
                statements: vec![Statement::Layer {
                    name: "fundo".to_string(),
                    statements: vec![],
                }],
            },
        ],
    };
    let result = Evaluator::evaluate(program);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "duplicate layer name 'fundo'".to_string(),
        }
    );
}

#[test]
fn error_and_requires_booleans() {
    let expression = binary(number(1), Operator::And, number(2));
    let result = Evaluator::evaluate_expression(&expression, 0, 0);
    assert_eq!(
        result.unwrap_err(),
        EvaluateError {
            message: "operator 'and' requires boolean operands".to_string(),
        }
    );
}
