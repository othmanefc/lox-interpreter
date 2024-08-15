use std::io::{self, Write};
use std::iter::Peekable;
use std::process;

use crate::exprs::Expr;
use crate::tokens::{Keyword, Operator, Token, TokenType};
use crate::utils::trim_string;

pub fn parse_tokens(tokens_iter: &mut std::slice::Iter<'_, Token>) -> Vec<Option<Expr>> {
    let mut expressions = Vec::new();
    let mut tokens_peek = tokens_iter.to_owned().peekable();
    while tokens_peek.peek().is_some() {
        expressions.push(parse_expression(&mut tokens_peek));
    }
    expressions
}

fn string_to_f64(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.parse()
}

fn parse_expression(tokens_iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<Expr> {
    parse_binary(tokens_iter)
}

fn parse_binary(tokens_iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<Expr> {
    let mut left = parse_unary(tokens_iter)?;
    while let Some(token) = tokens_iter.peek() {
        match &token.token_type {
            TokenType::Slash
            | TokenType::Star
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Greater
            | TokenType::Less
            | TokenType::Operator {
                op: Operator::LessEqual,
            }
            | TokenType::Operator {
                op: Operator::GreaterEqual,
            }
            | TokenType::Operator {
                op: Operator::EqualEqual,
            }
            | TokenType::Operator {
                op: Operator::BangEqual,
            } => {
                let consumed_token = tokens_iter.next()?;
                let right = parse_unary(tokens_iter)?;
                left = Expr::Binary {
                    operator: consumed_token.clone(),
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Some(left)
}

fn parse_unary(tokens_iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<Expr> {
    let mut expr = None;
    if let Some(token) = tokens_iter.peek() {
        match &token.token_type {
            TokenType::Minus | TokenType::Bang => {
                let consumed_token = tokens_iter.next()?;
                let right = parse_unary(tokens_iter)?;
                expr = Some(Expr::Unary {
                    right: Box::new(right),
                    operator: consumed_token.clone(),
                });
            }
            _ => {
                expr = parse_primary(tokens_iter);
            }
        }
    }
    expr
}

fn parse_primary(tokens_iter: &mut Peekable<std::slice::Iter<'_, Token>>) -> Option<Expr> {
    if let Some(token) = tokens_iter.next() {
        match &token.token_type {
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
            TokenType::String {
                string,
                finished: true,
            } => Some(Expr::String(trim_string(&string))),
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
            TokenType::EOF => None,
            t => {
                writeln!(io::stderr(), "Error: lol: {t}").unwrap();
                process::exit(65);
            }
        }
    } else {
        None
    }
}
