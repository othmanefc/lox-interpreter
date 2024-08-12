use phf::phf_map;
use std::io::{self, Write};
use std::process;
use strum_macros::Display;

#[derive(Display, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum Operator {
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(Display, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Display, Clone)]
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
    Identifier(String),
    #[strum(to_string = "{kw}")]
    Keyword {
        kw: Keyword,
        val: String,
    },
}
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "and" => Keyword::And,
    "class" => Keyword::Class,
    "else" => Keyword::Else,
    "false" => Keyword::False,
    "for" => Keyword::For,
    "fun" =>  Keyword::Fun,
    "if" =>  Keyword::If,
    "nil" =>  Keyword::Nil,
    "or" =>  Keyword::Or,
    "print" =>  Keyword::Print,
    "return" =>  Keyword::Return,
    "super" =>  Keyword::Super,
    "this" =>  Keyword::This,
    "true" =>  Keyword::True,
    "var" =>  Keyword::Var,
    "while" => Keyword::While,
};

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
                TokenType::Number(num_as_string)
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
                    TokenType::Keyword { kw: kw.clone(), val: cont }
                } else {
                    TokenType::Identifier(cont)
                }
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
            token_type: token_type.clone(),
            line: line_number,
        });

        match token_type {
            TokenType::Number(num) if num.ends_with('.') => tokens.push(Token {
                lexeme: '.'.into(),
                token_type: TokenType::Dot,
                line: line_number,
            }),
            _ => (),
        }
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
                    token.lexeme.trim_end_matches('.'),
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
    } else if !new_string.contains('.') {
        new_string.push_str(".0")
    } else if new_string.ends_with(".00") {
        new_string = new_string.replace(".00", ".0");
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
