use crate::lexer::Lexer;

#[test]
fn tokenize_unrecognized_character() {
    let mut lexer = Lexer::new("@");
    let error = lexer.tokenize().unwrap_err();
    assert_eq!(error, "[ERROR] Unexpected character '@' at line 1, column 1");
}
