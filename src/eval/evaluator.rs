use std::collections::HashMap;
use std::iter::Iterator;

use crate::ast::*;
use crate::errors;
use crate::eval::{types::*, utils::*};

// None appears only in local environment and indicates it is global variable
type LocalEnv = Option<HashMap<String, Option<py_val_t>>>;
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
        // native関数の登録
    }

    fn eval_expr(&mut self, expr: &ASTExpr, local_env: &mut LocalEnv) -> py_val_t {
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
                self.call_func(f, &vec![&**lhs, &**rhs], local_env)
            }
            UnaryOp(op, operand) => {
                let f = *(self
                    .global_env
                    .get(unary_operator_to_function_name(&op))
                    .unwrap());
                self.call_func(f, &vec![&**operand], local_env)
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
                let mut current_left = &**left;
                for (op, comparator) in ops.iter().zip(comparators.iter()) {
                    let comp = comparator.clone();
                    let f = *(self
                        .global_env
                        .get(compare_operator_to_function_name(&op))
                        .unwrap());
                    if is_falsy(self.call_func(f, &vec![current_left, &comp], local_env)) {
                        return make_false();
                    }
                    current_left = comparator;
                }
                make_true()
            }
            Call(func, args) => {
                let f = self.eval_expr(func, local_env);
                let mut refs = Vec::new();
                for arg in args.iter() {
                    refs.push(arg);
                }
                self.call_func(f, &refs, local_env)
            }
            Constant(ASTConstant::Int(v)) => make_int(*v),
            Constant(ASTConstant::Float(v)) => make_float(*v),
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
                self.call_func(f, &vec![&*value, &*index], local_env)
                // Subscript(value, ASTSlice::Slice(lower, upper, step))が未実装
                // __getitem__の引数を2~4つにする?
            }
            Name(name) => self.get_env(local_env, &name),
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

    fn eval_stmt(&mut self, stmt: &ASTStmt, local_env: &mut LocalEnv) -> StmtResult {
        use ASTStmt::*;
        match stmt {
            FuncDef(name, arguments, body) => {
                let func = make_py_func(name.clone(), arguments, body);
                self.set_env(local_env, &name, func);
                StmtResult::Next
            }
            Return(value) => StmtResult::Return(if value.is_none() {
                make_none()
            } else {
                self.eval_expr(value.as_ref().unwrap(), local_env)
            }),
            Delete(_targets) => {
                // 変数に適用するとその変数が環境から消える...?
                unimplemented!();
            }
            Assign(targets, value) => {
                let val = self.eval_expr(value, local_env);
                for target in targets {
                    match target {
                        ASTExpr::Name(n) => self.set_env(local_env, &n, val),
                        ASTExpr::Subscript(_val, ASTSlice::Index(_i)) => unimplemented!(),
                        _ => panic!("can't assign"),
                    };
                }
                StmtResult::Next
            }
            For(target, iter, body) => {
                let iterator = self.eval_expr(iter, local_env);
                if !is_char(iterator) && !is_variant(iterator) {
                    panic!("cannot iterate over non iterable");
                }
                // とりあえずアンパック代入はないことにする
                let target = match target {
                    ASTExpr::Name(name) => name,
                    _ => panic!(),
                };
                if is_char(iterator) {
                    self.set_env(local_env, target, iterator);
                    match self.eval_stmt_vec(body, local_env) {
                        StmtResult::Next | StmtResult::Continue | StmtResult::Break => (),
                        StmtResult::Return(val) => return StmtResult::Return(val),
                    };
                    StmtResult::Next
                } else {
                    match unsafe { &*iterator } {
                        py_val::list(elts) | py_val::tuple(elts) => {
                            for elt in elts.iter() {
                                self.set_env(local_env, target, *elt);
                                match self.eval_stmt_vec(body, local_env) {
                                    StmtResult::Next | StmtResult::Continue => (),
                                    StmtResult::Break => break,
                                    StmtResult::Return(val) => return StmtResult::Return(val),
                                };
                            }
                            StmtResult::Next
                        }
                        py_val::string(s) => {
                            for c in s.chars() {
                                self.set_env(local_env, target, make_char(c));
                                match self.eval_stmt_vec(body, local_env) {
                                    StmtResult::Next | StmtResult::Continue => (),
                                    StmtResult::Break => break,
                                    StmtResult::Return(val) => return StmtResult::Return(val),
                                };
                            }
                            StmtResult::Next
                        }
                        _ => panic!("cannot iterate over non iterable"),
                    }
                }
            }
            While(test, body) => {
                while is_truthy(self.eval_expr(test, local_env)) {
                    match self.eval_stmt_vec(body, local_env) {
                        StmtResult::Next | StmtResult::Continue => (),
                        StmtResult::Break => break,
                        StmtResult::Return(val) => return StmtResult::Return(val),
                    };
                }
                StmtResult::Next
            }
            If(test, body, orelse) => {
                if is_truthy(self.eval_expr(test, local_env)) {
                    self.eval_stmt_vec(body, local_env)
                } else {
                    self.eval_stmt_vec(orelse, local_env)
                }
            }
            Global(names) => {
                if let Some(local) = local_env {
                    for name in names.iter() {
                        if local.get(name).is_none() {
                            local.insert(name.to_string(), None);
                        } else {
                            panic!("name {} is assigned before global declaration", name);
                        }
                    }
                }
                // 大域環境では特に何もしない
                StmtResult::Next
            }
            Print(values, nl) => {
                let f = *(self
                    .global_env
                    .get(if *nl { "_print_nl" } else { "_print" })
                    .unwrap());
                let mut refs = Vec::new();
                for value in values.iter() {
                    refs.push(value);
                }
                self.call_func(f, &refs, local_env);
                StmtResult::Next
            }
            Expr(expr) => {
                self.eval_expr(expr, local_env);
                StmtResult::Next
            }
            Pass => StmtResult::Next,
            Break => StmtResult::Break,
            Continue => StmtResult::Continue,
        }
    }

    pub fn eval_file_input(&mut self, ast: &AST) {
        for stmt in ast {
            match self.eval_stmt(stmt, &mut None) {
                StmtResult::Next => (),
                StmtResult::Continue | StmtResult::Break => panic!("outside loop"),
                StmtResult::Return(_) => panic!("outside function."),
            };
        }

        // cleanup global environment
        for (_, v) in self.global_env.iter() {
            drop(*v);
        }
        self.global_env.clear();
    }

    fn eval_stmt_vec(&mut self, body: &Vec<ASTStmt>, local_env: &mut LocalEnv) -> StmtResult {
        for stmt in body {
            match self.eval_stmt(stmt, local_env) {
                StmtResult::Next => (),
                r @ _ => return r,
            };
        }
        StmtResult::Next
    }

    fn call_func(
        &mut self,
        func: py_val_t,
        args: &Vec<&ASTExpr>,
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
                let mut new_local_env: LocalEnv = Some(
                    py_func
                        .args
                        .into_iter()
                        .zip(args.iter())
                        .map(|(v, r)| (v, Some(*r)))
                        .collect(),
                );
                // call
                let res = self.eval_stmt_vec(&py_func.stmt, &mut new_local_env);
                // return from function
                // clean up environment
                for (_, v) in new_local_env.unwrap().into_iter() {
                    if let Some(v) = v {
                        drop(v);
                    }
                }
                self.back_trace.pop();
                match res {
                    StmtResult::Continue | StmtResult::Break => {
                        panic!("continue/break outside loop");
                    }
                    StmtResult::Return(v) => v,
                    StmtResult::Next => make_none(),
                }
            } else {
                panic!("this should not occur...");
            }
        } else {
            errors::type_error();
        }
    }

    fn set_env(&mut self, local_env: &mut LocalEnv, key: &str, value: py_val_t) {
        if let Some(local) = local_env {
            if let Some(v) = local.get(key) {
                if v.is_none() {
                    // `global`
                    if let Some(old) = self.global_env.insert(key.to_string(), value) {
                        drop(old);
                    }
                }
            }
            if let Some(Some(old)) = local.insert(key.to_string(), Some(value)) {
                drop(old);
            }
        } else {
            // Top level
            if let Some(old) = self.global_env.insert(key.to_string(), value) {
                drop(old);
            }
        }
    }

    fn get_env(&self, local_env: &LocalEnv, key: &str) -> py_val_t {
        if let Some(local) = local_env {
            if let Some(val) = local.get(key) {
                match val {
                    Some(v) => *v,                                // local variable
                    None => *(self.global_env.get(key).unwrap()), // explicit global variable
                }
            } else {
                match self.global_env.get(key) {
                    Some(v) => *v, // implicit global variable
                    None => errors::name_error(key),
                }
            }
        } else {
            match self.global_env.get(key) {
                Some(v) => *v, // implicit global variable
                None => errors::name_error(key),
            }
        }
    }
}
