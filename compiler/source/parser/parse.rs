use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::{ColorEntry, ColorValue, Expression, Format, Operator, Point, Program, Statement};

/// Represents an error encountered during parsing.
#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[ERROR] {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

/// Transforms a sequence of tokens into an AST.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while self.position < self.tokens.len() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token() {
            Some(token) => match token.kind {
                TokenKind::Grid => self.parse_grid(),
                TokenKind::Draw => self.parse_draw(),
                TokenKind::Erase => self.parse_erase(),
                TokenKind::Clear => {
                    self.advance();
                    Ok(Statement::Clear)
                }
                TokenKind::Pixel => self.parse_pixel(),
                TokenKind::Line => self.parse_line(),
                TokenKind::Rectangle => self.parse_rectangle(),
                TokenKind::Triangle => self.parse_triangle(),
                TokenKind::Circle => self.parse_circle(),
                TokenKind::Export => self.parse_export(),
                TokenKind::Color => self.parse_color_block(),
                TokenKind::Frame => self.parse_frame(),
                TokenKind::Copy => self.parse_copy(),
                TokenKind::Move => self.parse_move(),
                TokenKind::Layer => self.parse_layer(),
                TokenKind::Mirror => self.parse_mirror(),
                _ => Err(ParseError {
                    message: format!("unexpected token {:?}", token.kind),
                    line: token.position.line,
                    column: token.position.column,
                }),
            },
            None => Err(self.unexpected_end_of_input()),
        }
    }

    fn parse_grid(&mut self) -> Result<Statement, ParseError> {
        self.advance();

        let width = self.expect_number()?;
        self.expect_token(TokenKind::By)?;
        let height = self.expect_number()?;

        Ok(Statement::Grid { width, height })
    }

    fn expect_number(&mut self) -> Result<u32, ParseError> {
        match self.current_token() {
            Some(token) => match token.kind {
                TokenKind::Number(value) => {
                    self.advance();
                    Ok(value)
                }
                _ => Err(ParseError {
                    message: format!("expected number, found {:?}", token.kind),
                    line: token.position.line,
                    column: token.position.column,
                }),
            },
            None => Err(self.unexpected_end_of_input()),
        }
    }

    fn expect_token(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        match self.current_token() {
            Some(token) => {
                if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
                    self.advance();
                    Ok(())
                } else {
                    Err(ParseError {
                        message: format!("expected {:?}, found {:?}", expected, token.kind),
                        line: token.position.line,
                        column: token.position.column,
                    })
                }
            }
            None => Err(self.unexpected_end_of_input()),
        }
    }

    fn parse_color_block(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        self.expect_token(TokenKind::LeftBrace)?;

        let mut entries = Vec::new();

        while !matches!(self.current_token(), Some(token) if token.kind == TokenKind::RightBrace) {
            let name = match self.current_token() {
                Some(token) => match &token.kind {
                    TokenKind::Identifier(value) => {
                        let name = value.clone();
                        self.advance();
                        name
                    }
                    _ => {
                        return Err(ParseError {
                            message: format!("expected identifier, found {:?}", token.kind),
                            line: token.position.line,
                            column: token.position.column,
                        })
                    }
                },
                None => return Err(self.unexpected_end_of_input()),
            };

            self.expect_token(TokenKind::Colon)?;

            let color = match self.current_token() {
                Some(token) => match &token.kind {
                    TokenKind::HexadecimalColor(hex) => {
                        let color = hex.clone();
                        self.advance();
                        color
                    }
                    _ => {
                        return Err(ParseError {
                            message: format!("expected hex color, found {:?}", token.kind),
                            line: token.position.line,
                            column: token.position.column,
                        })
                    }
                },
                None => return Err(self.unexpected_end_of_input()),
            };

            entries.push(ColorEntry { name, color });
        }

        self.expect_token(TokenKind::RightBrace)?;

        Ok(Statement::ColorBlock { entries })
    }

    fn parse_export(&mut self) -> Result<Statement, ParseError> {
        self.advance();

        let filename = match self.current_token() {
            Some(token) => match &token.kind {
                TokenKind::StringLiteral(value) => {
                    let filename = value.clone();
                    self.advance();
                    filename
                }
                _ => {
                    return Err(ParseError {
                        message: format!("expected string, found {:?}", token.kind),
                        line: token.position.line,
                        column: token.position.column,
                    })
                }
            },
            None => return Err(self.unexpected_end_of_input()),
        };

        self.expect_token(TokenKind::In)?;

        let format = match self.current_token() {
            Some(token) => match token.kind {
                TokenKind::Png => {
                    self.advance();
                    Format::Png
                }
                TokenKind::Svg => {
                    self.advance();
                    Format::Svg
                }
                TokenKind::Webp => {
                    self.advance();
                    Format::Webp
                }
                TokenKind::Gif => {
                    self.advance();
                    Format::Gif
                }
                _ => {
                    return Err(ParseError {
                        message: format!("expected format, found {:?}", token.kind),
                        line: token.position.line,
                        column: token.position.column,
                    })
                }
            },
            None => return Err(self.unexpected_end_of_input()),
        };

        let scale = if matches!(self.current_token(), Some(token) if token.kind == TokenKind::Scale) {
            self.advance();
            Some(self.expect_number()?)
        } else {
            None
        };

        Ok(Statement::Export { filename, format, scale })
    }

    fn parse_pixel(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let point = self.parse_point()?;
        self.expect_token(TokenKind::With)?;
        let color = self.parse_color_value()?;
        Ok(Statement::Pixel { point, color })
    }

    fn parse_line(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let from = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let to = self.parse_point()?;
        self.expect_token(TokenKind::With)?;
        let color = self.parse_color_value()?;
        Ok(Statement::Line { from, to, color })
    }

    fn parse_rectangle(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let from = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let to = self.parse_point()?;
        self.expect_token(TokenKind::With)?;
        let color = self.parse_color_value()?;
        Ok(Statement::Rectangle { from, to, color })
    }

    fn parse_triangle(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let first = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let second = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let third = self.parse_point()?;
        self.expect_token(TokenKind::With)?;
        let color = self.parse_color_value()?;
        Ok(Statement::Triangle { first, second, third, color })
    }

    fn parse_circle(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let center = self.parse_point()?;
        self.expect_token(TokenKind::Radius)?;
        let radius = self.expect_number()?;
        self.expect_token(TokenKind::With)?;
        let color = self.parse_color_value()?;
        Ok(Statement::Circle { center, radius, color })
    }

    fn parse_point(&mut self) -> Result<Point, ParseError> {
        self.expect_token(TokenKind::LeftParenthesis)?;
        let x = self.expect_number()?;
        self.expect_token(TokenKind::Comma)?;
        let y = self.expect_number()?;
        self.expect_token(TokenKind::RightParenthesis)?;
        Ok(Point { x, y })
    }

    fn parse_erase(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let condition = self.parse_expression()?;
        Ok(Statement::Erase { condition })
    }

    fn parse_draw(&mut self) -> Result<Statement, ParseError> {
        self.advance();

        let condition = self.parse_expression()?;
        self.expect_token(TokenKind::With)?;
        let color = self.parse_color_value()?;

        Ok(Statement::Draw { condition, color })
    }

    fn parse_color_value(&mut self) -> Result<ColorValue, ParseError> {
        match self.current_token() {
            Some(token) => match &token.kind {
                TokenKind::HexadecimalColor(hex) => {
                    let color = ColorValue::Hexadecimal(hex.clone());
                    self.advance();
                    Ok(color)
                }
                TokenKind::Color => {
                    self.advance();
                    match self.current_token() {
                        Some(token) => match &token.kind {
                            TokenKind::Identifier(name) => {
                                let name = name.clone();
                                self.advance();
                                Ok(ColorValue::Named(name))
                            }
                            _ => Err(ParseError {
                                message: format!("expected color name, found {:?}", token.kind),
                                line: token.position.line,
                                column: token.position.column,
                            }),
                        },
                        None => Err(self.unexpected_end_of_input()),
                    }
                }
                _ => Err(ParseError {
                    message: format!("expected color, found {:?}", token.kind),
                    line: token.position.line,
                    column: token.position.column,
                }),
            },
            None => Err(self.unexpected_end_of_input()),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_and()?;

        while matches!(self.current_token(), Some(token) if token.kind == TokenKind::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOperation {
                left: Box::new(left),
                operator: Operator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_not()?;

        while matches!(self.current_token(), Some(token) if token.kind == TokenKind::And) {
            self.advance();
            let right = self.parse_not()?;
            left = Expression::BinaryOperation {
                left: Box::new(left),
                operator: Operator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expression, ParseError> {
        if matches!(self.current_token(), Some(token) if token.kind == TokenKind::Not) {
            self.advance();
            let operand = self.parse_not()?;
            return Ok(Expression::UnaryNot {
                operand: Box::new(operand),
            });
        }

        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_addition()?;

        let operator = match self.current_token() {
            Some(token) if token.kind == TokenKind::Equal => Some(Operator::Equal),
            Some(token) if token.kind == TokenKind::LessThan => Some(Operator::LessThan),
            Some(token) if token.kind == TokenKind::GreaterThan => Some(Operator::GreaterThan),
            Some(token) if token.kind == TokenKind::LessThanOrEqual => Some(Operator::LessThanOrEqual),
            Some(token) if token.kind == TokenKind::GreaterThanOrEqual => Some(Operator::GreaterThanOrEqual),
            _ => None,
        };

        if let Some(operator) = operator {
            self.advance();
            let right = self.parse_addition()?;
            return Ok(Expression::BinaryOperation {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplication()?;

        loop {
            let operator = match self.current_token() {
                Some(token) if token.kind == TokenKind::Plus => Some(Operator::Add),
                Some(token) if token.kind == TokenKind::Minus => Some(Operator::Subtract),
                _ => None,
            };

            match operator {
                Some(operator) => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expression::BinaryOperation {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }
                None => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_exponentiation()?;

        loop {
            let operator = match self.current_token() {
                Some(token) if token.kind == TokenKind::Asterisk => Some(Operator::Multiply),
                Some(token) if token.kind == TokenKind::Slash => Some(Operator::Divide),
                _ => None,
            };

            match operator {
                Some(operator) => {
                    self.advance();
                    let right = self.parse_exponentiation()?;
                    left = Expression::BinaryOperation {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }
                None => break,
            }
        }

        Ok(left)
    }

    fn parse_exponentiation(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_primary()?;

        if matches!(self.current_token(), Some(token) if token.kind == TokenKind::Caret) {
            self.advance();
            let right = self.parse_exponentiation()?;
            return Ok(Expression::BinaryOperation {
                left: Box::new(left),
                operator: Operator::Power,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(token) => match token.kind {
                TokenKind::Number(value) => {
                    self.advance();
                    Ok(Expression::Number(value))
                }
                TokenKind::X => {
                    self.advance();
                    Ok(Expression::CoordinateX)
                }
                TokenKind::Y => {
                    self.advance();
                    Ok(Expression::CoordinateY)
                }
                TokenKind::LeftParenthesis => {
                    self.advance();
                    let expression = self.parse_expression()?;
                    self.expect_token(TokenKind::RightParenthesis)?;
                    Ok(expression)
                }
                _ => Err(ParseError {
                    message: format!("expected expression, found {:?}", token.kind),
                    line: token.position.line,
                    column: token.position.column,
                }),
            },
            None => Err(self.unexpected_end_of_input()),
        }
    }

    fn parse_frame(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let delay = self.expect_number()?;
        Ok(Statement::Frame { delay })
    }

    fn parse_copy(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let from = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let to = self.parse_point()?;
        self.expect_token(TokenKind::At)?;
        let destination = self.parse_point()?;
        Ok(Statement::Copy { from, to, destination })
    }

    fn parse_move(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let from = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let to = self.parse_point()?;
        self.expect_token(TokenKind::At)?;
        let destination = self.parse_point()?;
        Ok(Statement::Move { from, to, destination })
    }

    fn parse_layer(&mut self) -> Result<Statement, ParseError> {
        self.advance();

        let name = match self.current_token() {
            Some(token) => match &token.kind {
                TokenKind::Identifier(value) => {
                    let name = value.clone();
                    self.advance();
                    name
                }
                _ => {
                    return Err(ParseError {
                        message: format!("expected layer name, found {:?}", token.kind),
                        line: token.position.line,
                        column: token.position.column,
                    })
                }
            },
            None => return Err(self.unexpected_end_of_input()),
        };

        self.expect_token(TokenKind::LeftBrace)?;

        let mut statements = Vec::new();
        while !matches!(self.current_token(), Some(token) if token.kind == TokenKind::RightBrace) {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.expect_token(TokenKind::RightBrace)?;

        Ok(Statement::Layer { name, statements })
    }

    fn parse_mirror(&mut self) -> Result<Statement, ParseError> {
        self.advance();
        let from = self.parse_point()?;
        self.expect_token(TokenKind::To)?;
        let to = self.parse_point()?;
        self.expect_token(TokenKind::LeftBrace)?;

        let mut statements = Vec::new();
        while !matches!(self.current_token(), Some(token) if token.kind == TokenKind::RightBrace) {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.expect_token(TokenKind::RightBrace)?;

        Ok(Statement::Mirror { from, to, statements })
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn unexpected_end_of_input(&self) -> ParseError {
        ParseError {
            message: "unexpected end of input".to_string(),
            line: 0,
            column: 0,
        }
    }
}
