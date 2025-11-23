use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    And,
    Not,
    Or,
    Nor,

    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,

    Equals,
    NotEquals,

    Assign,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "mod",
            Operator::And => "and",
            Operator::Not => "not",
            Operator::Or => "or",
            Operator::Nor => "nor",
            Operator::Less => "<",
            Operator::LessOrEqual => "<=",
            Operator::Greater => ">",
            Operator::GreaterOrEqual => ">=",
            Operator::Equals => "==",
            Operator::NotEquals => "!=",
            Operator::Assign => "=",
        })
    }
}
