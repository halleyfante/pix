use crate::parser::ast::{Expression, Operator, Program, Statement};

use super::instruction::{
    EvaluateError, EvaluatedProgram, Instruction, Value, ValueType,
};
use super::shapes;

pub struct Evaluator;

impl Evaluator {
    /// Evaluates an expression at a given (x, y) grid position.
    pub fn evaluate_expression(
        expression: &Expression,
        x: i64,
        y: i64,
    ) -> Result<Value, EvaluateError> {
        match expression {
            Expression::Number(number) => Ok(Value::Number(*number as i64)),
            Expression::CoordinateX => Ok(Value::Number(x)),
            Expression::CoordinateY => Ok(Value::Number(y)),
            Expression::UnaryNot { operand } => {
                let value = Self::evaluate_expression(operand, x, y)?;
                match value {
                    Value::Boolean(boolean) => Ok(Value::Boolean(!boolean)),
                    Value::Number(_) => Err(EvaluateError {
                        message: "operator 'not' requires a boolean operand".to_string(),
                    }),
                }
            }
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_value = Self::evaluate_expression(left, x, y)?;
                let right_value = Self::evaluate_expression(right, x, y)?;

                match operator {
                    Operator::Add
                    | Operator::Subtract
                    | Operator::Multiply
                    | Operator::Divide
                    | Operator::Power => {
                        let left_number = Self::expect_number(left_value, operator)?;
                        let right_number = Self::expect_number(right_value, operator)?;
                        Self::evaluate_arithmetic(left_number, operator, right_number)
                    }
                    Operator::Equal | Operator::LessThan | Operator::GreaterThan | Operator::LessThanOrEqual | Operator::GreaterThanOrEqual => {
                        let left_number = Self::expect_number(left_value, operator)?;
                        let right_number = Self::expect_number(right_value, operator)?;
                        Self::evaluate_comparison(left_number, operator, right_number)
                    }
                    Operator::And | Operator::Or => {
                        let left_boolean = Self::expect_boolean(left_value, operator)?;
                        let right_boolean = Self::expect_boolean(right_value, operator)?;
                        Self::evaluate_logical(left_boolean, operator, right_boolean)
                    }
                }
            }
        }
    }

    /// Checks the type of an expression without evaluating it.
    pub fn type_of_expression(
        expression: &Expression,
    ) -> Result<ValueType, EvaluateError> {
        match expression {
            Expression::Number(_) | Expression::CoordinateX | Expression::CoordinateY => {
                Ok(ValueType::Number)
            }
            Expression::UnaryNot { operand } => {
                let operand_type = Self::type_of_expression(operand)?;
                if operand_type != ValueType::Boolean {
                    return Err(EvaluateError {
                        message: "operator 'not' requires a boolean operand".to_string(),
                    });
                }
                Ok(ValueType::Boolean)
            }
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_type = Self::type_of_expression(left)?;
                let right_type = Self::type_of_expression(right)?;

                match operator {
                    Operator::Add
                    | Operator::Subtract
                    | Operator::Multiply
                    | Operator::Divide
                    | Operator::Power => {
                        Self::expect_type_number(&left_type, operator)?;
                        Self::expect_type_number(&right_type, operator)?;
                        Ok(ValueType::Number)
                    }
                    Operator::Equal | Operator::LessThan | Operator::GreaterThan | Operator::LessThanOrEqual | Operator::GreaterThanOrEqual => {
                        Self::expect_type_number(&left_type, operator)?;
                        Self::expect_type_number(&right_type, operator)?;
                        Ok(ValueType::Boolean)
                    }
                    Operator::And | Operator::Or => {
                        Self::expect_type_boolean(&left_type, operator)?;
                        Self::expect_type_boolean(&right_type, operator)?;
                        Ok(ValueType::Boolean)
                    }
                }
            }
        }
    }

    /// Validates a parsed program and converts it into an evaluated program ready for rendering.
    pub fn evaluate(program: Program) -> Result<EvaluatedProgram, EvaluateError> {
        let mut grid_width: Option<u32> = None;
        let mut grid_height: Option<u32> = None;
        let mut instructions = Vec::new();

        for statement in program.statements {
            match statement {
                Statement::Grid { width, height } => {
                    if grid_width.is_some() {
                        return Err(EvaluateError {
                            message: "duplicate grid statement".to_string(),
                        });
                    }
                    if width == 0 || height == 0 {
                        return Err(EvaluateError {
                            message: "grid dimensions must be greater than zero".to_string(),
                        });
                    }
                    grid_width = Some(width);
                    grid_height = Some(height);
                }
                Statement::Draw { condition, color } => {
                    let condition_type = Self::type_of_expression(&condition)?;
                    if condition_type != ValueType::Boolean {
                        return Err(EvaluateError {
                            message: "draw condition must be a boolean expression".to_string(),
                        });
                    }
                    instructions.push(Instruction::Draw { condition, color });
                }
                Statement::Erase { condition } => {
                    let condition_type = Self::type_of_expression(&condition)?;
                    if condition_type != ValueType::Boolean {
                        return Err(EvaluateError {
                            message: "erase condition must be a boolean expression".to_string(),
                        });
                    }
                    instructions.push(Instruction::Erase { condition });
                }
                Statement::Clear => {
                    instructions.push(Instruction::Clear);
                }
                Statement::Pixel { point, color } => {
                    let condition = shapes::pixel(&point);
                    instructions.push(Instruction::Pixel { condition, point, color });
                }
                Statement::Line { from, to, color } => {
                    let condition = shapes::line(&from, &to);
                    instructions.push(Instruction::Line { condition, from, to, color });
                }
                Statement::Rectangle { from, to, color } => {
                    let condition = shapes::rectangle(&from, &to);
                    instructions.push(Instruction::Rectangle { condition, from, to, color });
                }
                Statement::Triangle {
                    first,
                    second,
                    third,
                    color,
                } => {
                    let condition = shapes::triangle(&first, &second, &third);
                    instructions.push(Instruction::Triangle {
                        condition,
                        first,
                        second,
                        third,
                        color,
                    });
                }
                Statement::Circle {
                    center,
                    radius,
                    color,
                } => {
                    let condition = shapes::circle(&center, radius);
                    instructions.push(Instruction::Circle {
                        condition,
                        center,
                        radius,
                        color,
                    });
                }
                Statement::Export {
                    filename,
                    format,
                    scale,
                } => {
                    instructions.push(Instruction::Export {
                        filename,
                        format,
                        scale,
                    });
                }
                Statement::ColorBlock { entries } => {
                    instructions.push(Instruction::ColorBlock { entries });
                }
                Statement::Frame { delay } => {
                    instructions.push(Instruction::Frame { delay });
                }
                Statement::Copy { from, to, destination } => {
                    instructions.push(Instruction::Copy { from, to, destination });
                }
                Statement::Move { from, to, destination } => {
                    instructions.push(Instruction::Move { from, to, destination });
                }
                Statement::Layer { name, statements } => {
                    let inner = Self::evaluate_statements(statements)?;
                    instructions.push(Instruction::Layer { name, instructions: inner });
                }
                Statement::Mirror { from, to, statements } => {
                    let inner = Self::evaluate_statements(statements)?;
                    instructions.push(Instruction::Mirror { from, to, instructions: inner });
                }
            }
        }

        let grid_width = grid_width.ok_or_else(|| EvaluateError {
            message: "missing grid statement".to_string(),
        })?;
        let grid_height = grid_height.ok_or_else(|| EvaluateError {
            message: "missing grid statement".to_string(),
        })?;

        let mut seen_layer_names = std::collections::HashSet::new();
        Self::check_duplicate_layer_names(&instructions, &mut seen_layer_names)?;

        Ok(EvaluatedProgram {
            grid_width,
            grid_height,
            instructions,
        })
    }

    fn check_duplicate_layer_names(
        instructions: &[Instruction],
        seen: &mut std::collections::HashSet<String>,
    ) -> Result<(), EvaluateError> {
        for instruction in instructions {
            match instruction {
                Instruction::Layer { name, instructions } => {
                    if !seen.insert(name.clone()) {
                        return Err(EvaluateError {
                            message: format!("duplicate layer name '{}'", name),
                        });
                    }
                    Self::check_duplicate_layer_names(instructions, seen)?;
                }
                Instruction::Mirror { instructions, .. } => {
                    Self::check_duplicate_layer_names(instructions, seen)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn evaluate_statements(statements: Vec<Statement>) -> Result<Vec<Instruction>, EvaluateError> {
        let mut instructions = Vec::new();
        for statement in statements {
            match statement {
                Statement::Draw { condition, color } => {
                    let condition_type = Self::type_of_expression(&condition)?;
                    if condition_type != ValueType::Boolean {
                        return Err(EvaluateError {
                            message: "draw condition must be a boolean expression".to_string(),
                        });
                    }
                    instructions.push(Instruction::Draw { condition, color });
                }
                Statement::Erase { condition } => {
                    let condition_type = Self::type_of_expression(&condition)?;
                    if condition_type != ValueType::Boolean {
                        return Err(EvaluateError {
                            message: "erase condition must be a boolean expression".to_string(),
                        });
                    }
                    instructions.push(Instruction::Erase { condition });
                }
                Statement::Clear => {
                    instructions.push(Instruction::Clear);
                }
                Statement::Pixel { point, color } => {
                    let condition = shapes::pixel(&point);
                    instructions.push(Instruction::Pixel { condition, point, color });
                }
                Statement::Line { from, to, color } => {
                    let condition = shapes::line(&from, &to);
                    instructions.push(Instruction::Line { condition, from, to, color });
                }
                Statement::Rectangle { from, to, color } => {
                    let condition = shapes::rectangle(&from, &to);
                    instructions.push(Instruction::Rectangle { condition, from, to, color });
                }
                Statement::Triangle { first, second, third, color } => {
                    let condition = shapes::triangle(&first, &second, &third);
                    instructions.push(Instruction::Triangle { condition, first, second, third, color });
                }
                Statement::Circle { center, radius, color } => {
                    let condition = shapes::circle(&center, radius);
                    instructions.push(Instruction::Circle { condition, center, radius, color });
                }
                Statement::Export { filename, format, scale } => {
                    instructions.push(Instruction::Export { filename, format, scale });
                }
                Statement::ColorBlock { entries } => {
                    instructions.push(Instruction::ColorBlock { entries });
                }
                Statement::Frame { delay } => {
                    instructions.push(Instruction::Frame { delay });
                }
                Statement::Copy { from, to, destination } => {
                    instructions.push(Instruction::Copy { from, to, destination });
                }
                Statement::Move { from, to, destination } => {
                    instructions.push(Instruction::Move { from, to, destination });
                }
                Statement::Mirror { from, to, statements } => {
                    let inner = Self::evaluate_statements(statements)?;
                    instructions.push(Instruction::Mirror { from, to, instructions: inner });
                }
                Statement::Layer { name, statements } => {
                    let inner = Self::evaluate_statements(statements)?;
                    instructions.push(Instruction::Layer { name, instructions: inner });
                }
                Statement::Grid { .. } => {
                    return Err(EvaluateError {
                        message: "grid statement is not allowed inside a block".to_string(),
                    });
                }
            }
        }
        Ok(instructions)
    }

    fn expect_number(value: Value, operator: &Operator) -> Result<i64, EvaluateError> {
        match value {
            Value::Number(number) => Ok(number),
            Value::Boolean(_) => Err(EvaluateError {
                message: format!(
                    "operator '{}' requires number operands",
                    Self::operator_name(operator)
                ),
            }),
        }
    }

    fn expect_boolean(value: Value, operator: &Operator) -> Result<bool, EvaluateError> {
        match value {
            Value::Boolean(boolean) => Ok(boolean),
            Value::Number(_) => Err(EvaluateError {
                message: format!(
                    "operator '{}' requires boolean operands",
                    Self::operator_name(operator)
                ),
            }),
        }
    }

    fn expect_type_number(
        value_type: &ValueType,
        operator: &Operator,
    ) -> Result<(), EvaluateError> {
        if *value_type != ValueType::Number {
            return Err(EvaluateError {
                message: format!(
                    "operator '{}' requires number operands",
                    Self::operator_name(operator)
                ),
            });
        }
        Ok(())
    }

    fn expect_type_boolean(
        value_type: &ValueType,
        operator: &Operator,
    ) -> Result<(), EvaluateError> {
        if *value_type != ValueType::Boolean {
            return Err(EvaluateError {
                message: format!(
                    "operator '{}' requires boolean operands",
                    Self::operator_name(operator)
                ),
            });
        }
        Ok(())
    }

    fn evaluate_arithmetic(
        left: i64,
        operator: &Operator,
        right: i64,
    ) -> Result<Value, EvaluateError> {
        let result = match operator {
            Operator::Add => left.checked_add(right),
            Operator::Subtract => left.checked_sub(right),
            Operator::Multiply => left.checked_mul(right),
            Operator::Divide => {
                if right == 0 {
                    return Err(EvaluateError {
                        message: "division by zero".to_string(),
                    });
                }
                left.checked_div(right)
            }
            Operator::Power => {
                if right < 0 {
                    return Err(EvaluateError {
                        message: "negative exponent".to_string(),
                    });
                }
                u32::try_from(right)
                    .ok()
                    .and_then(|exponent| left.checked_pow(exponent))
            }
            _ => unreachable!(),
        };

        result
            .map(Value::Number)
            .ok_or_else(|| EvaluateError {
                message: "arithmetic overflow".to_string(),
            })
    }

    fn evaluate_comparison(
        left: i64,
        operator: &Operator,
        right: i64,
    ) -> Result<Value, EvaluateError> {
        let result = match operator {
            Operator::Equal => left == right,
            Operator::LessThan => left < right,
            Operator::GreaterThan => left > right,
            Operator::LessThanOrEqual => left <= right,
            Operator::GreaterThanOrEqual => left >= right,
            _ => unreachable!(),
        };
        Ok(Value::Boolean(result))
    }

    fn evaluate_logical(
        left: bool,
        operator: &Operator,
        right: bool,
    ) -> Result<Value, EvaluateError> {
        let result = match operator {
            Operator::And => left && right,
            Operator::Or => left || right,
            _ => unreachable!(),
        };
        Ok(Value::Boolean(result))
    }

    fn operator_name(operator: &Operator) -> &'static str {
        match operator {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
            Operator::Equal => "=",
            Operator::LessThan => "<",
            Operator::GreaterThan => ">",
            Operator::LessThanOrEqual => "<=",
            Operator::GreaterThanOrEqual => ">=",
            Operator::And => "and",
            Operator::Or => "or",
        }
    }
}
