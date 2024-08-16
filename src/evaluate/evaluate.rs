use std::fmt::Display;

use crate::exprs::Expr;

fn evaluate_expr(expr: &Expr) -> Result<Box<dyn Display>, &'static str> {
    match expr {
        Expr::Number(t) => Ok(Box::new(t.to_owned())),
        Expr::String(s) => Ok(Box::new(s.to_owned())),
        Expr::Bool(b) => Ok(Box::new(b.to_owned())),
        Expr::Nil => Ok(Box::new("nil")),
        _ => Err("Unsupported expression type"),
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
