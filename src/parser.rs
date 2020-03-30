//! # parser
//! 構文解析器
// LALRPOP(https://github.com/lalrpop/lalrpop)でも良いかも...?

use crate::ast::AST;
use crate::errors;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::io;

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    /// generate new Parser
    pub fn new(file_name: &str) -> io::Result<Parser> {
        let tokenizer = Tokenizer::new(file_name)?;
        Ok(Parser {
            tokenizer: tokenizer,
        })
    }

    pub fn parse(&mut self) -> Box<AST> {
        let mut tree: Vec<Box<AST>> = Vec::new();
        self.tokenizer.next_token();
        loop {
            match self.tokenizer.get_current_token() {
                Token::NEWLINE => {
                    continue;
                }
                Token::EOF => {
                    break;
                }
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::MUL
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE
                | Token::DEL
                | Token::PASS
                | Token::BREAK
                | Token::CONTINUE
                | Token::RETURN
                | Token::GLOBAL
                | Token::IF
                | Token::WHILE
                | Token::FOR
                | Token::DEF => tree.push(self.parse_stmt()),
                _ => errors::unexpected_token(&self),
            };
        }
        Box::new(AST::File(tree))
    }

    fn parse_stmt(&mut self) -> Box<AST> {
        match self.tokenizer.get_current_token() {
            Token::IF | Token::WHILE | Token::FOR | Token::DEF => self.parse_compound_stmt(),
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE
            | Token::DEL
            | Token::PASS
            | Token::BREAK
            | Token::CONTINUE
            | Token::RETURN
            | Token::GLOBAL => self.parse_simple_stmt(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_compound_stmt(&mut self) -> Box<AST> {
        match self.tokenizer.get_current_token() {
            Token::IF => self.parse_if_stmt(),
            Token::WHILE => self.parse_while_stmt(),
            Token::FOR => self.parse_for_stmt(),
            Token::DEF => self.parse_funcdef(),
            _ => errors::unexpected_token(&self),
        }
    }
    fn parse_if_stmt(&mut self) -> Box<AST> {
        self.eat(Token::IF);
        let test = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_test(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::COLON);
        let suite = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE
            | Token::DEL
            | Token::PASS
            | Token::BREAK
            | Token::CONTINUE
            | Token::RETURN
            | Token::GLOBAL
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        let mut elif: Vec<(Box<AST>, Box<AST>)> = Vec::new();
        while *self.tokenizer.get_current_token() == Token::ELIF {
            self.eat(Token::ELIF);
            let test = match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE => self.parse_test(),
                _ => errors::unexpected_token(&self),
            };
            self.eat(Token::COLON);
            let suite = match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::MUL
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE
                | Token::DEL
                | Token::PASS
                | Token::BREAK
                | Token::CONTINUE
                | Token::RETURN
                | Token::GLOBAL
                | Token::NEWLINE => self.parse_suite(),
                _ => errors::unexpected_token(&self),
            };
            elif.push((test, suite));
        }
        let else_ = if *self.tokenizer.get_current_token() == Token::ELSE {
            self.eat(Token::ELSE);
            let test = match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE => self.parse_test(),
                _ => errors::unexpected_token(&self),
            };
            self.eat(Token::COLON);
            let suite = match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::MUL
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE
                | Token::DEL
                | Token::PASS
                | Token::BREAK
                | Token::CONTINUE
                | Token::RETURN
                | Token::GLOBAL
                | Token::NEWLINE => self.parse_suite(),
                _ => errors::unexpected_token(&self),
            };
            Some((test, suite))
        } else {
            None
        };
        Box::new(AST::IfStmt((test, suite), elif, else_))
    }

    fn parse_simple_stmt(&mut self) -> Box<AST> {
        let small_stmt = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE
            | Token::DEL
            | Token::PASS
            | Token::BREAK
            | Token::CONTINUE
            | Token::RETURN
            | Token::GLOBAL => self.parse_small_stmt(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::NEWLINE);
        Box::new(AST::SimpleStmt(small_stmt))
    }

    fn parse_test(&mut self) -> Box<AST> {
        let or_test = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_or_test(),
            _ => errors::unexpected_token(&self),
        };
        let (condition, test) = if *self.tokenizer.get_current_token() == Token::IF {
            self.eat(Token::IF);
            let condition = match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE => self.parse_or_test(),
                _ => errors::unexpected_token(&self),
            };
            self.eat(Token::ELSE);
            let test = match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE => self.parse_test(),
                _ => errors::unexpected_token(&self),
            };
            (Some(condition), Some(test))
        } else {
            (None, None)
        };
        Box::new(AST::Test(or_test, condition, test))
    }

    fn parse_suite(&mut self) -> Box<AST> {
        if *self.tokenizer.get_current_token() == Token::NEWLINE {
            self.eat(Token::NEWLINE);
            self.eat(Token::INDENT);
            let mut stmt: Vec<Box<AST>> = Vec::new();
            loop {
                match self.tokenizer.get_current_token() {
                    Token::NOT
                    | Token::PLUS
                    | Token::MINUS
                    | Token::TILDE
                    | Token::MUL
                    | Token::LPAREN
                    | Token::LBRACE
                    | Token::LBRACKET
                    | Token::ID(_)
                    | Token::INT(_)
                    | Token::FLOAT(_)
                    | Token::STRING(_)
                    | Token::NONE
                    | Token::TRUE
                    | Token::FALSE
                    | Token::DEL
                    | Token::PASS
                    | Token::BREAK
                    | Token::CONTINUE
                    | Token::RETURN
                    | Token::GLOBAL
                    | Token::IF
                    | Token::WHILE
                    | Token::FOR
                    | Token::DEF => {
                        stmt.push(self.parse_stmt());
                    }
                    Token::DEDENT => break,
                    _ => errors::unexpected_token(&self),
                };
            }
            self.eat(Token::DEDENT);
            Box::new(AST::Suite(stmt))
        } else {
            Box::new(AST::Suite(vec![match self.tokenizer.get_current_token() {
                Token::NOT
                | Token::PLUS
                | Token::MINUS
                | Token::TILDE
                | Token::MUL
                | Token::LPAREN
                | Token::LBRACE
                | Token::LBRACKET
                | Token::ID(_)
                | Token::INT(_)
                | Token::FLOAT(_)
                | Token::STRING(_)
                | Token::NONE
                | Token::TRUE
                | Token::FALSE
                | Token::DEL
                | Token::PASS
                | Token::BREAK
                | Token::CONTINUE
                | Token::RETURN
                | Token::GLOBAL => self.parse_small_stmt(),
                _ => errors::unexpected_token(&self),
            }]))
        }
    }

    fn parse_while_stmt(&mut self) -> Box<AST> {
        self.eat(Token::WHILE);
        let test = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_test(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::COLON);
        let suite = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE
            | Token::DEL
            | Token::PASS
            | Token::BREAK
            | Token::CONTINUE
            | Token::RETURN
            | Token::GLOBAL
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        Box::new(AST::WhileStmt(test, suite))
    }

    fn parse_for_stmt(&mut self) -> Box<AST> {
        self.eat(Token::FOR);
        let exprlist = match self.tokenizer.get_current_token() {
            Token::MUL
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_exprlist(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::IN);
        let testlist = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_testlist(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::COLON);
        let suite = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE
            | Token::DEL
            | Token::PASS
            | Token::BREAK
            | Token::CONTINUE
            | Token::RETURN
            | Token::GLOBAL
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        Box::new(AST::ForStmt(exprlist, testlist, suite))
    }

    fn parse_funcdef(&mut self) -> Box<AST> {
        self.eat(Token::DEF);
        let name = match self.tokenizer.get_current_token() {
            Token::ID(name) => name.to_owned(),
            _ => errors::unexpected_token(&self),
        };
        self.tokenizer.next_token();
        let parameters = match self.tokenizer.get_current_token() {
            Token::LPAREN => self.parse_parameters(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::COLON);
        let func_body_suite = match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE
            | Token::DEL
            | Token::PASS
            | Token::BREAK
            | Token::CONTINUE
            | Token::RETURN
            | Token::GLOBAL
            | Token::NEWLINE => self.parse_func_body_suite(),
            _ => errors::unexpected_token(&self),
        };
        Box::new(AST::FuncDef(name, parameters, func_body_suite))
    }

    fn parse_small_stmt(&mut self) -> Box<AST> {
        Box::new(AST::SmallStmt(match self.tokenizer.get_current_token() {
            Token::NOT
            | Token::PLUS
            | Token::MINUS
            | Token::TILDE
            | Token::MUL
            | Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_expr_stmt(),
            Token::DEL => self.parse_del_stmt(),
            Token::PASS => self.parse_pass_stmt(),
            Token::BREAK | Token::CONTINUE | Token::RETURN => self.parse_flow_stmt(),
            Token::GLOBAL => self.parse_global_stmt(),
            _ => errors::unexpected_token(&self),
        }))
    }

    fn parse_or_test(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_exprlist(&mut self) -> Box<AST> {
        unimplemented!();
    }
    fn parse_testlist(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_parameters(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_func_body_suite(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_expr_stmt(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_del_stmt(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_pass_stmt(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_flow_stmt(&mut self) -> Box<AST> {
        unimplemented!();
    }

    fn parse_global_stmt(&mut self) -> Box<AST> {
        unimplemented!();
    }
    fn eat(&mut self, expected: Token) {
        if *self.tokenizer.get_current_token() != expected {
            errors::unexpected_token(&self);
        } else {
            self.tokenizer.next_token();
        }
    }
}
