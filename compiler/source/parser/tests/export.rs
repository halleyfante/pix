use crate::lexer::Lexer;
use crate::parser::ast::{Format, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_export_with_scale() {
    let mut lexer = Lexer::new("export \"art\" in png scale 4");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "art".to_string(),
                format: Format::Png,
                scale: Some(4),
            }],
        }
    );
}

#[test]
fn parse_export_without_scale() {
    let mut lexer = Lexer::new("export \"art-small\" in png");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "art-small".to_string(),
                format: Format::Png,
                scale: None,
            }],
        }
    );
}

#[test]
fn parse_export_svg() {
    let mut lexer = Lexer::new("export \"art\" in svg");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "art".to_string(),
                format: Format::Svg,
                scale: None,
            }],
        }
    );
}

#[test]
fn parse_export_svg_with_scale() {
    let mut lexer = Lexer::new("export \"art\" in svg scale 2");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "art".to_string(),
                format: Format::Svg,
                scale: Some(2),
            }],
        }
    );
}

#[test]
fn parse_export_webp() {
    let mut lexer = Lexer::new("export \"art\" in webp");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "art".to_string(),
                format: Format::Webp,
                scale: None,
            }],
        }
    );
}

#[test]
fn parse_export_webp_with_scale() {
    let mut lexer = Lexer::new("export \"art\" in webp scale 8");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "art".to_string(),
                format: Format::Webp,
                scale: Some(8),
            }],
        }
    );
}

#[test]
fn parse_export_gif() {
    let mut lexer = Lexer::new("export \"animation\" in gif");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Export {
                filename: "animation".to_string(),
                format: Format::Gif,
                scale: None,
            }],
        }
    );
}

#[test]
fn parse_export_missing_filename() {
    let mut lexer = Lexer::new("export in png");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected string, found In");
}

#[test]
fn parse_export_missing_format() {
    let mut lexer = Lexer::new("export \"art\" in");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "unexpected end of input");
}
