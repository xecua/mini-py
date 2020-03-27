#![allow(dead_code)]
use std::io::{
    BufReader,
    prelude::*
};
use std::fs::File;

pub struct CharStream {
    buf: BufReader<File>,
    current_char: Option<u8>,
    current_line: usize,
    current_column: usize
}

impl CharStream {
    pub fn new(file_name: &str) -> std::io::Result<CharStream> {
        let file = File::open(file_name)?;
        let buf = BufReader::new(file);
        Ok(CharStream {
            buf: buf,
            current_char: None,
            current_line: 0,
            current_column: 0
        })
    }

    pub fn next_char(&mut self) {
        let mut cur = [0u8];
        match self.buf.read(&mut cur) {
            Ok(1) => {
                self.current_column += 1;
                self.current_char = Some(cur[0]);

                if self.current_line == 0 {
                    self.current_line = 1;
                }
                if char::from(cur[0]) == '\n' {
                    self.current_line += 1;
                    self.current_column = 0;
                }
            }
            _ => {
                self.current_char = None;
            }
        }
    }

    // count up line in the file
    pub fn lc(&mut self) {
        self.next_char();
        while self.current_char.is_some() {
            self.next_char();
        }
        println!("{}", self.current_line);
    }

    // line and col of char 'a'
    pub fn apos(&mut self) {
        self.next_char();
        while let Some(cur) = self.current_char {
            if char::from(cur) == 'a' {
                println!("line {}, col {}", self.current_line, self.current_column);
            }
            self.next_char();
        }
    }

    // None: EOF
    pub fn get_current_char(&self) -> Option<u8> {
        self.current_char
    }

    pub fn get_current_line(&self) -> usize {
        self.current_line
    }

    pub fn get_current_column(&self) -> usize {
        self.current_column
    }
}
