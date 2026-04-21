use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_single_digit() {
    let mut lexer = Lexer::new("0");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Number(0));
    assert_eq!(tokens[0].text, "0");
}

#[test]
fn tokenize_multi_digit() {
    let mut lexer = Lexer::new("255");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Number(255));
    assert_eq!(tokens[0].text, "255");
}

#[test]
fn tokenize_number_in_parentheses() {
    let mut lexer = Lexer::new("(16)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
    assert_eq!(tokens[1].kind, TokenKind::Number(16));
    assert_eq!(tokens[2].kind, TokenKind::RightParenthesis);
}

#[test]
fn tokenize_numbers_with_operators() {
    let mut lexer = Lexer::new("3 + 5");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::Number(3));
    assert_eq!(tokens[1].kind, TokenKind::Plus);
    assert_eq!(tokens[2].kind, TokenKind::Number(5));
}

#[test]
fn tokenize_number_too_large() {
    let mut lexer = Lexer::new("4294967296");
    let result = lexer.tokenize();
    assert!(result.is_err());
}
