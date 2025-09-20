use crate::interpreter::*;

pub fn eval_to_number(input: &str) -> f64 {
    match Evaluator::eval_once(input).unwrap() {
        Expr::Number(n) => n,
        other => panic!("Expected number, got {:?}", other),
    }
}

pub fn eval_to_string(input: &str) -> String {
    match Evaluator::eval_once(input).unwrap() {
        Expr::String(s) => s,
        other => panic!("Expected string, got {:?}", other),
    }
}

pub fn eval_to_list(input: &str) -> Vec<Expr> {
    match Evaluator::eval_once(input).unwrap() {
        Expr::List(l) => l,
        other => panic!("Expected list, got {:?}", other),
    }
}