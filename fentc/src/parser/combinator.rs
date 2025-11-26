use chumsky::prelude::*;

use crate::parser::{
    ast::{ArgumentList, Expression, Function, Module, Mutability, Statement, Type},
    identifier::Identifier,
    span::{Span, Spanned},
    token::Token,
};

pub trait ValueInput<'src>:
    chumsky::input::ValueInput<'src, Token = Token<'src>, Span = Span>
{
}

impl<'src, I: chumsky::input::ValueInput<'src, Token = Token<'src>, Span = Span>> ValueInput<'src>
    for I
{
}

pub type Extra<'src> = extra::Err<Rich<'src, Token<'src>, Span>>;

#[must_use]
pub fn module<'src, I>() -> impl Parser<'src, I, Module, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    function()
        .repeated()
        .collect()
        .map(|functions| Module { functions })
}

#[must_use]
fn r#type<'src, I>() -> impl Parser<'src, I, Spanned<Type>, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    select! {
        Token::Identifier(identifier) => match identifier {
            "f32" => Type::F32,
            "unit" => Type::Unit,
            _ => return None
        }
    }
    .map_with(|x, e| (x, e.span()))
    .labelled("Type")
}

#[must_use]
fn arguments<'src, I>() -> impl Parser<'src, I, ArgumentList, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    let argument = select! {
        Token::Identifier(ident) => Identifier::new(ident.into())
    }
    .map_with(|ident, e| (ident, e.span()))
    .then_ignore(just(Token::Colon))
    .then(r#type());

    argument
        .separated_by(just(Token::Comma))
        .collect::<Vec<_>>()
}

#[must_use]
fn function<'src, I>() -> impl Parser<'src, I, Spanned<Function>, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    just(Token::Fn)
        .ignore_then(arguments().delimited_by(just(Token::ParenOpen), just(Token::ParenClosed)))
        .then(just(Token::FatArrow).ignore_then(r#type()).or_not())
        .then(block())
        .map_with(|((arguments, returns), body), e| {
            (
                Function {
                    parameters: arguments,
                    returns: returns.unwrap_or((Type::Unit, e.span())),
                    body,
                },
                e.span(),
            )
        })
}

#[must_use]
fn block<'src, I>() -> impl Parser<'src, I, Vec<Spanned<Statement>>, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    statement().repeated().collect::<Vec<_>>().labelled("block")
}

#[must_use]
fn statement<'src, I>() -> impl Parser<'src, I, Spanned<Statement>, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    let expression = expression();

    let name_binding = select! {
        Token::Let => Mutability::Immutable,
        Token::Var => Mutability::Mutable,
    }
    .then(
        select! { Token::Identifier(ident) => Identifier::new(ident.into()) }
            .map_with(|x, e| (x, e.span())),
    )
    .then_ignore(just(Token::Assign))
    .then(expression.clone())
    .map_with(|((mutability, identifier), initializer), e| {
        (
            Statement::Let {
                identifier,
                mutability,
                initializer,
            },
            e.span(),
        )
    });

    let statement_expression = expression
        .clone()
        .map(|(expr, span)| (Statement::Expression(expr), span));

    statement_expression.or(name_binding)
}

#[must_use]
fn expression<'src, I>() -> impl Parser<'src, I, Spanned<Expression>, Extra<'src>> + Clone
where
    I: ValueInput<'src>,
{
    recursive(|expr| {
        let value = select! {
            Token::Integer(n) => Expression::I32(n as i32),
            Token::Float(x) => Expression::F32(x as f32)
        }
        .map_with(|x, e| (x, e.span()));

        let identifier = select! {
            Token::Identifier(ident) => Identifier::new(ident.into())
        };

        let atom = value
            // parenthesis  delimited
            .or(expr
                .clone()
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClosed)))
            // identifier binding
            .or(identifier.map_with(|ident, e| (Expression::Ident(ident), e.span())));

        atom
    })
    .boxed()
}

#[cfg(test)]
mod test {}
