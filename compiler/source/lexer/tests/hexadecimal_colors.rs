use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

#[test]
fn tokenize_hex_color_6_digits() {
    let mut lexer = Lexer::new("#FF0000");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::HexadecimalColor("FF0000".to_string()));
    assert_eq!(tokens[0].text, "#FF0000");
}

#[test]
fn tokenize_hex_color_3_digits() {
    let mut lexer = Lexer::new("#F00");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::HexadecimalColor("F00".to_string()));
}

#[test]
fn tokenize_hex_color_8_digits() {
    let mut lexer = Lexer::new("#FF0000FF");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::HexadecimalColor("FF0000FF".to_string()));
}

#[test]
fn tokenize_hex_color_lowercase() {
    let mut lexer = Lexer::new("#ff00aa");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::HexadecimalColor("ff00aa".to_string()));
}

#[test]
fn tokenize_hex_color_2_digits() {
    let mut lexer = Lexer::new("#FF");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] Hexadecimal color '#FF' must have 3, 6, or 8 digits at line 1, column 1");
}

#[test]
fn tokenize_hex_color_4_digits() {
    let mut lexer = Lexer::new("#FF00");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] Hexadecimal color '#FF00' must have 3, 6, or 8 digits at line 1, column 1");
}

#[test]
fn tokenize_hex_color_7_digits() {
    let mut lexer = Lexer::new("#FF00001");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] Hexadecimal color '#FF00001' must have 3, 6, or 8 digits at line 1, column 1");
}

#[test]
fn tokenize_hex_color_empty() {
    let mut lexer = Lexer::new("#");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] Hexadecimal color '#' must have 3, 6, or 8 digits at line 1, column 1");
}
