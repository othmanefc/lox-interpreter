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
                let mut depth = 1;
                let mut enclosed_tokens = Vec::new();

                while let Some(next_token) = tokens_iter.next() {
                    match &next_token.token_type {
                        TokenType::LeftParen => {
                            depth += 1;
                            enclosed_tokens.push(next_token.clone());
                        }
                        TokenType::RightParen => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                            enclosed_tokens.push(next_token.clone());
                        }
                        _ => enclosed_tokens.push(next_token.clone()),
                    }
                }

                let enclosed_exprs: Vec<Expr> = parse_tokens(&enclosed_tokens)
                    .into_iter()
                    .flatten()
                    .collect();

                if depth != 0 || enclosed_exprs.len() == 0 {
                    writeln!(io::stderr(), "Error: Unmatched parentheses.").unwrap();
                    process::exit(65);
                }
                Some(Expr::Grouping(enclosed_exprs))
            }
            TokenType::Minus | TokenType::Bang => {
                let final_expr = if let Some(next_token) = tokens_iter.next() {
                    let vec_next_token = vec![next_token.clone()];
                    let vec_next_expr: Vec<Expr> = parse_tokens(&vec_next_token)
                        .into_iter()
                        .flatten()
                        .collect();
                    if let Some(next_expr) = vec_next_expr.into_iter().next() {
                        Expr::Unary {
                            operator: token.clone(),
                            right: Box::new(next_expr),
                        }
                    } else {
                        panic!("Expected expression after unary operator")
                    }
                } else {
                    panic!("Unexpected end of input after unary operator")
                };
                Some(final_expr)
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
