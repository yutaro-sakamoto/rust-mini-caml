#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod ast;
pub mod id;
pub mod typedef;

#[test]
fn parser_test() {
    assert_eq!(
        *(parser::ExpParser::new().parse("()").unwrap()),
        ast::Exp::Unit
    )
}

fn main() {
    let source = "let a=1 in a";
    let x = parser::ExpParser::new().parse(source);
    println!("result: {}", format!("{x:?}"));
    println!("Hello, world!");
}
