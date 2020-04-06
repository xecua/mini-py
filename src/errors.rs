use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

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
pub fn unexpected_token(parser: &Parser) -> ! {
    eprintln!(
        "Syntax Error:unexpected token {:?} at {} line {}, column {}\n{}\n{}^",
        parser.get_current_token(),
        parser.get_file_name(),
        parser.get_current_line(),
        parser.get_current_column(),
        parser.get_current_line_content(),
        " ".repeat(parser.get_current_line_content().len())
    );
    std::process::exit(1);
}

// evaluator
pub fn name_error(name: &String) -> ! {
    eprintln!("Name Error: {} is not defined", name);
    std::process::exit(1);
}
