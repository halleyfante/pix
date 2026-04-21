use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_comment_is_ignored() {
    let mut lexer = Lexer::new("// this is a comment");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn tokenize_comment_before_code() {
    let mut lexer = Lexer::new("// comment\n(");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
}

#[test]
fn tokenize_code_before_comment() {
    let mut lexer = Lexer::new("( // comment");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LeftParenthesis);
}

#[test]
fn tokenize_slash_is_not_comment() {
    let mut lexer = Lexer::new("/");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Slash);
}
