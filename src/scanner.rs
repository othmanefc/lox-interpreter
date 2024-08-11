// use convert_case::{Case, Casing};
// use std::fmt;
use std::io::{self, Write};
use std::process;
use strum_macros::Display;

#[derive(Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum Operator {
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum TokenType {
    RightParen,
    LeftParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Equal,
    #[strum(to_string = "{op}")]
    Operator {
        op: Operator,
    },
    Bang,
    Less,
    Greater,
    EOF,
    Unknown(String),
    Comment,
    Slash,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

fn operators(
    char_: Option<char>,
    operator_type: TokenType,
    simple_type: TokenType,
    match_char: char,
) -> TokenType {
    if let Some(i) = char_ {
        if i == match_char {
            return operator_type;
        }
    }
    return simple_type;
}

fn tokenize_line(line: &str, line_number: usize) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = line.chars().peekable();

    while let Some(char) = chars.next() {
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
            '=' => operators(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::EqualEqual,
                },
                TokenType::Equal,
                '=',
            ),
            '!' => operators(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::BangEqual,
                },
                TokenType::Bang,
                '=',
            ),
            '<' => operators(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::LessEqual,
                },
                TokenType::Less,
                '=',
            ),
            '>' => operators(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::GreaterEqual,
                },
                TokenType::Greater,
                '=',
            ),
            '/' => operators(
                chars.peek().copied(),
                TokenType::Comment,
                TokenType::Slash,
                '/',
            ),
            _ => TokenType::Unknown(char.into()),
        };

        if matches!(token_type, TokenType::Comment) {
            return tokens;
        }

        tokens.push(Token {
            lexeme: match token_type {
                TokenType::Unknown(_) => String::new(),
                TokenType::Operator { op: _ } => {
                    let mut s = char.to_string();
                    if let Some(next_char) = chars.next() {
                        s.push(next_char)
                    }
                    s
                }
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
        let mut line_tokens = tokenize_line(line, i + 1);
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
