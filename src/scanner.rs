use std::io::{self, Write};
use std::process;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    EOF,
    Unknown(String),
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

fn tokenize(line: &str, line_number: usize) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    for char in line.chars() {
        let token_type = match char {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '*' => TokenType::Star,
            '.' => TokenType::Dot,
            ',' => TokenType::Comma,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '-' => TokenType::Minus,
            _ => TokenType::Unknown(char.into()),
        };
        tokens.push(Token {
            lexeme: match token_type {
                TokenType::Unknown(_) => String::new(),
                _ => char.into(),
            },
            token_type,
            line: line_number,
        });
    }
    tokens
}

fn print_tokens(tokens: &Vec<Token>) {
    let mut has_errored = false;
    for token in tokens.iter() {
        match &token.token_type {
            TokenType::Unknown(unk) => {
                has_errored = true;
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unexpected character: {}",
                    token.line,
                    unk.as_str()
                )
                .unwrap()
            }
            _ => println!("{} {} null", token.token_type, token.lexeme),
        }
    }

    if has_errored {
        process::exit(65);
    }
}
pub fn scanner(source: String) {
    let lines = source.lines();
    let mut tokens = Vec::<Token>::new();
    let mut lines_count = 0;

    for (i, line) in lines.enumerate() {
        let mut line_tokens = tokenize(line, i + 1);
        tokens.append(&mut line_tokens);
        lines_count += 1;
    }
    tokens.push(Token {
        lexeme: String::new(),
        token_type: TokenType::EOF,
        line: lines_count + 1,
    });
    print_tokens(&tokens)
}
