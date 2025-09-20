use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBracket, // For vector literals
    RightBracket,
    Symbol(String),
    Keyword(String), // Self-evaluating keyword symbols (e.g., :keyword)
    Integer(i64),
    Float(f64),
    String(String),
    Character(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolData {
    Interned(String),        // Normal symbols that are interned
    Uninterned(String, u64), // Uninterned symbols from gensym with unique ID
    Keyword(String),         // Self-evaluating keyword symbols
}

impl SymbolData {
    pub fn name(&self) -> &str {
        match self {
            SymbolData::Interned(name) => name,
            SymbolData::Uninterned(name, _) => name,
            SymbolData::Keyword(name) => name,
        }
    }

    pub fn as_str(&self) -> &str {
        self.name()
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, SymbolData::Keyword(_))
    }

    pub fn is_uninterned(&self) -> bool {
        matches!(self, SymbolData::Uninterned(_, _))
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Rational { numerator: i64, denominator: i64 },
    Symbol(SymbolData),
    String(String),
    Character(char),
    Cons(Box<Expr>, Box<Expr>),
    List(Vec<Expr>),
    Vector(Vec<Expr>),
    HashTable(Rc<HashMap<HashKey, Expr>>),
}

// Custom PartialEq implementation for Expr to handle HashTable comparison
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Integer(a), Expr::Integer(b)) => a == b,
            (Expr::Float(a), Expr::Float(b)) => a == b,
            (
                Expr::Rational {
                    numerator: n1,
                    denominator: d1,
                },
                Expr::Rational {
                    numerator: n2,
                    denominator: d2,
                },
            ) => n1 == n2 && d1 == d2,
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Character(a), Expr::Character(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            (Expr::Cons(a_car, a_cdr), Expr::Cons(b_car, b_cdr)) => {
                a_car == b_car && a_cdr == b_cdr
            }
            (Expr::Vector(a), Expr::Vector(b)) => a == b,
            (Expr::HashTable(a), Expr::HashTable(b)) => {
                // HashTables are equal if they have the same keys and values
                a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).map_or(false, |v2| v == v2))
            }
            _ => false,
        }
    }
}

// A hashable key type for hash tables
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum HashKey {
    Integer(i64),
    Symbol(String),
    String(String),
    Character(char),
    Keyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    Message(String),
    Throw { tag: Expr, value: Expr },
    ReturnFrom { name: String, value: Expr },
    Go { label: String },
}

pub type EvalResult = Result<Expr, EvalError>;

impl EvalError {
    pub fn message<T: Into<String>>(msg: T) -> Self {
        EvalError::Message(msg.into())
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Message(msg) => write!(f, "{}", msg),
            EvalError::Throw { tag, .. } => write!(f, "Uncaught throw for tag {:?}", tag),
            EvalError::ReturnFrom { name, .. } => {
                write!(f, "Unhandled return-from for block {}", name)
            }
            EvalError::Go { label } => write!(f, "Unhandled go to label {}", label),
        }
    }
}

impl std::error::Error for EvalError {}

impl From<String> for EvalError {
    fn from(value: String) -> Self {
        EvalError::Message(value)
    }
}

impl From<&str> for EvalError {
    fn from(value: &str) -> Self {
        EvalError::Message(value.to_string())
    }
}
