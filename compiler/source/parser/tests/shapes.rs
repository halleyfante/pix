use crate::lexer::Lexer;
use crate::parser::ast::{ColorValue, Point, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_pixel() {
    let mut lexer = Lexer::new("pixel (3, 5) with #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Pixel {
                point: Point { x: 3, y: 5 },
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            }],
        }
    );
}

#[test]
fn parse_line() {
    let mut lexer = Lexer::new("line (0, 0) to (8, 8) with #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Line {
                from: Point { x: 0, y: 0 },
                to: Point { x: 8, y: 8 },
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            }],
        }
    );
}

#[test]
fn parse_rectangle() {
    let mut lexer = Lexer::new("rectangle (0, 0) to (4, 4) with #0000FF");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Rectangle {
                from: Point { x: 0, y: 0 },
                to: Point { x: 4, y: 4 },
                color: ColorValue::Hexadecimal("0000FF".to_string()),
            }],
        }
    );
}

#[test]
fn parse_triangle() {
    let mut lexer = Lexer::new("triangle (0, 0) to (8, 0) to (4, 8) with #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Triangle {
                first: Point { x: 0, y: 0 },
                second: Point { x: 8, y: 0 },
                third: Point { x: 4, y: 8 },
                color: ColorValue::Hexadecimal("FF0000".to_string()),
            }],
        }
    );
}

#[test]
fn parse_circle() {
    let mut lexer = Lexer::new("circle (8, 8) radius 4 with #00FF00");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Circle {
                center: Point { x: 8, y: 8 },
                radius: 4,
                color: ColorValue::Hexadecimal("00FF00".to_string()),
            }],
        }
    );
}

#[test]
fn parse_pixel_missing_point() {
    let mut lexer = Lexer::new("pixel with #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected LeftParenthesis, found With");
}

#[test]
fn parse_circle_missing_radius() {
    let mut lexer = Lexer::new("circle (8, 8) with #00FF00");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected Radius, found With");
}

#[test]
fn parse_line_missing_to() {
    let mut lexer = Lexer::new("line (0, 0) (8, 8) with #FF0000");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected To, found LeftParenthesis");
}
