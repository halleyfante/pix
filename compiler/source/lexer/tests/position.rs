use crate::lexer::Lexer;

#[test]
fn tokenize_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn tokenize_tracks_position() {
    let mut lexer = Lexer::new("( )");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].position.line, 1);
    assert_eq!(tokens[0].position.column, 1);
    assert_eq!(tokens[1].position.line, 1);
    assert_eq!(tokens[1].position.column, 3);
}

#[test]
fn tokenize_tracks_line_position() {
    let mut lexer = Lexer::new("(\n)");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].position.line, 1);
    assert_eq!(tokens[1].position.line, 2);
}
