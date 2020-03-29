//! # parser
//! 構文解析器
// LALRPOP(https://github.com/lalrpop/lalrpop)でも良いかも...?

use crate::tokenizer::Tokenizer;

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

