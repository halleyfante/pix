use std::env;
use std::fs;
use std::process;

use pix::evaluator::evaluate::Evaluator;
use pix::exporter::export::Exporter;
use pix::lexer::Lexer;
use pix::parser::parse::Parser;
use pix::renderer::render::Renderer;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        eprintln!("usage: pix <file.pix>");
        process::exit(1);
    }

    let filename = &arguments[1];
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("error: could not read '{}': {}", filename, error);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let evaluated = match Evaluator::evaluate(program) {
        Ok(evaluated) => evaluated,
        Err(error) => {
            eprintln!("error: {}", error.message);
            process::exit(1);
        }
    };

    let result = match Renderer::render(&evaluated) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("error: {}", error.message);
            process::exit(1);
        }
    };

    if result.has_frames && !result.has_animated_export {
        eprintln!("warning: frame statements have no effect without an animated export (gif)");
    }

    if let Err(error) = Exporter::export(&result, &evaluated.instructions) {
        eprintln!("error: {}", error.message);
        process::exit(1);
    }
}
