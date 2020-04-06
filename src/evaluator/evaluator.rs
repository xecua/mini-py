use std::collections::HashMap;

use crate::ast::*;
use crate::errors;
use crate::evaluator::types::*;

// None appears only in local environment and indicates it is global variable
type Env = HashMap<String, Option<py_val_t>>;
type StackTrace = ();

struct Evaluator {
    local_env: Env,
    global_env: Env,
    back_trace: StackTrace,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            local_env: HashMap::new(),
            global_env: HashMap::new(),
            back_trace: (),
        }
    }

    pub fn eval_expr(&mut self, expr: &ASTExpr) -> py_val_t {
        match expr {
            ASTExpr::Name(name) => {
                if let Some(val) = self.local_env.get(name) {
                    match val {
                        Some(v) => *v,                                       // local variable
                        None => self.global_env.get(name).unwrap().unwrap(), // explicit global variable
                    }
                } else {
                    match self.global_env.get(name) {
                        Some(v) => v.unwrap(), // implicit global variable
                        None => errors::name_error(name),
                    }
                }
            }
            ASTExpr::Constant(ASTConstant::Int(v)) => py_val::make_int(*v),
            ASTExpr::Constant(ASTConstant::Float(v)) => py_val::make_float(*v),
            ASTExpr::Constant(ASTConstant::None) => py_val::make_none(),
            ASTExpr::Constant(ASTConstant::String(s)) => {
                if s.len() == 1 {
                    py_val::make_char(s.to_owned().remove(0))
                } else {
                    py_val::make_string(s.to_owned())
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn eval_stmt(&mut self, stmt: &ASTStmt) -> py_val_t {
        py_val::make_none()
    }

    pub fn eval_file_input(ast: &AST) {
        ()
    }
}
