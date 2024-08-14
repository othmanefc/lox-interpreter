use std::fmt::Display;

#[derive(Clone)]
pub enum Expr {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
    // Unary {
    //     operator: Token,
    //     right: Box<Expr>,
    // },
    // Binary {
    //     operator: Token,
    //     left: Box<Expr>,
    //     right: Box<Expr>,
    // },
    // Grouping(Vec<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => f.write_fmt(format_args!("{b}")),
            Expr::Nil => f.write_str("nil"),
            Expr::Number(n) => f.write_fmt(format_args!("{n:?}")),
            Expr::String(s) => f.write_fmt(format_args!("{s:?}")),
            // Expr::Unary { operator, right } => {
            //     f.write_fmt(format_args!("{} {right}", operator.lexeme))
            // }
            // Expr::Binary {
            //     operator,
            //     left,
            //     right,
            // } => f.write_fmt(format_args!("({} {left} {right})", operator.lexeme)),
            // Expr::Grouping(_) => todo!(),
        }
    }
}

pub fn print_exprs(exprs: &Vec<Option<Expr>>) {
    for expr in exprs {
        if let Some(val) = expr {
            match val {
                Expr::String(s) => println!("{s}"),
                _ => println!("{val}"),
            }
        }
    }
}
