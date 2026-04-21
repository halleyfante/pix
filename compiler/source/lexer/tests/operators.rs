use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_plus() {
    let mut lexer = Lexer::new("+");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Plus);
}

#[test]
fn tokenize_minus() {
    let mut lexer = Lexer::new("-");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Minus);
}

#[test]
fn tokenize_asterisk() {
    let mut lexer = Lexer::new("*");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Asterisk);
}

#[test]
fn tokenize_slash() {
    let mut lexer = Lexer::new("/");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Slash);
}

#[test]
fn tokenize_caret() {
    let mut lexer = Lexer::new("^");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Caret);
}

#[test]
fn tokenize_equal() {
    let mut lexer = Lexer::new("=");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Equal);
}

#[test]
fn tokenize_less_than() {
    let mut lexer = Lexer::new("<");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LessThan);
}

#[test]
fn tokenize_greater_than() {
    let mut lexer = Lexer::new(">");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::GreaterThan);
}

#[test]
fn tokenize_less_than_or_equal() {
    let mut lexer = Lexer::new("<=");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LessThanOrEqual);
    assert_eq!(tokens[0].text, "<=");
}

#[test]
fn tokenize_greater_than_or_equal() {
    let mut lexer = Lexer::new(">=");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::GreaterThanOrEqual);
    assert_eq!(tokens[0].text, ">=");
}

#[test]
fn tokenize_expression() {
    let mut lexer = Lexer::new("(+ - *)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
    assert_eq!(tokens[1].kind, TokenKind::Plus);
    assert_eq!(tokens[2].kind, TokenKind::Minus);
    assert_eq!(tokens[3].kind, TokenKind::Asterisk);
    assert_eq!(tokens[4].kind, TokenKind::RightParenthesis);
}
