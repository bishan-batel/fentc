use crate::parser::{identifier::Identifier, span::Spanned};

pub struct Program {
    functions: Vec<(Identifier, Function)>,
}

impl Program {
    pub fn new(functions: Vec<(Identifier, Function)>) -> Self {
        Self { functions }
    }

    pub fn functions(&self) -> &[(Identifier, Function)] {
        &self.functions
    }
}

pub enum Type {
    Named(Identifier),
}

pub struct Function {}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Nil value literal
    Nil,

    /// Identifier binding
    Ident(Identifier),

    /// String Literal
    String(String),

    /// Boolean Literal
    Bool(bool),

    /// Number Literal
    Number(f64),

    /// Array Literal
    Array(Vec<Spanned<Self>>),
}
