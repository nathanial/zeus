#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Symbol(String),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(String),
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
