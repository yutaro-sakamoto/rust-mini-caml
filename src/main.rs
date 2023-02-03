#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod ast;
pub mod id;
pub mod typedef;

#[test]
fn parser_test() {
    assert_eq!(parser::ExpParser::new().parse("(").unwrap(), ast::Exp::Unit)
}

fn main() {
    println!("Hello, world!");
}
