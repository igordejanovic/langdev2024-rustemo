/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::arithmetic::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = i32;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.parse().unwrap()
}
pub type E = i32;
pub fn e_add(_ctx: &Ctx, left: E, right: E) -> E {
    left + right
}
pub fn e_mul(_ctx: &Ctx, left: E, right: E) -> E {
    left * right
}
pub fn e_paren(_ctx: &Ctx, e: E) -> E {
    e
}
pub fn e_num(_ctx: &Ctx, number: Number) -> E {
    number
}
