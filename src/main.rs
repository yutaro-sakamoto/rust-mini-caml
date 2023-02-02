#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);
lalrpop_mod!(pub calculator2);
lalrpop_mod!(pub calculator3);
lalrpop_mod!(pub calculator4);
lalrpop_mod!(pub calculator5);
lalrpop_mod!(pub calculator6);
lalrpop_mod!(pub calculator7);
lalrpop_mod!(pub calculator8);

pub mod ast;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Calculator6Error {
    InputTooBig,
    OddNumber,
}

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

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new().parse("22*44+66").unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

#[test]
fn calculator5() {
    let expr = calculator5::ExprsParser::new().parse("").unwrap();
    assert_eq!(&format!("{:?}", expr), "[]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");
}

#[test]
fn calculator6() {
    use lalrpop_util::ParseError;

    let expr = calculator6::ExprsParser::new().parse("2147483648");
    assert!(expr.is_err());
    assert_eq!(
        expr.unwrap_err(),
        ParseError::User {
            error: Calculator6Error::InputTooBig
        }
    );

    let expr = calculator6::ExprsParser::new().parse("3");
    assert!(expr.is_err());
    assert_eq!(
        expr.unwrap_err(),
        ParseError::User {
            error: Calculator6Error::OddNumber
        }
    );
}

#[test]
fn calculator7() {
    let mut errors = Vec::new();

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * error) + 3)]");

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "22 * 44 + 66, *3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "*")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

#[test]
fn calculator8() {
    let scale = 2;
    let expr = calculator8::ExprsParser::new()
        .parse(scale, "11 * 22 + 33")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
