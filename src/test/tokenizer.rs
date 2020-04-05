use crate::token::Token::{self, *};
use crate::tokenizer::Tokenizer;

fn test_tokenizer(file_name: &str, expected: Vec<Token>) {
    let mut tokenizer = Tokenizer::new(file_name).unwrap();
    tokenizer.next_token();
    for tok in expected.iter() {
        assert_eq!(tok, tokenizer.get_current_token());
        tokenizer.next_token();
    }
}

#[test]
fn test_test_1_py() {
    #[rustfmt::skip]
    test_tokenizer("testcase/test_1.py", vec![
        PRINT, INT(1), PLUS, INT(2), NEWLINE, EOF
    ]);
}

#[test]
fn test_test_2() {
    #[rustfmt::skip]
    test_tokenizer("testcase/test_2.py", vec![
        DEF, ID(String::from("test")), LPAREN, ID(String::from("poi")), RPAREN, COLON,
        NEWLINE, INDENT, ID(String::from("a")), EQ, FLOAT(1.0), NEWLINE, PRINT, ID(String::from("a")), NEWLINE, DEDENT, IF, ID(String::from("__name__")), EQEQ, STRING(String::from("__main__")), COLON, NEWLINE, INDENT, ID(String::from("test")), LPAREN, INT(2), RPAREN, NEWLINE, DEDENT, EOF
    ]);
}

#[test]
fn test_fizzbuzz() {
    #[rustfmt::skip]
    test_tokenizer("testcase/fizzbuzz.py", vec![
        FOR, ID(String::from("i")), IN, ID(String::from("range")), LPAREN, INT(100), RPAREN, COLON, NEWLINE, INDENT, IF, ID(String::from("i")), MOD, INT(15), EQEQ, INT(0), COLON, NEWLINE, INDENT, PRINT, STRING(String::from("fizzbuzz")), NEWLINE, DEDENT, ELIF, ID(String::from("i")), MOD, INT(5), EQEQ, INT(0), COLON, NEWLINE, INDENT, PRINT, STRING(String::from("fizz")), NEWLINE, DEDENT, ELIF, ID(String::from("i")), MOD, INT(3), EQEQ, INT(0), COLON, NEWLINE, INDENT, PRINT, STRING(String::from("buzz")), NEWLINE, DEDENT, ELSE, COLON, NEWLINE, INDENT, PRINT, ID(String::from("i")), NEWLINE, DEDENT, DEDENT, EOF
    ]);
}

#[test]
fn test_test_3() {
    #[rustfmt::skip]
    test_tokenizer("testcase/test_3.py", vec![
        PRINT, LBRACKET, INT(1), COMMA, INT(2), COMMA, INT(3), RBRACKET, EOF
    ]);
}
