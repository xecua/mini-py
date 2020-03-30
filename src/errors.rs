use crate::tokenizer::Tokenizer;
use crate::parser::Parser;

// tokenizer
pub fn wrong_indent(_tokenizer: &Tokenizer) -> ! {
    panic!();
}
pub fn invalid_token(_tokenizer: &Tokenizer) -> ! {
    panic!();
}
pub fn eol_while_string(_tokenizer: &Tokenizer) -> ! {
    panic!();
}
pub fn invalid_syntax(_tokenizer: &Tokenizer) -> ! {
    panic!();
}

// parser
pub fn unexpected_token(_parser: &Parser) -> ! {
    panic!();
}
