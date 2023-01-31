#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);
lalrpop_mod!(pub calculator2);
lalrpop_mod!(pub calculator3);

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("(((22)))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22))").is_ok());
}

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("(((22)))").is_ok());
    assert!(calculator2::TermParser::new().parse("((22))").is_ok());
}

#[test]
fn calculator3() {
    assert_eq!(calculator3::ExprParser::new().parse("2+3"), Ok(5));
    assert_eq!(calculator3::ExprParser::new().parse("2+3*4"), Ok(14));
    assert_eq!(calculator3::ExprParser::new().parse("2*3+4"), Ok(10));
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
