use std::collections::HashMap;
use std::iter::Iterator;

use crate::ast::*;
use crate::errors;
use crate::eval::{types::*, utils::*};

// None appears only in local environment and indicates it is global variable
type LocalEnv = HashMap<String, Option<py_val_t>>;
type GlobalEnv = HashMap<String, py_val_t>;
type StackTrace = Vec<String>;

pub struct Evaluator {
    global_env: GlobalEnv,
    back_trace: StackTrace,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            global_env: GlobalEnv::new(),
            back_trace: StackTrace::new(),
        }
    }

    pub fn eval_expr(&mut self, expr: ASTExpr, local_env: &mut LocalEnv) -> py_val_t {
        use ASTExpr::*;
        match expr {
            BoolOp(ASTBoolOp::And, values) => {
                for val in values {
                    if is_falsy(self.eval_expr(val, local_env)) {
                        return make_false();
                    }
                }
                make_true()
            }
            BoolOp(ASTBoolOp::Or, values) => {
                for val in values {
                    if is_truthy(self.eval_expr(val, local_env)) {
                        return make_true();
                    }
                }
                make_false()
            }
            BinOp(lhs, op, rhs) => {
                let f = *(self.global_env.get(operator_to_function_name(&op)).unwrap());
                self.call_func(f, vec![*lhs, *rhs], local_env)
            }
            UnaryOp(op, operand) => {
                let f = *(self
                    .global_env
                    .get(unary_operator_to_function_name(&op))
                    .unwrap());
                self.call_func(f, vec![*operand], local_env)
            }
            // IfExp
            Dict(keys, values) => make_dict(
                keys.into_iter()
                    .zip(values.into_iter())
                    .map(|(k, v)| (self.eval_expr(k, local_env), self.eval_expr(v, local_env)))
                    .collect(),
            ),
            Set(elts) => make_set(
                elts.into_iter()
                    .map(|el| self.eval_expr(el, local_env))
                    .collect(),
            ),
            Compare(left, ops, comparators) => {
                let mut current_left = *left;
                for (op, comparator) in ops.into_iter().zip(comparators.into_iter()) {
                    let comp = comparator.clone();
                    let f = *(self
                        .global_env
                        .get(compare_operator_to_function_name(&op))
                        .unwrap());
                    if is_falsy(self.call_func(f, vec![current_left, comp], local_env)) {
                        return make_false();
                    }
                    current_left = comparator;
                }
                make_true()
            }
            Call(func, args) => {
                let f = self.eval_expr(*func, local_env);
                self.call_func(f, args, local_env)
            }
            Constant(ASTConstant::Int(v)) => make_int(v),
            Constant(ASTConstant::Float(v)) => make_float(v),
            Constant(ASTConstant::None) => make_none(),
            Constant(ASTConstant::True) => make_true(),
            Constant(ASTConstant::False) => make_false(),
            Constant(ASTConstant::String(s)) => {
                if s.len() == 1 {
                    make_char(s.to_owned().remove(0))
                } else {
                    make_string(s.to_owned())
                }
            }
            Subscript(value, ASTSlice::Index(index)) => {
                let f = *(self.global_env.get("__getitem__").unwrap());
                self.call_func(f, vec![*value, *index], local_env)
                // Subscript(value, ASTSlice::Slice(lower, upper, step))が未実装
                // __getitem__の引数を2~4つにする?
            }
            Name(name) => {
                if let Some(val) = local_env.get(&name) {
                    match val {
                        Some(v) => *v,                                  // local variable
                        None => *(self.global_env.get(&name).unwrap()), // explicit global variable
                    }
                } else {
                    match self.global_env.get(&name) {
                        Some(v) => *v, // implicit global variable
                        None => errors::name_error(&name),
                    }
                }
            }
            List(elts) => make_list(
                elts.into_iter()
                    .map(|el| self.eval_expr(el, local_env))
                    .collect(),
            ),
            Tuple(elts) => make_tuple(
                elts.into_iter()
                    .map(|el| self.eval_expr(el, local_env))
                    .collect(),
            ),
            _ => unimplemented!(),
        }
    }

    pub fn eval_stmt(&mut self, stmt: &ASTStmt, local_env: &mut LocalEnv) -> py_val_t {
        make_none()
    }

    pub fn eval_file_input(ast: &AST) {
        // native関数を帯域環境に登録する

        ()
    }

    pub fn eval_func(&mut self, body: Vec<ASTStmt>, local_env: &mut LocalEnv) -> py_val_t {
        for stmt in body {
            match stmt {
                ASTStmt::Return(Some(expr)) => return self.eval_expr(expr, local_env),
                ASTStmt::Return(None) => return make_none(),
                // break/continue only appear in loop
                ASTStmt::Break | ASTStmt::Continue => errors::invalid_syntax_eval(&self),
                stmt @ _ => self.eval_stmt(&stmt, local_env),
            };
        }
        // function without return returns None
        make_none()
    }

    pub fn call_func(
        &mut self,
        func: py_val_t,
        args: Vec<ASTExpr>,
        local_env: &mut LocalEnv,
    ) -> py_val_t {
        if is_native_func(func) {
            if let py_val::native_func(native_func) = *(unsafe { Box::from_raw(func) }) {
                let args: Vec<py_val_t> = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg, local_env))
                    .collect();
                if native_func.arity != args.len() {
                    errors::type_error();
                }
                self.back_trace.push(native_func.name);
                let res = (native_func.body)(args);
                self.back_trace.pop();
                res
            } else {
                panic!("this should not occur...");
            }
        } else if unsafe { is_func_min(func) } {
            if let py_val::func(py_func) = *(unsafe { Box::from_raw(func) }) {
                let args: Vec<py_val_t> = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg, local_env))
                    .collect();
                if py_func.args.len() != args.len() {
                    errors::type_error();
                }

                // prepare for function call
                self.back_trace.push(py_func.name);
                let mut new_local_env: LocalEnv = py_func
                    .args
                    .into_iter()
                    .zip(args.iter())
                    .map(|(v, r)| (v, Some(*r)))
                    .collect();
                // call
                let res = self.eval_func(py_func.stmt, &mut new_local_env);
                // return from function
                // clean up environment
                for (_, v) in new_local_env.into_iter() {
                    if let Some(v) = v {
                        drop(v);
                    }
                }
                self.back_trace.pop();
                res
            } else {
                panic!("this should not occur...");
            }
        } else {
            errors::type_error();
        }
    }
}
