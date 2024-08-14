mod evaluate;
mod exprs;
mod parser;
mod scanner;
mod tokens;
mod utils;
use exprs::print_exprs;
use parser::parser::parse_tokens;
use scanner::tokenize::{print_tokens, scanner};
use std::env;
use std::fs;
use std::io::{self, Write};

fn get_file_content(filename: &String) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = get_file_content(filename);
            let tokens = scanner(file_contents);
            print_tokens(&tokens)
        }
        "evaluate" => {
            todo!()
        }
        "parse" => {
            let file_contents = get_file_content(filename);
            let tokens = scanner(file_contents);
            let exprs = parse_tokens(&tokens);
            print_exprs(&exprs)
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
