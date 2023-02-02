#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod ast;

#[test]
fn parser_test() {
    assert_eq!(
        parser::ExprsParser::new().parse("(").unwrap(),
        ast::Expr::Empty
    )
}

fn main() {
    println!("Hello, world!");
}
