#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Symbol(String),
    Keyword(String),  // Self-evaluating keyword symbols (e.g., :keyword)
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolData {
    Interned(String),      // Normal symbols that are interned
    Uninterned(String, u64), // Uninterned symbols from gensym with unique ID
    Keyword(String),       // Self-evaluating keyword symbols
}

impl SymbolData {
    pub fn name(&self) -> &str {
        match self {
            SymbolData::Interned(name) => name,
            SymbolData::Uninterned(name, _) => name,
            SymbolData::Keyword(name) => name,
        }
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, SymbolData::Keyword(_))
    }

    pub fn is_uninterned(&self) -> bool {
        matches!(self, SymbolData::Uninterned(_, _))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(SymbolData),
    String(String),
    List(Vec<Expr>),
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
