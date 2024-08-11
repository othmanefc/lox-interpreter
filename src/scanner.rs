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
    Blank,
    String {
        string: String,
        finished: bool,
    },
    Number(String),
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
            ' ' | '\n' | '\t' => TokenType::Blank,
            '"' => {
                let mut finished = false;
                let mut string_ = r#"""#.to_string();
                while let Some(new_char) = chars.next() {
                    string_.push(new_char);
                    if new_char == '"' {
                        finished = true;
                        break;
                    }
                }
                TokenType::String {
                    string: string_,
                    finished,
                }
            }
            char if char.is_digit(10) => {
                let mut comma = false;
                let mut num_as_string = String::from(char);
                while let Some(new_char) = chars.peek() {
                    if new_char.is_digit(10) {
                        num_as_string.push(*new_char);
                        chars.next();
                    } else if *new_char == '.' && !comma {
                        num_as_string.push(*new_char);
                        comma = true;
                        chars.next();
                    } else {
                        break;
                    }
                }
                TokenType::Number(num_as_string)
            }
            _ => TokenType::Unknown(char.into()),
        };

        if matches!(token_type, TokenType::Comment) {
            return tokens;
        }

        tokens.push(Token {
            lexeme: match &token_type {
                TokenType::Unknown(_) => String::new(),
                TokenType::String {
                    string,
                    finished: _,
                } => string.clone(),
                TokenType::Number(num) => num.clone(),
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
            TokenType::String {
                string: _,
                finished: false,
            } => {
                has_errored = true;
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unterminated string.",
                    token.line,
                )
                .unwrap()
            }
            TokenType::String {
                string,
                finished: true,
            } => println!(
                "{} {} {}",
                token.token_type,
                token.lexeme,
                trim_string(&string)
            ),
            TokenType::Number(_) => {
                println!(
                    "{} {} {}",
                    token.token_type,
                    token.lexeme,
                    format_number_as_string(&token.lexeme)
                )
            }
            TokenType::Blank => (),
            _ => println!("{} {} null", token.token_type, token.lexeme),
        }
    }

    if has_errored {
        process::exit(65);
    }
}

fn trim_string(to_split: &String) -> String {
    let length = to_split.len();
    to_split[1..length - 1].to_string()
}

fn format_number_as_string(num_as_string: &String) -> String {
    let mut new_string = num_as_string.clone();
    if new_string.ends_with('.') {
        new_string.push('0')
    }
    new_string
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
