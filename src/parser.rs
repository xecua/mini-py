//! # parser
//! 構文解析器
// LALRPOP(https://github.com/lalrpop/lalrpop)でも良いかも...?
// follow setの判定は行わないことにする
// (次のトークンでfirst setに含まれないとして弾く)

use crate::ast::*;
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

    pub fn parse(&mut self) -> AST {
        let mut tree: Vec<ASTStmt> = Vec::new();
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
                | Token::PRINT
                | Token::IF
                | Token::WHILE
                | Token::FOR
                | Token::DEF => tree.push(self.parse_stmt()),
                _ => errors::unexpected_token(&self),
            };
        }
        tree
    }

    fn parse_stmt(&mut self) -> ASTStmt {
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
            | Token::GLOBAL
            | Token::PRINT => self.parse_simple_stmt(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_compound_stmt(&mut self) -> ASTStmt {
        match self.tokenizer.get_current_token() {
            Token::IF => self.parse_if_stmt(false),
            Token::WHILE => self.parse_while_stmt(),
            Token::FOR => self.parse_for_stmt(),
            Token::DEF => self.parse_funcdef(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_if_stmt(&mut self, is_orelse: bool) -> ASTStmt {
        // orelseで呼ばれた場合、'if' はない
        if !is_orelse {
            self.eat(&Token::IF);
        }
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
        self.eat(&Token::COLON);
        let body = match self.tokenizer.get_current_token() {
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
            | Token::PRINT
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        let orelse: Vec<ASTStmt> = match self.tokenizer.get_current_token() {
            Token::ELIF => {
                // elifをelse ifに分解する(elifをeatした後、if文として再パース)
                self.eat(&Token::ELIF);
                match self.tokenizer.get_current_token() {
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
                    | Token::FALSE => vec![self.parse_if_stmt(true)],
                    _ => errors::unexpected_token(&self),
                }
            }
            Token::ELSE => {
                // elseのsuite
                self.eat(&Token::ELSE);
                self.eat(&Token::COLON);
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
                    | Token::PRINT
                    | Token::NEWLINE => self.parse_suite(),
                    _ => errors::unexpected_token(&self),
                }
            }
            _ => Vec::new(), // 特に何もしない
        };
        ASTStmt::If(test, body, orelse)
    }

    fn parse_simple_stmt(&mut self) -> ASTStmt {
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
            | Token::GLOBAL
            | Token::PRINT => self.parse_small_stmt(),
            _ => errors::unexpected_token(&self),
        };
        // 最後の改行の省略を許容
        if *self.tokenizer.get_current_token() != Token::EOF {
            self.eat(&Token::NEWLINE);
        }
        small_stmt
    }

    fn parse_test(&mut self) -> ASTExpr {
        let body = match self.tokenizer.get_current_token() {
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
        // ternary operator
        if *self.tokenizer.get_current_token() == Token::IF {
            self.eat(&Token::IF);
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
                | Token::FALSE => self.parse_or_test(),
                _ => errors::unexpected_token(&self),
            };
            self.eat(&Token::ELSE);
            let orelse = match self.tokenizer.get_current_token() {
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
            ASTExpr::IfExp(Box::new(test), Box::new(body), Box::new(orelse))
        } else {
            body
        }
    }

    fn parse_suite(&mut self) -> Vec<ASTStmt> {
        match self.tokenizer.get_current_token() {
            Token::NEWLINE => {
                self.eat(&Token::NEWLINE);
                self.eat(&Token::INDENT);
                let mut stmt: Vec<ASTStmt> = Vec::new();
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
                        | Token::PRINT
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
                self.eat(&Token::DEDENT);
                stmt
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
            | Token::PRINT => vec![self.parse_small_stmt()],
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_while_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::WHILE);
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
        self.eat(&Token::COLON);
        let body = match self.tokenizer.get_current_token() {
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
            | Token::PRINT
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        ASTStmt::While(test, body)
    }

    fn parse_for_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::FOR);
        let target = match self.tokenizer.get_current_token() {
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
            | Token::FALSE => {
                let mut exprlist = self.parse_exprlist();
                if exprlist.len() == 1 {
                    exprlist.swap_remove(0)
                } else {
                    ASTExpr::Tuple(exprlist)
                }
            }
            _ => errors::unexpected_token(&self),
        };
        self.eat(&Token::IN);
        let iter = match self.tokenizer.get_current_token() {
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
        self.eat(&Token::COLON);
        let body = match self.tokenizer.get_current_token() {
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
            | Token::PRINT
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        ASTStmt::For(target, iter, body)
    }

    fn parse_funcdef(&mut self) -> ASTStmt {
        self.eat(&Token::DEF);
        let name = self.eat_id();
        let arguments = match self.tokenizer.get_current_token() {
            Token::LPAREN => self.parse_parameters(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(&Token::COLON);
        let body = match self.tokenizer.get_current_token() {
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
            | Token::PRINT
            | Token::NEWLINE => self.parse_suite(),
            _ => errors::unexpected_token(&self),
        };
        ASTStmt::FuncDef(name, arguments, body)
    }

    fn parse_small_stmt(&mut self) -> ASTStmt {
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
            | Token::FALSE => self.parse_expr_stmt(),
            Token::DEL => self.parse_del_stmt(),
            Token::PASS => self.parse_pass_stmt(),
            Token::BREAK | Token::CONTINUE | Token::RETURN => self.parse_flow_stmt(),
            Token::GLOBAL => self.parse_global_stmt(),
            Token::PRINT => self.parse_print_stmt(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_or_test(&mut self) -> ASTExpr {
        let mut and_test: Vec<ASTExpr> = Vec::new();
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
            self.eat(&Token::OR);
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
        if and_test.len() == 1 {
            and_test.swap_remove(0)
        } else {
            ASTExpr::BoolOp(ASTBoolOp::Or, and_test)
        }
    }

    fn parse_exprlist(&mut self) -> Vec<ASTExpr> {
        let mut res: Vec<ASTExpr> = Vec::new();
        res.push(match self.tokenizer.get_current_token() {
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
            self.eat(&Token::COMMA);
            res.push(match self.tokenizer.get_current_token() {
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
        res
    }

    fn parse_testlist(&mut self) -> ASTExpr {
        let mut res: Vec<ASTExpr> = Vec::new();
        res.push(match self.tokenizer.get_current_token() {
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
            self.eat(&Token::COMMA);
            res.push(match self.tokenizer.get_current_token() {
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
        if res.len() == 1 {
            res.swap_remove(0)
        } else {
            ASTExpr::Tuple(res)
        }
    }

    fn parse_parameters(&mut self) -> ASTArguments {
        self.eat(&Token::LPAREN);
        let typedargslist = match self.tokenizer.get_current_token() {
            Token::ID(_) => self.parse_typedargslist(),
            Token::RPAREN => ASTArguments::new(),
            _ => errors::unexpected_token(&self),
        };
        self.eat(&Token::RPAREN);
        typedargslist
    }

    fn parse_expr_stmt(&mut self) -> ASTStmt {
        let mut testlist_star_expr = match self.tokenizer.get_current_token() {
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
            | Token::FALSE => vec![self.parse_testlist_star_expr()],
            _ => errors::unexpected_token(&self),
        };
        if *self.tokenizer.get_current_token() == Token::EQ {
            self.eat(&Token::EQ);
            let mut tmp = match self.tokenizer.get_current_token() {
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
            while *self.tokenizer.get_current_token() == Token::EQ {
                testlist_star_expr.push(tmp);
                self.eat(&Token::EQ);
                tmp = match self.tokenizer.get_current_token() {
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
            }
            ASTStmt::Assign(testlist_star_expr, tmp)
        } else {
            ASTStmt::Expr(testlist_star_expr.swap_remove(0))
        }
    }

    fn parse_del_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::DEL);
        ASTStmt::Delete(match self.tokenizer.get_current_token() {
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
        })
    }

    fn parse_pass_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::PASS);
        ASTStmt::Pass
    }

    fn parse_flow_stmt(&mut self) -> ASTStmt {
        match self.tokenizer.get_current_token() {
            Token::BREAK => self.parse_break_stmt(),
            Token::CONTINUE => self.parse_continue_stmt(),
            Token::RETURN => self.parse_return_stmt(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_global_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::GLOBAL);
        let mut name: Vec<String> = Vec::new();
        name.push(match self.tokenizer.get_current_token() {
            Token::ID(name) => name.to_owned(),
            _ => errors::unexpected_token(&self),
        });
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(&Token::COMMA);
            name.push(match self.tokenizer.get_current_token() {
                Token::ID(name) => name.to_owned(),
                _ => break,
            });
        }
        ASTStmt::Global(name)
    }

    fn parse_print_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::PRINT);
        let mut values = Vec::new();
        let mut nl = true;
        values.push(match self.tokenizer.get_current_token() {
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
            values.push(match self.tokenizer.get_current_token() {
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
                _ => {
                    nl = false;
                    break;
                }
            });
        }
        ASTStmt::Print(values, nl)
    }

    fn parse_and_test(&mut self) -> ASTExpr {
        let mut not_test: Vec<ASTExpr> = Vec::new();
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
            self.eat(&Token::AND);
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
        if not_test.len() == 1 {
            not_test.swap_remove(0)
        } else {
            ASTExpr::BoolOp(ASTBoolOp::And, not_test)
        }
    }

    fn parse_expr(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
        };
        if *self.tokenizer.get_current_token() == Token::BAR {
            self.eat(&Token::BAR);
            let right = match self.tokenizer.get_current_token() {
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
            };
            ASTExpr::BinOp(Box::new(left), ASTOperator::BitOr, Box::new(right))
        } else {
            left
        }
    }

    fn parse_star_expr(&mut self) -> ASTExpr {
        self.eat(&Token::MUL);
        ASTExpr::Starred(Box::new(match self.tokenizer.get_current_token() {
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

    fn parse_typedargslist(&mut self) -> ASTArguments {
        let mut name: Vec<String> = Vec::new();
        name.push(self.eat_id());
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(&Token::COMMA);
            name.push(match self.tokenizer.get_current_token() {
                Token::ID(_) => self.eat_id(),
                _ => break,
            });
        }
        name
    }

    fn parse_testlist_star_expr(&mut self) -> ASTExpr {
        let mut body = vec![match self.tokenizer.get_current_token() {
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
        }];
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(&Token::COMMA);
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
        if body.len() == 1 {
            body.swap_remove(0)
        } else {
            // ?
            ASTExpr::Tuple(body)
        }
    }

    fn parse_break_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::BREAK);
        ASTStmt::Break
    }

    fn parse_continue_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::CONTINUE);
        ASTStmt::Continue
    }

    fn parse_return_stmt(&mut self) -> ASTStmt {
        self.eat(&Token::RETURN);
        ASTStmt::Return(match self.tokenizer.get_current_token() {
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
        })
    }

    fn parse_xor_expr(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
        };
        if *self.tokenizer.get_current_token() == Token::XOR {
            self.eat(&Token::XOR);
            let right = match self.tokenizer.get_current_token() {
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
            };
            ASTExpr::BinOp(Box::new(left), ASTOperator::BitXor, Box::new(right))
        } else {
            left
        }
    }

    fn parse_not_test(&mut self) -> ASTExpr {
        match self.tokenizer.get_current_token() {
            Token::NOT => {
                self.eat(&Token::NOT);
                ASTExpr::UnaryOp(
                    ASTUnaryOp::Not,
                    match self.tokenizer.get_current_token() {
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
                        | Token::FALSE => Box::new(self.parse_not_test()),
                        _ => errors::unexpected_token(&self),
                    },
                )
            }
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
            | Token::FALSE => self.parse_comparison(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_and_expr(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
        };
        if *self.tokenizer.get_current_token() == Token::AMP {
            self.eat(&Token::AMP);
            let right = match self.tokenizer.get_current_token() {
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
            };
            ASTExpr::BinOp(Box::new(left), ASTOperator::BitAnd, Box::new(right))
        } else {
            left
        }
    }

    fn parse_shift_expr(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
        match self.tokenizer.get_current_token() {
            t @ Token::LSHIFT | t @ Token::RSHIFT => {
                let t = t.clone();
                self.eat(&t);
                let right = match self.tokenizer.get_current_token() {
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
                };
                ASTExpr::BinOp(
                    Box::new(left),
                    if t == Token::LSHIFT {
                        ASTOperator::LShift
                    } else {
                        ASTOperator::RShift
                    },
                    Box::new(right),
                )
            }
            _ => left,
        }
    }

    fn parse_arith_expr(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
            | Token::FALSE => self.parse_term(),
            _ => errors::unexpected_token(&self),
        };
        match self.tokenizer.get_current_token() {
            t @ Token::PLUS | t @ Token::MINUS => {
                let t = t.clone();
                self.eat(&t);
                let right = match self.tokenizer.get_current_token() {
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
                ASTExpr::BinOp(
                    Box::new(left),
                    if t == Token::PLUS {
                        ASTOperator::Add
                    } else {
                        ASTOperator::Sub
                    },
                    Box::new(right),
                )
            }
            _ => left,
        }
    }

    fn parse_comparison(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
        };
        let mut ops: Vec<ASTCmpOp> = Vec::new();
        let mut comparators: Vec<ASTExpr> = Vec::new();
        loop {
            match self.tokenizer.get_current_token() {
                Token::LT
                | Token::GT
                | Token::EQEQ
                | Token::GEQ
                | Token::LEQ
                | Token::NEQ
                | Token::IN
                | Token::NOT
                | Token::IS => {
                    ops.push(self.parse_comp_op());
                    comparators.push(match self.tokenizer.get_current_token() {
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
                        | Token::FALSE => self.parse_comparison(),
                        _ => errors::unexpected_token(&self),
                    });
                }
                _ => break,
            };
        }
        // assert_eq!(ops.len(), comparators.len());
        if ops.len() == 0 {
            left
        } else {
            ASTExpr::Compare(Box::new(left), ops, comparators)
        }
    }

    fn parse_term(&mut self) -> ASTExpr {
        let left = match self.tokenizer.get_current_token() {
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
            | Token::FALSE => self.parse_factor(),
            _ => errors::unexpected_token(&self),
        };
        match self.tokenizer.get_current_token() {
            t @ Token::MUL | t @ Token::DIV | t @ Token::MOD => {
                let t = t.clone();
                self.eat(&t);
                let right = match self.tokenizer.get_current_token() {
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
                    | Token::FALSE => self.parse_term(),
                    _ => errors::unexpected_token(&self),
                };
                ASTExpr::BinOp(
                    Box::new(left),
                    match t {
                        Token::MUL => ASTOperator::Mult,
                        Token::DIV => ASTOperator::Div,
                        Token::MOD => ASTOperator::Mod,
                        _ => errors::unexpected_token(&self),
                    },
                    Box::new(right),
                )
            }
            _ => left,
        }
    }

    fn parse_comp_op(&mut self) -> ASTCmpOp {
        match self.tokenizer.get_current_token() {
            Token::LT => {
                self.eat(&Token::LT);
                ASTCmpOp::Lt
            }
            Token::GT => {
                self.eat(&Token::GT);
                ASTCmpOp::Gt
            }
            Token::LEQ => {
                self.eat(&Token::LEQ);
                ASTCmpOp::LtE
            }
            Token::GEQ => {
                self.eat(&Token::GEQ);
                ASTCmpOp::GtE
            }
            Token::EQEQ => {
                self.eat(&Token::EQEQ);
                ASTCmpOp::Eq
            }
            Token::NEQ => {
                self.eat(&Token::NEQ);
                ASTCmpOp::NotEq
            }
            Token::IN => {
                self.eat(&Token::IN);
                ASTCmpOp::In
            }
            Token::NOT => {
                self.eat(&Token::NOT);
                self.eat(&Token::IN);
                ASTCmpOp::NotIn
            }
            Token::IS => {
                self.eat(&Token::IS);
                match self.tokenizer.get_current_token() {
                    Token::NOT => {
                        self.eat(&Token::NOT);
                        ASTCmpOp::IsNot
                    }
                    _ => ASTCmpOp::Is,
                }
            }
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_factor(&mut self) -> ASTExpr {
        match self.tokenizer.get_current_token() {
            t @ Token::PLUS | t @ Token::MINUS | t @ Token::TILDE => {
                let t = t.clone();
                self.eat(&t);
                let operand = match self.tokenizer.get_current_token() {
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
                    | Token::FALSE => self.parse_factor(),
                    _ => errors::unexpected_token(&self),
                };
                ASTExpr::UnaryOp(
                    match t {
                        Token::PLUS => ASTUnaryOp::UAdd,
                        Token::MINUS => ASTUnaryOp::USub,
                        Token::TILDE => ASTUnaryOp::Invert,
                        _ => errors::unexpected_token(&self),
                    },
                    Box::new(operand),
                )
            }
            Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_atom_expr(),
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_atom_expr(&mut self) -> ASTExpr {
        let mut atom = match self.tokenizer.get_current_token() {
            Token::LPAREN
            | Token::LBRACE
            | Token::LBRACKET
            | Token::ID(_)
            | Token::INT(_)
            | Token::FLOAT(_)
            | Token::STRING(_)
            | Token::NONE
            | Token::TRUE
            | Token::FALSE => self.parse_atom(),
            _ => errors::unexpected_token(&self),
        };
        // trailerはCallかSubscriptで、atomや直前のtrailerの結果を包んでいく形になる
        loop {
            match self.tokenizer.get_current_token() {
                Token::LPAREN => {
                    // function call
                    self.eat(&Token::LPAREN);
                    let args = self.parse_arglist();
                    self.eat(&Token::RPAREN);
                    atom = ASTExpr::Call(Box::new(atom), args);
                }
                Token::LBRACKET => {
                    // list/set/dict subscription
                    self.eat(&Token::LBRACKET);
                    let slice = self.parse_subscript();
                    self.eat(&Token::RBRACKET);
                    atom = ASTExpr::Subscript(Box::new(atom), slice);
                }
                _ => break,
            }
        }
        atom
    }

    fn parse_atom(&mut self) -> ASTExpr {
        match self.tokenizer.get_current_token() {
            Token::LPAREN => {
                self.eat(&Token::LPAREN);
                let res = match self.tokenizer.get_current_token() {
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
                    | Token::FALSE => ASTExpr::Tuple(self.parse_testlist_comp()),
                    _ => ASTExpr::Tuple(Vec::new()),
                };
                self.eat(&Token::RPAREN);
                res
            }
            Token::LBRACKET => {
                self.eat(&Token::LBRACKET);
                let res = match self.tokenizer.get_current_token() {
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
                    | Token::FALSE => ASTExpr::List(self.parse_testlist_comp()),
                    _ => ASTExpr::List(Vec::new()),
                };
                self.eat(&Token::RBRACKET);
                res
            }
            Token::LBRACE => {
                self.eat(&Token::LBRACE);
                let res = self.parse_dictorsetmaker();
                self.eat(&Token::RBRACE);
                res
            }
            Token::ID(_) => {
                let name = self.eat_id();
                ASTExpr::Name(name)
            }
            Token::INT(_) => {
                let num = self.eat_int();
                ASTExpr::Constant(ASTConstant::Int(num))
            }
            Token::FLOAT(_) => {
                let num = self.eat_float();
                ASTExpr::Constant(ASTConstant::Float(num))
            }
            Token::STRING(_) => {
                let val = self.eat_str();
                ASTExpr::Constant(ASTConstant::String(val))
            }
            Token::NONE => {
                self.eat(&Token::NONE);
                ASTExpr::Constant(ASTConstant::None)
            }
            Token::TRUE => {
                self.eat(&Token::TRUE);
                ASTExpr::Constant(ASTConstant::True)
            }
            Token::FALSE => {
                self.eat(&Token::FALSE);
                ASTExpr::Constant(ASTConstant::False)
            }
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_subscript(&mut self) -> ASTSlice {
        match self.tokenizer.get_current_token() {
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
            | Token::FALSE => {
                let lower = Box::new(self.parse_test());
                if *self.tokenizer.get_current_token() == Token::COLON {
                    self.eat(&Token::COLON);
                    let upper = match *self.tokenizer.get_current_token() {
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
                        | Token::FALSE => Some(Box::new(self.parse_test())),
                        _ => None,
                    };
                    let step = match *self.tokenizer.get_current_token() {
                        Token::COLON => {
                            self.eat(&Token::COLON);
                            match self.tokenizer.get_current_token() {
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
                                | Token::FALSE => Some(Box::new(self.parse_test())),
                                _ => None,
                            }
                        }
                        _ => None,
                    };
                    ASTSlice::Slice(Some(lower), upper, step)
                } else {
                    // 単体
                    ASTSlice::Index(lower)
                }
            }
            Token::COLON => {
                self.eat(&Token::COLON);
                let upper = match *self.tokenizer.get_current_token() {
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
                    | Token::FALSE => Some(Box::new(self.parse_test())),
                    _ => None,
                };
                let step = match *self.tokenizer.get_current_token() {
                    Token::COLON => {
                        self.eat(&Token::COLON);
                        match self.tokenizer.get_current_token() {
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
                            | Token::FALSE => Some(Box::new(self.parse_test())),
                            _ => None,
                        }
                    }
                    _ => None,
                };
                ASTSlice::Slice(None, upper, step)
            }
            _ => errors::unexpected_token(&self),
        }
    }

    fn parse_dictorsetmaker(&mut self) -> ASTExpr {
        let first_element = match self.tokenizer.get_current_token() {
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
        };
        match *self.tokenizer.get_current_token() {
            Token::COLON => {
                // dict
                let mut keys = vec![first_element];
                let mut values = Vec::new();
                self.eat(&Token::COLON);
                values.push(match self.tokenizer.get_current_token() {
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
                while *self.tokenizer.get_current_token() == Token::COLON {
                    self.eat(&Token::COLON);
                    keys.push(match self.tokenizer.get_current_token() {
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
                    values.push(match self.tokenizer.get_current_token() {
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
                }
                ASTExpr::Dict(keys, values)
            }
            Token::COMMA => {
                let mut body = vec![first_element];
                // set
                while *self.tokenizer.get_current_token() == Token::COMMA {
                    self.eat(&Token::COMMA);
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
                ASTExpr::Set(body)
            }
            _ => {
                // set with one element
                ASTExpr::Set(vec![first_element])
            }
        }
    }

    fn parse_arglist(&mut self) -> Vec<ASTExpr> {
        let mut arglist = Vec::new();
        arglist.push(match self.tokenizer.get_current_token() {
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
        while *self.tokenizer.get_current_token() == Token::COLON {
            self.eat(&Token::COLON);
            arglist.push(match self.tokenizer.get_current_token() {
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
        arglist
    }
    fn parse_testlist_comp(&mut self) -> Vec<ASTExpr> {
        let mut res = vec![match self.tokenizer.get_current_token() {
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
        }];
        while *self.tokenizer.get_current_token() == Token::COMMA {
            self.eat(&Token::COMMA);
            res.push(match self.tokenizer.get_current_token() {
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
        res
    }

    fn eat(&mut self, expected: &Token) {
        if self.tokenizer.get_current_token() != expected {
            errors::unexpected_token(&self);
        } else {
            self.tokenizer.next_token();
        }
    }

    fn eat_id(&mut self) -> String {
        let name = match self.tokenizer.get_current_token() {
            Token::ID(name) => name.to_owned(),
            _ => errors::unexpected_token(&self),
        };
        self.tokenizer.next_token();
        name
    }

    fn eat_int(&mut self) -> i32 {
        let num = *(match self.tokenizer.get_current_token() {
            Token::INT(num) => num,
            _ => errors::unexpected_token(&self),
        });
        self.tokenizer.next_token();
        num
    }

    fn eat_float(&mut self) -> f32 {
        let num = *(match self.tokenizer.get_current_token() {
            Token::FLOAT(num) => num,
            _ => errors::unexpected_token(&self),
        });
        self.tokenizer.next_token();
        num
    }

    fn eat_str(&mut self) -> String {
        let name = match self.tokenizer.get_current_token() {
            Token::STRING(name) => name.to_owned(),
            _ => errors::unexpected_token(&self),
        };
        self.tokenizer.next_token();
        name
    }

    pub fn get_file_name(&self) -> &String {
        self.tokenizer.get_file_name()
    }

    pub fn get_current_token(&self) -> &Token {
        self.tokenizer.get_current_token()
    }

    pub fn get_current_char_u8(&self) -> Option<u8> {
        self.tokenizer.get_current_char_u8()
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.tokenizer.get_current_char()
    }

    pub fn get_current_line(&self) -> usize {
        self.tokenizer.get_current_line()
    }

    pub fn get_current_column(&self) -> usize {
        self.tokenizer.get_current_column()
    }

    pub fn get_current_line_content(&self) -> &String {
        self.tokenizer.get_current_line_content()
    }
}
