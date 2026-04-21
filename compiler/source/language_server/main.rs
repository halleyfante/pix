use std::env;
use std::io::{self, Read};
use std::process;

use pix::language_server::{completion, server};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() > 1 && arguments[1] == "complete" {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap_or_else(|error| {
            eprintln!("error: failed to read stdin: {}", error);
            process::exit(1);
        });

        let request: serde_json::Value = serde_json::from_str(&input).unwrap_or_else(|error| {
            eprintln!("error: invalid JSON: {}", error);
            process::exit(1);
        });

        let source = request["source"].as_str().unwrap_or("");
        let line = request["line"].as_u64().unwrap_or(1) as usize;
        let column = request["column"].as_u64().unwrap_or(1) as usize;

        let items = completion::complete(source, line, column);
        let output = serde_json::to_string(&items).unwrap();
        println!("{}", output);
    } else {
        server::run();
    }
}
