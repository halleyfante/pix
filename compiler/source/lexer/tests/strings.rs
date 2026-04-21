use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_string() {
    let mut lexer = Lexer::new("\"art\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::StringLiteral("art".to_string()));
    assert_eq!(tokens[0].text, "\"art\"");
}

#[test]
fn tokenize_string_with_spaces() {
    let mut lexer = Lexer::new("\"my art\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::StringLiteral("my art".to_string()));
}

#[test]
fn tokenize_empty_string() {
    let mut lexer = Lexer::new("\"\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::StringLiteral("".to_string()));
}

#[test]
fn tokenize_unterminated_string() {
    let mut lexer = Lexer::new("\"art");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] String is missing a closing quote at line 1, column 1");
}

#[test]
fn tokenize_string_with_newline() {
    let mut lexer = Lexer::new("\"art\n\"");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] String is missing a closing quote at line 1, column 1");
}
