use std::fmt::Display;

use crate::exprs::Expr;
use crate::tokens::{Operator, TokenType};

enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            // Value::Grouping(g) => {
            //     let g_j = g
            //         .iter()
            //         .map(|e| format!("{e}"))
            //         .collect::<Vec<String>>()
            //         .join(", ");
            //     write!(f, "{}", g_j)
            // }
        }
    }
}

fn evaluate_expr(expr: &Expr) -> Result<Value, &'static str> {
    match expr {
        Expr::Number(t) => Ok(Value::Number(t.to_owned())),
        Expr::String(s) => Ok(Value::String(s.to_owned())),
        Expr::Bool(b) => Ok(Value::Bool(b.to_owned())),
        Expr::Nil => Ok(Value::Nil),
        Expr::Grouping(v) => evaluate_expr(v),
        Expr::Unary { operator, right } => {
            let res = evaluate_expr(right)?;
            match operator.token_type {
                TokenType::Minus => match res {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err("Unsupported value for Minus token"),
                },
                TokenType::Bang => match res {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    Value::Nil => Ok(Value::Bool(true)),
                    Value::Number(n) => Ok(Value::Bool(n == 0.0)),
                    Value::String(s) => Ok(Value::Bool(s.is_empty())),
                    // _ => Err("Unsupported value for Bang token"),
                },
                _ => Err("Unsupported token type for unary expression"),
            }
        }
        Expr::Binary {
            operator,
            left,
            right,
        } => {
            let left = evaluate_expr(left)?;
            let right = evaluate_expr(right)?;
            match (left, right) {
                (Value::Number(n), Value::Number(m)) => match operator.token_type {
                    TokenType::Star => Ok(Value::Number(n * m)),
                    TokenType::Slash => Ok(Value::Number(n / m)),
                    TokenType::Plus => Ok(Value::Number(n + m)),
                    TokenType::Minus => Ok(Value::Number(n - m)),
                    TokenType::Greater => Ok(Value::Bool(n > m)),
                    TokenType::Less => Ok(Value::Bool(n < m)),
                    TokenType::Operator {
                        op: Operator::LessEqual,
                    } => Ok(Value::Bool(n <= m)),
                    TokenType::Operator {
                        op: Operator::GreaterEqual,
                    } => Ok(Value::Bool(n >= m)),
                    TokenType::Operator {
                        op: Operator::EqualEqual,
                    } => Ok(Value::Bool(n == m)),
                    TokenType::Operator {
                        op: Operator::BangEqual,
                    } => Ok(Value::Bool(n != m)),
                    _ => Err("Unsupported token type for binary expression on numbers"),
                },
                (Value::String(s), Value::String(t)) => match operator.token_type {
                    TokenType::Plus => Ok(Value::String(s + t.as_str())),
                    TokenType::Operator {
                        op: Operator::EqualEqual,
                    } => Ok(Value::Bool(s == t)),
                    TokenType::Operator {
                        op: Operator::BangEqual,
                    } => Ok(Value::Bool(s != t)),
                    _ => Err("Unsupported token type for binary expression on strings"),
                },
                (Value::Number(_), Value::String(_)) | (Value::String(_), Value::Number(_)) => {
                    match operator.token_type {
                        TokenType::Operator {
                            op: Operator::EqualEqual,
                        } => Ok(Value::Bool(false)),
                        TokenType::Operator {
                            op: Operator::BangEqual,
                        } => Ok(Value::Bool(true)),
                        _ => {
                            Err("Unsupported token type for binary expression on string and number")
                        }
                    }
                }
                (_, _) => Err("Unsupported values for binary expression"),
            }
        } // _ => Err("Unsupported expression type"),
    }
}

pub fn evaluate_exprs(exprs: Vec<Option<Expr>>) {
    for expr in exprs {
        if let Some(expr_v) = expr {
            let evaluated = evaluate_expr(&expr_v);
            evaluated
                .and_then(|value| {
                    println!("{value}");
                    Ok(())
                })
                .unwrap_or_else(|e| panic!("Evaluation failed: {}", e));
        }
    }
}
