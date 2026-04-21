use crate::lexer::token::{Token, TokenKind, Position};

/// Reads source code and produces a sequence of tokens.
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            let character = self.source[self.position];

            if character.is_whitespace() {
                self.advance();
                continue;
            }

            let token = match character {
                '(' => self.single_character_token(TokenKind::LeftParenthesis),
                ')' => self.single_character_token(TokenKind::RightParenthesis),
                '{' => self.single_character_token(TokenKind::LeftBrace),
                '}' => self.single_character_token(TokenKind::RightBrace),
                ':' => self.single_character_token(TokenKind::Colon),
                ',' => self.single_character_token(TokenKind::Comma),
                '+' => self.single_character_token(TokenKind::Plus),
                '-' => self.single_character_token(TokenKind::Minus),
                '*' => self.single_character_token(TokenKind::Asterisk),
                '/' => {
                    if self.peek() == Some('/') {
                        self.skip_comment();
                        continue;
                    }
                    self.single_character_token(TokenKind::Slash)
                },
                '^' => self.single_character_token(TokenKind::Caret),
                '=' => self.single_character_token(TokenKind::Equal),
                '<' => {
                    if self.peek() == Some('=') {
                        let position = Position { line: self.line, column: self.column };
                        self.advance();
                        self.advance();
                        Token::new(TokenKind::LessThanOrEqual, "<=", position)
                    } else {
                        self.single_character_token(TokenKind::LessThan)
                    }
                },
                '>' => {
                    if self.peek() == Some('=') {
                        let position = Position { line: self.line, column: self.column };
                        self.advance();
                        self.advance();
                        Token::new(TokenKind::GreaterThanOrEqual, ">=", position)
                    } else {
                        self.single_character_token(TokenKind::GreaterThan)
                    }
                },
                '0'..='9' => self.number_token()?,
                '#' => self.hexadecimal_color_token()?,
                '"' => self.string_literal_token()?,
                'a'..='z' | 'A'..='Z' => self.keyword_token()?,
                _ => return Err(format!(
                    "[ERROR] Unexpected character '{}' at line {}, column {}",
                    character, self.line, self.column
                )),
            };

            tokens.push(token);
        }

        Ok(tokens)
    }

    fn single_character_token(&mut self, kind: TokenKind) -> Token {
        let character = self.source[self.position];
        let position = Position {
            line: self.line,
            column: self.column,
        };
        self.advance();
        Token::new(kind, &character.to_string(), position)
    }

    fn keyword_token(&mut self) -> Result<Token, String> {
        let position = Position {
            line: self.line,
            column: self.column,
        };
        let mut text = String::new();

        while self.position < self.source.len() && self.source[self.position].is_ascii_alphabetic() {
            text.push(self.source[self.position]);
            self.advance();
        }

        let kind = match text.as_str() {
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "with" => TokenKind::With,
            "in" => TokenKind::In,
            "by" => TokenKind::By,
            "to" => TokenKind::To,
            "radius" => TokenKind::Radius,
            "scale" => TokenKind::Scale,
            "color" => TokenKind::Color,
            "x" => TokenKind::X,
            "y" => TokenKind::Y,
            "draw" => TokenKind::Draw,
            "erase" => TokenKind::Erase,
            "clear" => TokenKind::Clear,
            "grid" => TokenKind::Grid,
            "export" => TokenKind::Export,
            "pixel" => TokenKind::Pixel,
            "line" => TokenKind::Line,
            "rectangle" => TokenKind::Rectangle,
            "triangle" => TokenKind::Triangle,
            "circle" => TokenKind::Circle,
            "png" => TokenKind::Png,
            "svg" => TokenKind::Svg,
            "webp" => TokenKind::Webp,
            "gif" => TokenKind::Gif,
            "layer" => TokenKind::Layer,
            "mirror" => TokenKind::Mirror,
            "frame" => TokenKind::Frame,
            "copy" => TokenKind::Copy,
            "move" => TokenKind::Move,
            "at" => TokenKind::At,
            _ => TokenKind::Identifier(text.clone()),
        };

        Ok(Token::new(kind, &text, position))
    }

    fn string_literal_token(&mut self) -> Result<Token, String> {
        let position = Position {
            line: self.line,
            column: self.column,
        };
        self.advance(); // skip opening "

        let mut content = String::new();

        while self.position < self.source.len() && self.source[self.position] != '"' {
            if self.source[self.position] == '\n' {
                return Err(format!(
                    "[ERROR] String is missing a closing quote at line {}, column {}",
                    position.line, position.column
                ));
            }
            content.push(self.source[self.position]);
            self.advance();
        }

        if self.position >= self.source.len() {
            return Err(format!(
                "[ERROR] String is missing a closing quote at line {}, column {}",
                position.line, position.column
            ));
        }

        self.advance(); // skip closing "

        let text = format!("\"{}\"", content);
        Ok(Token::new(TokenKind::StringLiteral(content), &text, position))
    }

    fn hexadecimal_color_token(&mut self) -> Result<Token, String> {
        let position = Position {
            line: self.line,
            column: self.column,
        };
        self.advance(); // skip #
        let mut hex = String::new();

        while self.position < self.source.len() && self.source[self.position].is_ascii_hexdigit() {
            hex.push(self.source[self.position]);
            self.advance();
        }

        if hex.len() != 3 && hex.len() != 6 && hex.len() != 8 {
            return Err(format!(
                "[ERROR] Hexadecimal color '#{}' must have 3, 6, or 8 digits at line {}, column {}",
                hex, position.line, position.column
            ));
        }

        let text = format!("#{}", hex);
        Ok(Token::new(TokenKind::HexadecimalColor(hex), &text, position))
    }

    fn number_token(&mut self) -> Result<Token, String> {
        let position = Position {
            line: self.line,
            column: self.column,
        };
        let mut text = String::new();

        while self.position < self.source.len() && self.source[self.position].is_ascii_digit() {
            text.push(self.source[self.position]);
            self.advance();
        }

        let value = text.parse::<u32>().map_err(|_| {
            format!(
                "[ERROR] Number '{}' is too large at line {}, column {}",
                text, position.line, position.column
            )
        })?;
        Ok(Token::new(TokenKind::Number(value), &text, position))
    }

    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.source.len() {
            Some(self.source[self.position + 1])
        } else {
            None
        }
    }

    fn skip_comment(&mut self) {
        while self.position < self.source.len() && self.source[self.position] != '\n' {
            self.advance();
        }
    }

    fn advance(&mut self) {
        if self.position < self.source.len() && self.source[self.position] == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }
}
