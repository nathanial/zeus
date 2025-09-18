//! Abstract syntax tree definitions for Zeus.

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub declarations: Vec<Declaration>,
}

impl Module {
    pub fn prelude() -> Self {
        Module {
            name: "Prelude".into(),
            declarations: vec![Declaration::Function(Function {
                name: "identity".into(),
                arguments: vec!["a".into()],
                body: Expression::Variable("a".into()),
            })],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Declaration {
    Function(Function),
    Constant { name: String, value: Expression },
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub body: Expression,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Variable(String),
    Lambda {
        parameter: String,
        body: Box<Expression>,
    },
    Apply {
        function: Box<Expression>,
        argument: Box<Expression>,
    },
    Literal(Literal),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Integer(i64),
    Boolean(bool),
    Text(String),
}
