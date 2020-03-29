use crate::tokenizer::Tokenizer;

// 構文解析器
pub struct Parser {
    tokenizer: Tokenizer
}

impl Parser {
    /// generate new Parser
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser {
            tokenizer: tokenizer
        }
    }

    pub fn parse(&mut self) {

    }
}

