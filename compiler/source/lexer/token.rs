/// Represents a position in the source code.
#[derive(Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Represents the type of a token.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(u32),
    HexadecimalColor(String),
    StringLiteral(String),
    Identifier(String),

    // Delimiters
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Colon,
    Comma,

    // Arithmetic operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,

    // Logical keywords
    And,
    Or,
    Not,

    // Keywords
    With,
    In,
    By,
    To,
    Radius,
    Scale,
    Color,
    X,
    Y,

    // Statements
    Draw,
    Erase,
    Clear,
    Grid,
    Export,

    // Shape statements
    Pixel,
    Line,
    Rectangle,
    Triangle,
    Circle,

    // Format keywords
    Png,
    Svg,
    Webp,
    Gif,

    // Block keywords
    Layer,
    Mirror,
    Frame,

    // Movement keywords
    Copy,
    Move,
    At,

    // Comparison operators
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

/// Represents a token produced by the lexer.
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub position: Position,
}

impl Token {
    pub fn new(kind: TokenKind, text: &str, position: Position) -> Self {
        Self {
            kind,
            text: text.to_string(),
            position,
        }
    }
}
