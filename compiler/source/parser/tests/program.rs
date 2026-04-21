use crate::parser::ast::Program;
use crate::parser::parse::Parser;

#[test]
fn parse_empty_program() {
    let program = Parser::new(vec![]).parse().unwrap();

    assert_eq!(
        program,
        Program {
            statements: vec![],
        }
    );
}
