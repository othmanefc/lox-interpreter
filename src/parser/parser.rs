use std::io::{self, Write};
use std::process;

use crate::exprs::Expr;
use crate::tokens::{Keyword, Token, TokenType};
use crate::utils::trim_string;

pub fn parse_tokens(tokens: &Vec<Token>) -> Vec<Option<Expr>> {
    let mut exprs = Vec::<Option<Expr>>::new();
    let mut tokens_iter = tokens.iter().peekable();
    while let Some(token) = tokens_iter.next() {
        let expr = match &token.token_type {
            TokenType::Keyword {
                kw: Keyword::True, ..
            } => Some(Expr::Bool(true)),
            TokenType::Keyword {
                kw: Keyword::False, ..
            } => Some(Expr::Bool(false)),
            TokenType::Keyword {
                kw: Keyword::Nil, ..
            } => Some(Expr::Nil),
            TokenType::Number(val) => Some(Expr::Number(string_to_f64(val).unwrap())),
            TokenType::String { string, .. } => Some(Expr::String(trim_string(&string))),
            TokenType::Blank => None,
            TokenType::LeftParen => {
                let mut closed = false;
                let mut enclosed_exprs = Vec::<Expr>::new();
                while let Some(next_token) = tokens_iter.next() {
                    match &next_token.token_type {
                        TokenType::RightParen => {
                            closed = true;
                            break;
                        }
                        _ => {
                            let mut vec = Vec::new();
                            vec.push(next_token.clone());
                            let exp = parse_tokens(&vec);
                            let mut a = exp.into_iter().flatten().collect::<Vec<Expr>>();
                            enclosed_exprs.append(&mut a);
                        }
                    }
                }
                if closed == false {
                    writeln!(io::stderr(), "Error: Unmatched parentheses.").unwrap();
                    process::exit(65)
                }
                Some(Expr::Grouping(enclosed_exprs))
            }
            _ => None,
        };
        exprs.push(expr)
    }
    exprs
}

fn string_to_f64(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.parse()
}
