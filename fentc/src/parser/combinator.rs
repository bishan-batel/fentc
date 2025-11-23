use chumsky::{input::ValueInput, prelude::*};

use crate::parser::{
    ast::{Program, Type},
    identifier::Identifier,
    span::{Span, Spanned},
    token::Token,
};

#[must_use]
pub fn program<'src, I>()
-> impl Parser<'src, I, Program, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    just(Token::Fn)
        .map_with(|x, e| (x, e.span()))
        .map_with(|_, _| Program::new(vec![]))
}

#[must_use]
pub fn type_parser<'src, I>()
-> impl Parser<'src, I, Spanned<Type>, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    select! {
        Token::Identifier(identifier) => identifier
    }
    .map_with(|x, e| (Type::Named(Identifier::new(x.into())), e.span()))
}

#[must_use]
pub fn fn_args<'src, I>()
-> impl Parser<'src, I, Program, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    just(Token::Fn)
        .map_with(|x, e| (x, e.span()))
        .map_with(|_, _| Program::new(vec![]))
}
