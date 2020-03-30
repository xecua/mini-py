//! # parser
//! 構文解析器
// LALRPOP(https://github.com/lalrpop/lalrpop)でも良いかも...?
// follow setの判定は行わないことにする
// (次のトークンでfirst setに含まれないとして弾く)

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
        Box::new(AST::FuncDef(name, parameters, suite))
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
        let mut and_test: Vec<Box<AST>> = Vec::new();
        and_test.push(match self.tokenizer.get_current_token() {
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
            | Token::FALSE => self.parse_and_test(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::OR {
            self.eat(Token::OR);
            and_test.push(match self.tokenizer.get_current_token() {
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
                | Token::FALSE => self.parse_and_test(),
                _ => errors::unexpected_token(&self),
            });
        }
        Box::new(AST::OrTest(and_test))
    }

    fn parse_exprlist(&mut self) -> Box<AST> {
        let mut body: Vec<Box<AST>> = Vec::new();
        body.push(match self.tokenizer.get_current_token() {
            Token::MUL => self.parse_star_expr(),
            Token::PLUS
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
            | Token::FALSE => self.parse_expr(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(Token::COMMA);
            body.push(match self.tokenizer.get_current_token() {
                Token::MUL => self.parse_star_expr(),
                Token::PLUS
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
                | Token::FALSE => self.parse_expr(),
                _ => break,
            });
        }
        Box::new(AST::Exprlist(body))
    }

    fn parse_testlist(&mut self) -> Box<AST> {
        let mut test: Vec<Box<AST>> = Vec::new();
        test.push(match self.tokenizer.get_current_token() {
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
        });
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(Token::COMMA);
            test.push(match self.tokenizer.get_current_token() {
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
                _ => break,
            });
        }

        Box::new(AST::Testlist(test))
    }

    fn parse_parameters(&mut self) -> Box<AST> {
        self.eat(Token::LPAREN);
        let typedargslist = match self.tokenizer.get_current_token() {
            Token::ID(_) => Some(self.parse_typedargslist()),
            Token::RPAREN => None,
            _ => errors::unexpected_token(&self),
        };
        self.eat(Token::RPAREN);
        Box::new(AST::Parameters(typedargslist))
    }

    fn parse_expr_stmt(&mut self) -> Box<AST> {
        let testlist_star_expr = match self.tokenizer.get_current_token() {
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
            | Token::FALSE => self.parse_testlist_star_expr(),
            _ => errors::unexpected_token(&self),
        };
        let body = match self.tokenizer.get_current_token() {
            Token::COLON => vec![self.parse_annassign()],
            Token::EQ => {
                let mut body: Vec<Box<AST>> = Vec::new();
                self.eat(Token::EQ);
                body.push(match self.tokenizer.get_current_token() {
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
                    | Token::FALSE => self.parse_testlist_star_expr(),
                    _ => errors::unexpected_token(&self),
                });
                while *self.tokenizer.get_current_token() == Token::EQ {
                    self.eat(Token::EQ);
                    body.push(match self.tokenizer.get_current_token() {
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
                        | Token::FALSE => self.parse_testlist_star_expr(),
                        _ => errors::unexpected_token(&self),
                    });
                }
                body
            }
            _ => errors::unexpected_token(&self),
        };
        Box::new(AST::ExprStmt(testlist_star_expr, body))
    }

    fn parse_del_stmt(&mut self) -> Box<AST> {
        self.eat(Token::DEL);
        Box::new(AST::DelStmt(match self.tokenizer.get_current_token() {
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
        }))
    }

    fn parse_pass_stmt(&mut self) -> Box<AST> {
        self.eat(Token::PASS);
        Box::new(AST::PassStmt)
    }

    fn parse_flow_stmt(&mut self) -> Box<AST> {
        match self.tokenizer.get_current_token() {
            Token::BREAK => self.parse_break_stmt(),
            Token::CONTINUE => self.parse_continue_stmt(),
            Token::RETURN => self.parse_return_stmt(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_global_stmt(&mut self) -> Box<AST> {
        self.eat(Token::GLOBAL);
        let mut name: Vec<String> = Vec::new();
        name.push(match self.tokenizer.get_current_token() {
            Token::ID(name) => name.to_owned(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(Token::COMMA);
            name.push(match self.tokenizer.get_current_token() {
                Token::ID(name) => name.to_owned(),
                _ => break,
            });
        }
        Box::new(AST::GlobalStmt(name))
    }

    fn parse_and_test(&mut self) -> Box<AST> {
        let mut not_test: Vec<Box<AST>> = Vec::new();
        not_test.push(match self.tokenizer.get_current_token() {
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
            | Token::FALSE => self.parse_not_test(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::AND {
            self.eat(Token::AND);
            not_test.push(match self.tokenizer.get_current_token() {
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
                | Token::FALSE => self.parse_not_test(),
                _ => errors::unexpected_token(&self),
            });
        }
        Box::new(AST::AndTest(not_test))
    }

    fn parse_expr(&mut self) -> Box<AST> {
        let mut xor_expr: Vec<Box<AST>> = Vec::new();
        xor_expr.push(match self.tokenizer.get_current_token() {
            Token::PLUS
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
            | Token::FALSE => self.parse_xor_expr(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::BAR {
            self.eat(Token::BAR);
            xor_expr.push(match self.tokenizer.get_current_token() {
                Token::PLUS
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
                | Token::FALSE => self.parse_xor_expr(),
                _ => errors::unexpected_token(&self),
            });
        }
        Box::new(AST::Expr(xor_expr))
    }

    fn parse_star_expr(&mut self) -> Box<AST> {
        self.eat(Token::MUL);
        Box::new(AST::StarExpr(match self.tokenizer.get_current_token() {
            Token::PLUS
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
            | Token::FALSE => self.parse_expr(),
            _ => errors::unexpected_token(&self),
        }))
    }

    fn parse_typedargslist(&mut self) -> Box<AST> {
        let mut name: Vec<String> = Vec::new();
        name.push(match self.tokenizer.get_current_token() {
            Token::ID(name) => name.to_owned(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(Token::COMMA);
            name.push(match self.tokenizer.get_current_token() {
                Token::ID(name) => name.to_owned(),
                _ => break,
            });
        }
        Box::new(AST::TypedArgsList(name))
    }

    fn parse_testlist_star_expr(&mut self) -> Box<AST> {
        let mut body: Vec<Box<AST>> = Vec::new();
        body.push(match self.tokenizer.get_current_token() {
            Token::MUL => self.parse_star_expr(),
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
        });
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(Token::COMMA);
            body.push(match self.tokenizer.get_current_token() {
                Token::MUL => self.parse_star_expr(),
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
                _ => break,
            });
        }
        Box::new(AST::TestlistStarExpr(body))
    }

    fn parse_break_stmt(&mut self) -> Box<AST> {
        self.eat(Token::BREAK);
        Box::new(AST::BreakStmt)
    }

    fn parse_continue_stmt(&mut self) -> Box<AST> {
        self.eat(Token::CONTINUE);
        Box::new(AST::ContinueStmt)
    }

    fn parse_return_stmt(&mut self) -> Box<AST> {
        self.eat(Token::RETURN);
        Box::new(AST::ReturnStmt(match self.tokenizer.get_current_token() {
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
            | Token::FALSE => Some(self.parse_testlist_star_expr()),
            _ => None,
        }))
    }

    fn parse_annassign(&mut self) -> Box<AST> {
        self.eat(Token::COLON);
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
        let testlist_star_expr = if *self.tokenizer.get_current_token() == Token::EQ {
            self.eat(Token::EQ);
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
                | Token::FALSE => Some(self.parse_testlist_star_expr()),
                _ => errors::unexpected_token(&self),
            }
        } else {
            None
        };
        Box::new(AST::Annassign(test, testlist_star_expr))
    }

    fn parse_xor_expr(&mut self) -> Box<AST> {
        let mut and_expr: Vec<Box<AST>> = Vec::new();
        and_expr.push(match self.tokenizer.get_current_token() {
            Token::PLUS
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
            | Token::FALSE => self.parse_and_expr(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::XOR {
            self.eat(Token::XOR);
            and_expr.push(match self.tokenizer.get_current_token() {
                Token::PLUS
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
                | Token::FALSE => self.parse_and_expr(),
                _ => errors::unexpected_token(&self),
            });
        }
        Box::new(AST::XorExpr(and_expr))
    }

    fn parse_not_test(&mut self) -> Box<AST> {
        match self.tokenizer.get_current_token() {
            Token::NOT => {
                self.eat(Token::NOT);
                Box::new(AST::NotTest)
            }
        };( self.eat()
    }

    fn parse_and_expr(&mut self) -> Box<AST> {
        let mut shift_expr: Vec<Box<AST>> = Vec::new();
        shift_expr.push(match self.tokenizer.get_current_token() {
            Token::PLUS
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
            | Token::FALSE => self.parse_shift_expr(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::AMP {
            self.eat(Token::AMP);
            shift_expr.push(match self.tokenizer.get_current_token() {
                Token::PLUS
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
                | Token::FALSE => self.parse_shift_expr(),
                _ => errors::unexpected_token(&self),
            });
        }
        Box::new(AST::AndExpr(shift_expr))
    }

    fn parse_shift_expr(&mut self) -> Box<AST> {
        let arith_expr = match self.tokenizer.get_current_token() {
            Token::PLUS
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
            | Token::FALSE => self.parse_arith_expr(),
            _ => errors::unexpected_token(&self),
        };
        let mut rest: Vec<(Box<AST>, Box<AST>)> = Vec::new();
        loop {
            let op = match *self.tokenizer.get_current_token() {
                Token::LSHIFT => {
                    self.eat(Token::LSHIFT);
                    Box::new(AST::Lshift)
                }
                Token::RSHIFT => {
                    self.eat(Token::RSHIFT);
                    Box::new(AST::Rshift)
                }
                _ => break,
            };
            rest.push((op, match self.tokenizer.get_current_token() {
                Token::PLUS
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
                | Token::FALSE => self.parse_arith_expr(),
                _ => errors::unexpected_token(&self),
            }));
        }
        Box::new(AST::ShiftExpr(arith_expr, rest))
    }

    fn parse_arith_expr(&mut self) -> Box<AST> {
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
