use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_left_parenthesis() {
    let mut lexer = Lexer::new("(");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
    assert_eq!(tokens[0].text, "(");
}

#[test]
fn tokenize_right_parenthesis() {
    let mut lexer = Lexer::new(")");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::RightParenthesis);
    assert_eq!(tokens[0].text, ")");
}

#[test]
fn tokenize_left_brace() {
    let mut lexer = Lexer::new("{");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[0].text, "{");
}

#[test]
fn tokenize_right_brace() {
    let mut lexer = Lexer::new("}");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::RightBrace);
    assert_eq!(tokens[0].text, "}");
}

#[test]
fn tokenize_colon() {
    let mut lexer = Lexer::new(":");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Colon);
    assert_eq!(tokens[0].text, ":");
}

#[test]
fn tokenize_comma() {
    let mut lexer = Lexer::new(",");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Comma);
    assert_eq!(tokens[0].text, ",");
}

#[test]
fn tokenize_multiple_delimiters() {
    let mut lexer = Lexer::new("({})");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
    assert_eq!(tokens[1].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[2].kind, TokenKind::RightBrace);
    assert_eq!(tokens[3].kind, TokenKind::RightParenthesis);
}

#[test]
fn tokenize_delimiters_with_whitespace() {
    let mut lexer = Lexer::new("( { } )");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
    assert_eq!(tokens[1].kind, TokenKind::LeftBrace);
    assert_eq!(tokens[2].kind, TokenKind::RightBrace);
    assert_eq!(tokens[3].kind, TokenKind::RightParenthesis);
}
