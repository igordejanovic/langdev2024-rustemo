/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::arithmetic::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = String;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Add {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Mul {
    pub left: Box<E>,
    pub right: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    Add(Add),
    Mul(Mul),
    Paren(Box<E>),
    Num(Number),
}
pub fn e_add(_ctx: &Ctx, left: E, right: E) -> E {
    E::Add(Add {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_mul(_ctx: &Ctx, left: E, right: E) -> E {
    E::Mul(Mul {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn e_paren(_ctx: &Ctx, e: E) -> E {
    E::Paren(Box::new(e))
}
pub fn e_num(_ctx: &Ctx, number: Number) -> E {
    E::Num(number)
}
