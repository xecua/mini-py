use crate::token::Token::*;
use crate::tokenizer::Tokenizer;

#[test]
fn test_test_1_py() {
    let mut tokenizer = Tokenizer::new("testcase/test_1.py").unwrap();
    #[rustfmt::skip]
    let expected = vec![
        PRINT, INT(1), PLUS, INT(2), NEWLINE, EOF
    ];
    tokenizer.next_token();
    for tok in expected.iter() {
        assert_eq!(tok, tokenizer.get_current_token());
        tokenizer.next_token();
    }
}

#[test]
fn test_test_2() {
    let mut tokenizer = Tokenizer::new("testcase/test_2.py").unwrap();
    #[rustfmt::skip]
    let expected = vec![
        DEF, ID(String::from("test")), LPAREN, ID(String::from("poi")), RPAREN, COLON,
        NEWLINE, INDENT, ID(String::from("a")), EQ, FLOAT(1.0), NEWLINE, PRINT, ID(String::from("a")), NEWLINE, DEDENT, IF, ID(String::from("__name__")), EQEQ, STRING(String::from("__main__")), COLON, NEWLINE, INDENT, ID(String::from("test")), LPAREN, INT(2), RPAREN, NEWLINE, DEDENT, EOF
    ];
    tokenizer.next_token();
    for tok in expected.iter() {
        assert_eq!(tok, tokenizer.get_current_token());
        tokenizer.next_token();
    }
}

#[test]
fn test_fizzbuzz() {
    let mut tokenizer = Tokenizer::new("testcase/fizzbuzz.py").unwrap();
    #[rustfmt::skip]
    let expected = vec![
        FOR, ID(String::from("i")), IN, ID(String::from("range")), LPAREN, INT(100), RPAREN, COLON, NEWLINE, INDENT, IF, ID(String::from("i")), MOD, INT(15), EQEQ, INT(0), COLON, NEWLINE, INDENT, PRINT, STRING(String::from("fizzbuzz")), NEWLINE, DEDENT, ELIF, ID(String::from("i")), MOD, INT(5), EQEQ, INT(0), COLON, NEWLINE, INDENT, PRINT, STRING(String::from("fizz")), NEWLINE, DEDENT, ELIF, ID(String::from("i")), MOD, INT(3), EQEQ, INT(0), COLON, NEWLINE, INDENT, PRINT, STRING(String::from("buzz")), NEWLINE, DEDENT, ELSE, COLON, NEWLINE, INDENT, PRINT, ID(String::from("i")), NEWLINE, DEDENT, DEDENT, EOF
    ];
    tokenizer.next_token();
    for tok in expected.iter() {
        assert_eq!(tok, tokenizer.get_current_token());
        tokenizer.next_token();
    }
}
