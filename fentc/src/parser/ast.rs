use std::error::Error;

use crate::parser::{identifier::Identifier, span::Spanned, token::Token};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    parameters: Identifier,
}

#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mutability {
    /// Value can be reassigned
    Mutable,

    /// Value cannot be reassigned
    Immutable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Assignment {
        lhs: Identifier,
        rhs: Spanned<Expression>,
    },
    Let {
        identifier: Identifier,
        mutability: Mutability,
        initializer: Spanned<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Nil value literal
    Nil,

    /// unit literal
    Unit,

    /// Identifier binding
    Ident(Identifier),

    /// String Literal
    String(String),

    /// Boolean Literal
    Bool(bool),

    /// Number Literal
    Number(f64),

    If {
        condition: Box<Spanned<Self>>,
        then: Box<Spanned<Self>>,
        or_else: Box<Spanned<Self>>,
    },

    While {
        condition: Box<Spanned<Self>>,
        then: Box<Spanned<Self>>,
    },

    Block {
        statements: Vec<Spanned<Statement>>,
        eval: Box<Spanned<Self>>,
    },

    Binary {
        lhs: Box<Spanned<Self>>,
        op: Operator,
        rhs: Box<Spanned<Self>>,
    },
}

#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    /// +
    Add,

    /// -
    Sub,

    /// *
    Mul,

    /// /
    Div,

    /// mod
    Mod,

    /// and
    And,

    /// or
    Or,

    /// nor
    Nor,

    /// <
    Less,

    /// <=
    LessOrEqual,

    /// >
    Greater,

    /// >=
    GreaterOrEqual,

    /// ==
    Equals,

    /// !=
    NotEquals,
}

impl<'a> TryFrom<Token<'a>> for BinaryOperator {
    type Error = NotABinaryOperator;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        use super::token::Token as T;

        Ok(match value {
            T::Add => Self::Add,
            T::Sub => Self::Sub,
            T::Mul => Self::Mul,
            T::Div => Self::Div,
            T::Mod => Self::Mod,
            T::And => Self::And,
            T::Or => Self::Or,
            T::Nor => Self::Nor,
            T::Less => Self::Less,
            T::LessOrEqual => Self::LessOrEqual,
            T::Greater => Self::Greater,
            T::GreaterOrEqual => Self::GreaterOrEqual,
            T::Equals => Self::Equals,
            T::NotEquals => Self::NotEquals,

            _ => return Err(NotABinaryOperator),
        })
    }
}

/// Operator is not a binary operator
#[derive(displaydoc::Display, Debug, Clone, Copy)]
pub struct NotABinaryOperator;

impl std::error::Error for NotABinaryOperator {}

/// Operator is not a unary operator
#[derive(displaydoc::Display, Debug, Clone, Copy)]
pub struct NotAUnaryOperator;

impl std::error::Error for NotAUnaryOperator {}
