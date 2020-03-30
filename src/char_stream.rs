//! # char_stream
//! file reader

use std::fs::File;
use std::io::{prelude::*, Bytes};

pub struct CharStream {
    _file_name: String,
    buf: Bytes<File>,
    current_char: Option<u8>,
    current_line: usize,
    current_column: usize,
    current_line_content: String,
}

impl CharStream {
    pub fn new(file_name: &str) -> std::io::Result<CharStream> {
        let file = File::open(file_name)?;
        let buf = file.bytes();
        Ok(CharStream {
            _file_name: String::from(file_name),
            buf: buf,
            current_char: None,
            current_line: 0,
            current_column: 0,
            current_line_content: String::new(),
        })
    }

    pub fn next_char(&mut self) {
        if self.current_char == Some('\n' as u8) {
            self.current_line += 1;
            self.current_column = 0;
            self.current_line_content.clear();
        }

        match self.buf.next() {
            Some(Ok(c)) => {
                self.current_column += 1;
                self.current_char = Some(c);
                self.current_line_content.push(c as char);

                if self.current_line == 0 {
                    self.current_line = 1;
                }
            }
            Some(Err(e)) => {
                panic!("{}", e);
            }
            None => {
                // EOF
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
            if cur as char == 'a' {
                println!("line {}, col {}", self.current_line, self.current_column);
            }
            self.next_char();
        }
    }

    // None: EOF
    pub fn get_current_char_u8(&self) -> Option<u8> {
        self.current_char
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.current_char.map(|x| x as char)
    }

    pub fn get_current_line(&self) -> usize {
        self.current_line
    }

    pub fn get_current_column(&self) -> usize {
        self.current_column
    }

    pub fn get_current_line_content(&self) -> &String {
        &self.current_line_content
    }
}
