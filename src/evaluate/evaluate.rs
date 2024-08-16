use std::fmt::Display;

use crate::exprs::Expr;

fn evaluate_expr(expr: &Expr) -> Result<Box<dyn Display>, &'static str> {
    match expr {
        Expr::Number(t) => Ok(Box::new(t.to_owned())),
        Expr::String(s) => Ok(Box::new(s.to_owned())),
        Expr::Bool(b) => Ok(Box::new(b.to_owned())),
        Expr::Nil => Ok(Box::new("nil")),
        // Expr::Grouping(vec_expr) => {
        // let mut final_ev = Vec::new();
        // for ex in vec_expr {
        //     let ev = evaluate_expr(ex)?;
        //     final_ev.push(ev)
        // }
        // Ok(Box::new(final_ev))
        // let evaluated: Result<Vec<Box<dyn Display>>, &'static str> =
        //     vec_expr.iter().map(evaluate_expr).collect();
        // Ok(Box::new(evaluated?))
        // }
        Expr::Grouping(vec_expr) => {
            let evaluated: Result<Vec<Box<dyn Display>>, &'static str> =
                vec_expr.iter().map(evaluate_expr).collect();
            let result = evaluated?
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(", ");

            Ok(Box::new(result))
        }
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
