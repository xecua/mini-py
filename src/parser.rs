#![allow(dead_code)]
use std::io::BufReader;
use std::fs::File;

use crate::token::Token;

pub struct Parser {
    buf: BufReader<File>,
    current_token: Token,
    line: u32,
    column: u32,
}

impl Parser {
    pub fn new(file_name: &str) -> std::io::Result<Parser> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        Ok(Parser {
            buf: reader,
            current_token: Token::EMPTY,
            line: 0,
            column: 0
        })
    }

    pub fn get_current_token(&self) -> Token {
        self.current_token.clone()
    }

    pub fn next_token(&mut self) {
        
    }
}
