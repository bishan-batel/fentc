use std::fmt::Display;
use std::str::FromStr;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"([ \t\n\f]+)|(\/\/.*\n)|(\/\*.*\*\/)")]
pub enum Token<'a> {
    /// A single operator
    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("mod")]
    Mod,

    #[token("and")]
    And,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[token("nor")]
    Nor,

    #[token("<")]
    Less,

    #[token("<=")]
    LessOrEqual,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterOrEqual,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[token("=")]
    Assign,

    #[token("{")]
    CurlyBraceOpen,

    #[token("}")]
    CurlyBraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClosed,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token(";")]
    Semicolon,

    #[token("=>")]
    FatArrow,

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    /// String containing a valid number string
    #[regex(
        r"-?(?:0|[1-9]\d*)(?:\.\d+)", 
        |lex| f64::from_str(lex.slice()).expect("Integer regex must be invalid for this to fail.")
    )]
    Float(f64),

    #[regex(
        r"-?(?:0|[1-9]\d*)", 
        |lex| i64::from_str(lex.slice()).expect("Integer regex must be invalid for this to fail.")
    )]
    Integer(i64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| &lex.slice()[1..(lex.slice().len() - 1)])]
    String(&'a str),

    #[regex(
        r#"[^0-9\s\=\[\]\(\)\!\@\#\$\%\^\&\*\-\+\{\}<>\,\.\`\~\/\\\;\:\'\"][^\s\=\[\]\(\)\!\@\#\$\%\^\&\*\-\+\{\}<>\,\.\`\~\/\\\;\:\'\"]*"#, 
        priority = 0
    )]
    Identifier(&'a str),

    #[token("fn", priority = 100)]
    Fn,

    #[token("nil", priority = 100)]
    Nil,

    #[token("return", priority = 100)]
    Return,

    #[token("if", priority = 100)]
    If,

    #[token("unless", priority = 100)]
    Unless,

    #[token("else", priority = 100)]
    Else,

    #[token("while", priority = 100)]
    While,

    #[token("until", priority = 100)]
    Until,

    #[token("for", priority = 100)]
    For,

    #[token("in", priority = 100)]
    In,

    #[token("do", priority = 100)]
    Do,

    #[token("let", priority = 100)]
    Let,

    #[token("var", priority = 100)]
    Var,

    Error,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurlyBraceOpen => f.write_str("{"),
            Self::CurlyBraceClose => f.write_str("}"),
            Self::BracketOpen => f.write_str("["),
            Self::BracketClose => f.write_str("]"),
            Self::ParenOpen => f.write_str("("),
            Self::ParenClosed => f.write_str(")"),
            Self::Colon => f.write_str(":"),
            Self::Comma => f.write_str(","),
            Self::Dot => f.write_str("."),
            Self::Semicolon => f.write_str(";"),
            Self::FatArrow => f.write_str("=>"),
            Self::Bool(b) => b.fmt(f),
            Self::Float(x) => x.fmt(f),
            Self::Integer(n) => n.fmt(f),
            Self::String(s) => f.write_fmt(format_args!("\"{s}\"")),
            Self::Identifier(iden) => iden.fmt(f),
            Self::Fn => f.write_str("fn"),
            Self::Nil => f.write_str("nil"),
            Self::Return => f.write_str("return"),
            Self::If => f.write_str("if"),
            Self::Unless => f.write_str("unless"),
            Self::Else => f.write_str("else"),
            Self::While => f.write_str("while"),
            Self::Until => f.write_str("until"),
            Self::For => f.write_str("for"),
            Self::In => f.write_str("in"),
            Self::Do => f.write_str("do"),
            Self::Let => f.write_str("let"),
            Self::Var => f.write_str("var"),
            Self::Error => f.write_str("[Error]"),
            Self::Add => f.write_str("+"),
            Self::Sub => f.write_str("-"),
            Self::Mul => f.write_str("*"),
            Self::Div => f.write_str("/"),
            Self::Mod => f.write_str("mod"),
            Self::And => f.write_str("and"),
            Self::Not => f.write_str("not"),
            Self::Or => f.write_str(""),
            Self::Nor => f.write_str("nor"),
            Self::Less => f.write_str("<"),
            Self::LessOrEqual => f.write_str("<="),
            Self::Greater => f.write_str(">"),
            Self::GreaterOrEqual => f.write_str(">="),
            Self::Equals => f.write_str("=="),
            Self::NotEquals => f.write_str("!="),
            Self::Assign => f.write_str("="),
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::Token;

    #[test]
    fn nil() {
        let mut lexer = Token::lexer("nil");
        assert_eq!(lexer.next(), Some(Ok(Token::Nil)));

        let mut lexer = Token::lexer("nil nil nil");
        assert_eq!(lexer.next(), Some(Ok(Token::Nil)));
        assert_eq!(lexer.next(), Some(Ok(Token::Nil)));
        assert_eq!(lexer.next(), Some(Ok(Token::Nil)));
    }

    #[test]
    fn whitespace() {
        let mut lexer = Token::lexer("  \n  \t\t");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn comments() {
        let mut lexer = Token::lexer("// bruh moment\n\n");
        assert_eq!(lexer.next(), None);

        let mut lexer = Token::lexer("// bruh moment\n\nnil // hello world \nnil");
        assert_eq!(lexer.next(), Some(Ok(Token::Nil)));
        assert_eq!(lexer.next(), Some(Ok(Token::Nil)));
    }

    #[test]
    fn bool() {
        let lexer: Vec<_> = Token::lexer("true false").map(Result::unwrap).collect();

        assert_eq!(lexer, &[Token::Bool(true), Token::Bool(false)]);
    }

    #[test]
    fn general() {
        color_eyre::install().unwrap();

        let toks: Vec<_> = Token::lexer(
            r#"
            if true {
                a = (a + 1 + 0.5)
            } else {
                b.what = "huh"
            }
        "#,
        )
        .map(|x| x.unwrap())
        .collect();

        assert_eq!(
            toks,
            &[
                Token::If,
                Token::Bool(true),
                Token::CurlyBraceOpen,
                Token::Identifier("a"),
                Token::Assign,
                Token::ParenOpen,
                Token::Identifier("a"),
                Token::Add,
                Token::Integer(1),
                Token::Add,
                Token::Float(0.5),
                Token::ParenClosed,
                Token::CurlyBraceClose,
                Token::Else,
                Token::CurlyBraceOpen,
                Token::Identifier("b"),
                Token::Dot,
                Token::Identifier("what"),
                Token::Assign,
                Token::String("huh"),
                Token::CurlyBraceClose,
            ]
        );
    }
}
