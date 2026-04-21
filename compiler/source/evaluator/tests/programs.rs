use crate::evaluator::evaluate::Evaluator;
use crate::evaluator::instruction::{EvaluatedProgram, Instruction};
use crate::parser::ast::{
    ColorEntry, ColorValue, Expression, Format, Operator, Point, Program, Statement,
};

#[test]
fn evaluate_minimal_program() {
    let program = Program {
        statements: vec![Statement::Grid {
            width: 10,
            height: 10,
        }],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(
        result,
        EvaluatedProgram {
            grid_width: 10,
            grid_height: 10,
            instructions: vec![],
        }
    );
}

#[test]
fn evaluate_program_with_draw() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Draw {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::CoordinateX),
                    operator: Operator::LessThan,
                    right: Box::new(Expression::Number(5)),
                },
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            },
        ],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(result.instructions.len(), 1);
    assert!(matches!(&result.instructions[0], Instruction::Draw { .. }));
}

#[test]
fn evaluate_program_with_erase() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Erase {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::CoordinateX),
                    operator: Operator::LessThan,
                    right: Box::new(Expression::Number(5)),
                },
            },
        ],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(result.instructions.len(), 1);
    assert!(matches!(&result.instructions[0], Instruction::Erase { .. }));
}

#[test]
fn evaluate_program_with_clear() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Clear,
        ],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(result.instructions.len(), 1);
    assert!(matches!(&result.instructions[0], Instruction::Clear));
}

#[test]
fn evaluate_program_with_shapes() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 16,
                height: 16,
            },
            Statement::Pixel {
                point: Point { x: 5, y: 5 },
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            },
            Statement::Line {
                from: Point { x: 0, y: 0 },
                to: Point { x: 10, y: 10 },
                color: ColorValue::Hexadecimal("00FF00".to_string()),
            },
            Statement::Rectangle {
                from: Point { x: 1, y: 1 },
                to: Point { x: 8, y: 8 },
                color: ColorValue::Hexadecimal("0000FF".to_string()),
            },
            Statement::Triangle {
                first: Point { x: 0, y: 0 },
                second: Point { x: 5, y: 10 },
                third: Point { x: 10, y: 0 },
                color: ColorValue::Hexadecimal("FFFF00".to_string()),
            },
            Statement::Circle {
                center: Point { x: 8, y: 8 },
                radius: 5,
                color: ColorValue::Hexadecimal("FF00FF".to_string()),
            },
        ],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(result.instructions.len(), 5);
}

#[test]
fn evaluate_program_with_export() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::Export {
                filename: "output".to_string(),
                format: Format::Png,
                scale: Some(4),
            },
        ],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(result.instructions.len(), 1);
    assert!(matches!(&result.instructions[0], Instruction::Export { .. }));
}

#[test]
fn evaluate_program_with_color_block() {
    let program = Program {
        statements: vec![
            Statement::Grid {
                width: 10,
                height: 10,
            },
            Statement::ColorBlock {
                entries: vec![
                    ColorEntry {
                        name: "red".to_string(),
                        color: "FF0000".to_string(),
                    },
                    ColorEntry {
                        name: "blue".to_string(),
                        color: "0000FF".to_string(),
                    },
                ],
            },
        ],
    };
    let result = Evaluator::evaluate(program).unwrap();
    assert_eq!(result.instructions.len(), 1);
    assert!(matches!(
        &result.instructions[0],
        Instruction::ColorBlock { .. }
    ));
}
