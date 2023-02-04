extern crate lalrpop_util;

mod parser {
    lalrpop_mod!(pub parser);
    use crate::ast::Exp;

    fn test_debug_idempotence(s: &str) {
        assert_eq!(
            s,
            &format!("{:?}", parser::ExpParser::new().parse(s).unwrap())
        );
    }

    #[test]
    fn parser_test() {
        test_debug_idempotence("()");
        test_debug_idempotence("1");
        test_debug_idempotence("-1");
        test_debug_idempotence("true");
        test_debug_idempotence("false");
        test_debug_idempotence("a");
        test_debug_idempotence("a + 1");
        test_debug_idempotence("a +. 1");
        test_debug_idempotence("a - 1");
        test_debug_idempotence("a -. 1");
        test_debug_idempotence("a *. 1");
        test_debug_idempotence("a /. 1");
        test_debug_idempotence("a = 1");
        test_debug_idempotence("a <= 1");
        test_debug_idempotence("let a = 1 in a");
        test_debug_idempotence("if a then 1 else 2");
        test_debug_idempotence("1, true, a, false");
        test_debug_idempotence("let rec f a b = a + b in f a b");
        test_debug_idempotence("let (x, y) = a in x + y");
        test_debug_idempotence("Array.create e f");
        test_debug_idempotence("e.(f)");
        test_debug_idempotence("e.(f) <- (g + 1)");
    }
}
