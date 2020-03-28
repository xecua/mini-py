#![allow(dead_code)]

use crate::errors;
use crate::stream::CharStream;
use crate::token::Token;

pub struct Tokenizer {
    current_token: Token,
    indent_stack: Vec<usize>,
    leading_space: usize,
    char_stream: CharStream,
    token_buf: String,
}

impl Tokenizer {
    pub fn new(file_name: &str) -> std::io::Result<Tokenizer> {
        Ok(Tokenizer {
            current_token: Token::EMPTY,
            indent_stack: Vec::new(),
            leading_space: 0,
            char_stream: CharStream::new(file_name)?,
            token_buf: String::new(),
        })
    }

    pub fn get_current_token(&self) -> Token {
        self.current_token.clone()
    }

    pub fn next_token(&mut self) {
        self.token_buf.clear(); // clear token buffer

        self.skip_space();

        // 必要があればindent/unindentを生成
        if self.leading_space > self.indent_stack[self.indent_stack.len() - 1] {
            // indent
            self.indent_stack.push(self.leading_space);
            self.current_token = Token::INDENT;
            return;
        } else if self.leading_space < self.indent_stack[self.indent_stack.len() - 1] {
            // unindent
            self.indent_stack.pop();
            if self.leading_space > self.indent_stack[self.indent_stack.len() - 1] {
                // IndentationError: unindent does not match any outer indentation level
                errors::wrong_indent(&self);
            }
            self.current_token = Token::UNINDENT;
            return;
        }

        self.current_token = match self.char_stream.get_current_char() {
            None => Token::EOF,
            Some('\n') => {
                // NEWLINE
                self.char_stream.next_char();
                Token::NEWLINE
            }
            Some('=') => {
                // =, ==
                // 3つ以上は2回に分ける(字句解析器では何もしない。構文解析器で弾かれる)
                self.char_stream.next_char();
                if self.char_stream.get_current_char() == Some('=') {
                    self.char_stream.next_char();
                    Token::EQEQ
                } else {
                    Token::EQ
                }
            }
            Some('!') => {
                // !=
                self.char_stream.next_char();
                if self.char_stream.get_current_char() != Some('=') {
                    errors::invalid_syntax(&self);
                }
                self.char_stream.next_char();
                Token::NEQ
            }
            Some('>') => {
                // >, >=, >>
                self.char_stream.next_char();
                match self.char_stream.get_current_char() {
                    Some('=') => {
                        // >=
                        self.char_stream.next_char();
                        Token::GEQ
                    }
                    Some('>') => {
                        // >>
                        self.char_stream.next_char();
                        Token::RSHIFT
                    }
                    _ => {
                        // >
                        Token::GT
                    }
                }
            }
            Some('<') => {
                // <, <=, <<
                self.char_stream.next_char();
                match self.char_stream.get_current_char() {
                    Some('=') => {
                        // >=
                        self.char_stream.next_char();
                        Token::LEQ
                    }
                    Some('<') => {
                        // <<
                        self.char_stream.next_char();
                        Token::LSHIFT
                    }
                    _ => {
                        // >
                        Token::LT
                    }
                }
            }
            Some('+') => {
                self.char_stream.next_char();
                Token::PLUS
            }
            Some('-') => {
                self.char_stream.next_char();
                Token::MINUS
            }
            Some('*') => {
                self.char_stream.next_char();
                Token::MUL
            }
            Some('/') => {
                self.char_stream.next_char();
                Token::DIV
            }
            Some('%') => {
                self.char_stream.next_char();
                Token::MOD
            }
            Some('~') => {
                self.char_stream.next_char();
                Token::TILDE
            }
            Some('^') => {
                self.char_stream.next_char();
                Token::XOR
            }
            Some('&') => {
                self.char_stream.next_char();
                Token::AMP
            }
            Some('|') => {
                self.char_stream.next_char();
                Token::BAR
            }
            Some('(') => {
                self.char_stream.next_char();
                Token::LPAREN
            }
            Some(')') => {
                self.char_stream.next_char();
                Token::RPAREN
            }
            Some('{') => {
                self.char_stream.next_char();
                Token::LBRACE
            }
            Some('}') => {
                self.char_stream.next_char();
                Token::RBRACE
            }
            Some('[') => {
                self.char_stream.next_char();
                Token::LBRACKET
            }
            Some(']') => {
                self.char_stream.next_char();
                Token::RBRACKET
            }
            Some('.') => {
                self.char_stream.next_char();
                Token::PERIOD
            }
            Some(',') => {
                self.char_stream.next_char();
                Token::COMMA
            }
            Some(':') => {
                self.char_stream.next_char();
                Token::COLON
            }
            Some('0'..='9') => self.tokenize_number(),
            Some('"') => self.tokenize_string(),
            Some(_) => self.tokenize_other(),
        };
    }

    // space ::= \s* | #.*(?=\n)
    // 要件
    // ・スペースを読み飛ばす
    // ・コメントも読み飛ばす
    // ・行の先頭であればleading_spaceに加算 -> `is_in_leading_space`
    // ・スペース+コメントのみの行からは何も生成しない -> is_in_leading_spaceがtrueかつ改行のとき カウントをリセット
    // 終了条件
    // ・コメント中: 改行
    //   ・ただし、スペース+コメントのみの行であった場合は続行する
    //   ・この場合、スペースのカウントをリセットすること
    // ・その他: 次のトークン(改行含む)
    // ・共通: EOF(処理全体が終了?)
    fn skip_space(&mut self) {
        // 直前の文字が改行 = 行の先頭
        let is_in_leading_space = self.char_stream.get_current_char() == Some('\n');

        let mut is_in_comment = false;

        // 読み進める
        self.char_stream.next_char();

        loop {
            let c = self.char_stream.get_current_char();

            if c.is_none() {
                // EOF。特にすることなし
                break;
            } else if c == Some('#') {
                // コメント開始
                is_in_comment = true;
            } else if c == Some(' ') {
                if is_in_leading_space {
                    self.leading_space += 1;
                }
            } else if c == Some('\n') {
                if is_in_leading_space {
                    // スペースとコメントしかない行だった 続行
                    // 行の先頭のスペース数をリセット
                    self.leading_space = 0;
                } else if is_in_comment {
                    // !is_in_leading_space
                    // スペースとコメント以外になにかを含む行のコメント中
                    return;
                }
            } else if !is_in_comment {
                // 次のトークン
                return;
            }
            // else : コメント中で、改行ではない文字
        }
    }

    // number ::= digit* (if begin with 0, raise SyntaxError)
    // float ::= digit+ . digit*
    //         | . digit*
    // digit ::= 0-9
    fn tokenize_number(&mut self) -> Token {
        self.token_buf
            .push(self.char_stream.get_current_char().unwrap());
        self.char_stream.next_char();
        while let Some(d @ '0'..='9') = self.char_stream.get_current_char() {
            self.token_buf.push(d);
            self.char_stream.next_char();
        }
        // floating point number
        if self.char_stream.get_current_char() == Some('.') {
            self.token_buf.push('.');
            while let Some(d @ '0'..='9') = self.char_stream.get_current_char() {
                self.token_buf.push(d);
                self.char_stream.next_char();
            }
            Token::FLOAT(self.token_buf.parse().unwrap())
        }
        // integer
        else {
            if self.token_buf.get(0..1).unwrap() == "0" && self.token_buf.len() > 1 {
                // SyntaxError: invalid token
                errors::invalid_token(&self);
            }
            Token::INT(self.token_buf.parse().unwrap())
        }
    }

    // string ::= " ([^"\] | \.)* "
    fn tokenize_string(&mut self) -> Token {
        // 直前の文字が'\'
        let mut in_espace = false;

        loop {
            self.char_stream.next_char();
            if let Some(c) = self.char_stream.get_current_char() {
                if c == '\\' && !in_espace {
                    in_espace = true;
                } else if c == '"' && !in_espace {
                    break;
                } else {
                    in_espace = false;
                }
                self.token_buf.push(c);
            } else {
                // SyntaxError: EOL while scanning string literal
                errors::eol_while_string(&self);
            }
        }
        Token::STRING(self.token_buf.clone())
    }

    fn tokenize_other(&mut self) -> Token {
        Token::NONE
    }
}
