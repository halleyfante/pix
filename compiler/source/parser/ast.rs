/// Represents a complete Pix program as a sequence of statements.
#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Represents a color value.
#[derive(Debug, PartialEq)]
pub enum ColorValue {
    Hexadecimal(String),
    Named(String),
}

/// Represents a point on the grid.
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

/// Represents a statement in the language.
#[derive(Debug, PartialEq)]
pub enum Statement {
    Grid { width: u32, height: u32 },
    Draw { condition: Expression, color: ColorValue },
    Erase { condition: Expression },
    Clear,
    Pixel { point: Point, color: ColorValue },
    Line { from: Point, to: Point, color: ColorValue },
    Rectangle { from: Point, to: Point, color: ColorValue },
    Triangle { first: Point, second: Point, third: Point, color: ColorValue },
    Circle { center: Point, radius: u32, color: ColorValue },
    Export { filename: String, format: Format, scale: Option<u32> },
    ColorBlock { entries: Vec<ColorEntry> },
    Frame { delay: u32 },
    Copy { from: Point, to: Point, destination: Point },
    Move { from: Point, to: Point, destination: Point },
    Layer { name: String, statements: Vec<Statement> },
    Mirror { from: Point, to: Point, statements: Vec<Statement> },
}

/// Represents a named color entry in a color block.
#[derive(Debug, PartialEq)]
pub struct ColorEntry {
    pub name: String,
    pub color: String,
}

/// Represents an image format for export.
#[derive(Debug, PartialEq)]
pub enum Format {
    Png,
    Svg,
    Webp,
    Gif,
}

/// Represents a binary operator.
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

/// Represents an expression in the language.
#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(u32),
    CoordinateX,
    CoordinateY,
    BinaryOperation {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    UnaryNot {
        operand: Box<Expression>,
    },
}
