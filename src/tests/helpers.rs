use crate::interpreter::evaluator::Evaluator as Eval;
use crate::interpreter::*;

pub fn eval_to_number(input: &str) -> f64 {
    let result = Evaluator::eval_once(input).unwrap();
    Eval::to_number(&result).expect("Expected a number")
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

pub fn eval_to_bool(input: &str) -> bool {
    let result = Evaluator::eval_once(input).unwrap();
    Evaluator::is_truthy(&result)
}
