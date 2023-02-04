#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod ast;
pub mod id;
mod test;
pub mod typedef;
pub mod typing;

fn main() {
    let source = "let a=1 in a";
    let x = parser::ExpParser::new().parse(source);
    println!("result: {}", format!("{x:?}"));
    println!("Hello, world!");
}
