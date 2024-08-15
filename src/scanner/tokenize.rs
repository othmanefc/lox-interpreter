use crate::tokens::{Operator, Token, TokenType, KEYWORDS};
use crate::utils::{format_number_as_string, trim_string};
use std::io::{self, Write};
use std::process;

fn gen_operator(
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
        // let mut token_type = None;
        let token_type = match char {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            '*' => Some(TokenType::Star),
            '.' => Some(TokenType::Dot),
            ',' => Some(TokenType::Comma),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '-' => Some(TokenType::Minus),
            '=' => Some(gen_operator(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::EqualEqual,
                },
                TokenType::Equal,
                '=',
            )),
            '!' => Some(gen_operator(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::BangEqual,
                },
                TokenType::Bang,
                '=',
            )),
            '<' => Some(gen_operator(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::LessEqual,
                },
                TokenType::Less,
                '=',
            )),
            '>' => Some(gen_operator(
                chars.peek().copied(),
                TokenType::Operator {
                    op: Operator::GreaterEqual,
                },
                TokenType::Greater,
                '=',
            )),
            '/' => Some(gen_operator(
                chars.peek().copied(),
                TokenType::Comment,
                TokenType::Slash,
                '/',
            )),
            ' ' | '\n' | '\t' => None,
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
                Some(TokenType::String {
                    string: string_,
                    finished,
                })
            }
            char if char.is_digit(10) => {
                let mut has_decimal = false;
                let mut num_as_string = String::from(char);

                while let Some(&next_char) = chars.peek() {
                    match next_char {
                        '0'..='9' => {
                            num_as_string.push(next_char);
                            chars.next();
                        }
                        '.' if !has_decimal => {
                            num_as_string.push(next_char);
                            has_decimal = true;
                            chars.next();
                        }
                        _ => break,
                    }
                }
                Some(TokenType::Number(num_as_string))
            }
            char if char.is_alphanumeric() || char == '_' => {
                let mut cont = String::from(char);
                while let Some(&n) = chars.peek() {
                    match n {
                        n if n.is_alphanumeric() || n == '_' => {
                            cont.push(n);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                if let Some(kw) = KEYWORDS.get(cont.as_str()) {
                    Some(TokenType::Keyword {
                        kw: kw.clone(),
                        val: cont,
                    })
                } else {
                    Some(TokenType::Identifier(cont))
                }
            }
            _ => Some(TokenType::Unknown(char.into())),
        };

        if let Some(token_type_u) = token_type {
            if matches!(token_type_u, TokenType::Comment) {
                return tokens;
            }
            tokens.push(Token {
                lexeme: match &token_type_u {
                    TokenType::Unknown(_) => String::new(),
                    TokenType::String {
                        string,
                        finished: _,
                    } => string.clone(),
                    TokenType::Number(num) => num.clone(),
                    TokenType::Identifier(st) => st.clone(),
                    TokenType::Keyword { kw: _, val } => val.clone(),
                    TokenType::Operator { op: _ } => {
                        let mut s = char.to_string();
                        if let Some(next_char) = chars.next() {
                            s.push(next_char)
                        }
                        s
                    }
                    _ => char.into(),
                },
                token_type: token_type_u.clone(),
                line: line_number,
            });

            match token_type_u {
                TokenType::Number(num) if num.ends_with('.') => tokens.push(Token {
                    lexeme: '.'.into(),
                    token_type: TokenType::Dot,
                    line: line_number,
                }),
                _ => (),
            }
        }
    }
    tokens
}

pub fn print_tokens(tokens: &Vec<Token>) {
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
                    token.lexeme.trim_end_matches('.'),
                    format_number_as_string(&token.lexeme)
                )
            }
            _ => println!("{} {} null", token.token_type, token.lexeme),
        }
    }

    if has_errored {
        process::exit(65);
    }
}

pub fn scanner(source: String) -> Vec<Token> {
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
    tokens
}
