use crate::lexer::Lexer;
use crate::parser::ast::{ColorEntry, ColorValue, Expression, Operator, Program, Statement};
use crate::parser::parse::Parser;

#[test]
fn parse_color_block() {
    let source = "color {\n    skin: #FFCB96\n    eyes: #000000\n}";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::ColorBlock {
                entries: vec![
                    ColorEntry {
                        name: "skin".to_string(),
                        color: "FFCB96".to_string(),
                    },
                    ColorEntry {
                        name: "eyes".to_string(),
                        color: "000000".to_string(),
                    },
                ],
            }],
        }
    );
}

#[test]
fn parse_empty_color_block() {
    let mut lexer = Lexer::new("color { }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::ColorBlock { entries: vec![] }],
        }
    );
}

#[test]
fn parse_draw_with_named_color() {
    let mut lexer = Lexer::new("draw x = 3 and y = 5 with color skin");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![Statement::Draw {
                condition: Expression::BinaryOperation {
                    left: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::CoordinateX),
                        operator: Operator::Equal,
                        right: Box::new(Expression::Number(3)),
                    }),
                    operator: Operator::And,
                    right: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::CoordinateY),
                        operator: Operator::Equal,
                        right: Box::new(Expression::Number(5)),
                    }),
                },
                color: ColorValue::Named("skin".to_string()),
            }],
        }
    );
}

#[test]
fn parse_color_block_missing_brace() {
    let mut lexer = Lexer::new("color skin: #FFCB96");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected LeftBrace, found Identifier(\"skin\")");
}

#[test]
fn parse_color_block_missing_colon() {
    let mut lexer = Lexer::new("color { skin #FFCB96 }");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let error = parser.parse().unwrap_err();

    assert_eq!(error.message, "expected Colon, found HexadecimalColor(\"FFCB96\")");
}
