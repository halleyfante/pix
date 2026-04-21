use crate::parser::ast::{ColorEntry, ColorValue, Expression, Format, Point};

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(i64),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    Number,
    Boolean,
}

#[derive(Debug, PartialEq)]
pub struct EvaluateError {
    pub message: String,
}

/// The validated output of the evaluator, with grid dimensions extracted and all statements converted to instructions.
#[derive(Debug, PartialEq)]
pub struct EvaluatedProgram {
    pub grid_width: u32,
    pub grid_height: u32,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Draw {
        condition: Expression,
        color: ColorValue,
    },
    Erase {
        condition: Expression,
    },
    Clear,
    Pixel {
        condition: Expression,
        point: Point,
        color: ColorValue,
    },
    Line {
        condition: Expression,
        from: Point,
        to: Point,
        color: ColorValue,
    },
    Rectangle {
        condition: Expression,
        from: Point,
        to: Point,
        color: ColorValue,
    },
    Triangle {
        condition: Expression,
        first: Point,
        second: Point,
        third: Point,
        color: ColorValue,
    },
    Circle {
        condition: Expression,
        center: Point,
        radius: u32,
        color: ColorValue,
    },
    Export {
        filename: String,
        format: Format,
        scale: Option<u32>,
    },
    ColorBlock {
        entries: Vec<ColorEntry>,
    },
    Frame {
        delay: u32,
    },
    Copy {
        from: Point,
        to: Point,
        destination: Point,
    },
    Move {
        from: Point,
        to: Point,
        destination: Point,
    },
    Layer {
        name: String,
        instructions: Vec<Instruction>,
    },
    Mirror {
        from: Point,
        to: Point,
        instructions: Vec<Instruction>,
    },
}
