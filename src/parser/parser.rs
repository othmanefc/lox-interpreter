use std::io::{self, Write};
use std::process;

use crate::exprs::Expr;
use crate::tokens::{Keyword, Token, TokenType};
use crate::utils::trim_string;

pub fn parse_tokens(tokens_iter: &mut std::slice::Iter<'_, Token>) -> Vec<Option<Expr>> {
    let mut expressions = Vec::new();

    while tokens_iter.len() > 0 {
        expressions.push(parse_tokens_into_expr(tokens_iter));
    }
    expressions
}

pub fn parse_tokens_into_expr(tokens_iter: &mut std::slice::Iter<'_, Token>) -> Option<Expr> {
    let mut expr = None;
    if let Some(token) = tokens_iter.next() {
        expr = match &token.token_type {
            TokenType::Keyword {
                kw: Keyword::True, ..
            } => Some(Expr::Bool(true)),
            TokenType::Keyword {
                kw: Keyword::False, ..
            } => Some(Expr::Bool(false)),
            TokenType::Keyword {
                kw: Keyword::Nil, ..
            } => Some(Expr::Nil),
            TokenType::Number(val) => Some(Expr::Number(string_to_f64(&val).unwrap())),
            TokenType::String { string, .. } => {
                Some(Expr::String(trim_string(&string)))
            }
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

                let enclosed_exprs: Vec<Expr> = parse_tokens(&mut enclosed_tokens.iter())
                    .into_iter()
                    .flatten()
                    .collect();

                if depth != 0 || enclosed_exprs.len() == 0 {
                    writeln!(io::stderr(), "Error: Unmatched parentheses.").unwrap();
                    process::exit(65);
                }
                Some(Expr::Grouping(enclosed_exprs))
            }
            TokenType::Minus | TokenType::Bang => Some(Expr::Unary {
                operator: token.clone(),
                right: Box::new(parse_tokens_into_expr(tokens_iter).clone()?),
            }),
            _ => None,
        };
    }
    expr
}

fn string_to_f64(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.parse()
}
