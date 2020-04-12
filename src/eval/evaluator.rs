use std::collections::HashMap;
use std::iter::Iterator;
use std::rc::Rc;

use crate::ast::*;
use crate::errors;
use crate::eval::{native_func::*, types::*, utils::*};
use crate::parser::Parser;

// None appears only in local environment and indicates it is global variable
type LocalEnv = Option<HashMap<String, Option<py_val_t>>>;
type GlobalEnv = HashMap<String, py_val_t>;
type StackTrace = Vec<String>;

pub struct Evaluator {
    global_env: GlobalEnv,
    back_trace: StackTrace,
}

macro_rules! insert_native_functions {
    ($env: expr, [$(($name: ident, $arity: expr)),*]) => {
        $(
            $env.insert(stringify!($name).to_string(),
            Rc::new(
                py_val::native_func(py_native_func {
                    name: stringify!($name).to_string(),
                    arity: $arity,
                    body: $name as fn(Vec<py_val_t>) -> py_val_t
                })
            ));
        )*
    }
}

impl Evaluator {
    pub fn new() -> Evaluator {
        // native関数の登録
        let mut global_env: GlobalEnv = GlobalEnv::new();
        #[rustfmt::skip]
        insert_native_functions!(global_env, [
            (ntv_itof, 1), (ntv_ftoi, 1), (ntv_repr_int, 1), (ntv_repr_float, 1),
            (ntv_add_int, 2), (ntv_sub_int, 2), (ntv_mul_int, 2), (ntv_div_int, 2), (ntv_mod_int, 2),
            (ntv_cmp_int, 2), (ntv_eq_int, 2), (ntv_ne_int, 2), (ntv_gt_int, 2), (ntv_ge_int, 2), (ntv_lt_int, 2), (ntv_le_int, 2),
            (ntv_invert_int, 1), (ntv_and_int, 2), (ntv_or_int, 2), (ntv_xor_int, 2), (ntv_lshift_int, 2), (ntv_rshift_int, 2),
            (ntv_add_float, 2), (ntv_sub_float, 2), (ntv_mul_float, 2), (ntv_div_float, 2), (ntv_mod_float, 2), (ntv_cmp_float, 2),
            (ntv_len_string, 1), (ntv_add_string, 2), (ntv_getitem_string, 2),
            (ntv_add_tuple, 2), (ntv_len_tuple, 2), (ntv_getitem_tuple, 2),
            (ntv_add_list, 2),
            (ntv_print_string, 1), (ntv_range, 1), (ntv_panic, 0), (ntv_not, 1),
            (ntv_is_int, 1), (ntv_is_float, 1), (ntv_is_tuple, 1), (ntv_is_list, 1), (ntv_is_dict, 1), (ntv_is_set, 1)
        ]);
        Evaluator {
            global_env: global_env,
            back_trace: StackTrace::new(),
        }
    }

    pub fn eval_ast(&mut self, ast: &AST) {
        for stmt in ast {
            match self.eval_stmt(stmt, &mut None) {
                StmtResult::Next => (),
                StmtResult::Continue | StmtResult::Break => panic!("outside loop"),
                StmtResult::Return(_) => panic!("outside function."),
            };
        }
    }

    pub fn eval_file_input(&mut self, file_name: &str) -> std::io::Result<()> {
        self.eval_ast(&Parser::new(&format!("{}/src/std/init.py", env!("PWD")))?.parse());
        self.eval_ast(&Parser::new(file_name)?.parse());
        Ok(())
    }

    fn eval_expr(&mut self, expr: &ASTExpr, local_env: &mut LocalEnv) -> py_val_t {
        use ASTExpr::*;
        match expr {
            BoolOp(ASTBoolOp::And, values) => {
                for val in values {
                    if self.eval_expr(val, local_env).is_false() {
                        return py_val::new(py_val::False);
                    }
                }
                py_val::new(py_val::True)
            }
            BoolOp(ASTBoolOp::Or, values) => {
                for val in values {
                    if self.eval_expr(val, local_env).is_true() {
                        return py_val::new(py_val::True);
                    }
                }
                py_val::new(py_val::False)
            }
            BinOp(lhs, op, rhs) => {
                let f = self
                    .global_env
                    .get(operator_to_function_name(&op))
                    .unwrap()
                    .clone();
                self.call_func(f, &vec![lhs, rhs], local_env)
            }
            UnaryOp(op, operand) => {
                let f = self
                    .global_env
                    .get(unary_operator_to_function_name(&op))
                    .unwrap()
                    .clone();
                self.call_func(f, &vec![operand], local_env)
            }
            // IfExp
            Dict(keys, values) => py_val::new(py_val::dict(
                keys.into_iter()
                    .zip(values.into_iter())
                    .map(|(k, v)| (self.eval_expr(k, local_env), self.eval_expr(v, local_env)))
                    .collect(),
            )),
            Set(elts) => py_val::new(py_val::set(
                elts.into_iter()
                    .map(|el| self.eval_expr(el, local_env))
                    .collect(),
            )),
            Compare(left, ops, comparators) => {
                let mut current_left = &**left;
                for (op, comparator) in ops.iter().zip(comparators.iter()) {
                    let f = self
                        .global_env
                        .get(compare_operator_to_function_name(&op))
                        .unwrap()
                        .clone();
                    if self
                        .call_func(f, &vec![current_left, comparator], local_env)
                        .is_false()
                    {
                        return py_val::new(py_val::False);
                    }
                    current_left = comparator;
                }
                py_val::new(py_val::True)
            }
            Call(func, args) => {
                // func will be moved
                let f = self.eval_expr(func, local_env);
                let refs = args.iter().collect();
                self.call_func(f, &refs, local_env)
            }
            Constant(ASTConstant::Int(v)) => py_val::new(py_val::int(*v)),
            // v will be moved
            Constant(ASTConstant::Float(v)) => py_val::new(py_val::float(*v)),
            Constant(ASTConstant::None) => py_val::new(py_val::None),
            Constant(ASTConstant::True) => py_val::new(py_val::True),
            Constant(ASTConstant::False) => py_val::new(py_val::False),
            // s will be moved
            Constant(ASTConstant::String(s)) => py_val::new(py_val::string(s.clone())),
            Subscript(value, ASTSlice::Index(index)) => {
                let f = self.global_env.get("__getitem__").unwrap().clone();
                self.call_func(f, &vec![value, index], local_env)
                // Subscript(value, ASTSlice::Slice(lower, upper, step))が未実装
                // __getitem__の引数を2~4つにする?
            }
            Name(name) => self.get_env(local_env, &name),
            List(elts) => py_val::new(py_val::list(
                elts.into_iter()
                    .map(|el| self.eval_expr(el, local_env))
                    .collect(),
            )),
            Tuple(elts) => py_val::new(py_val::tuple(
                elts.into_iter()
                    .map(|el| self.eval_expr(el, local_env))
                    .collect(),
            )),
            _ => unimplemented!(),
        }
    }

    fn eval_stmt(&mut self, stmt: &ASTStmt, local_env: &mut LocalEnv) -> StmtResult {
        use ASTStmt::*;
        match stmt {
            FuncDef(name, arguments, body) => {
                // each values will be moved
                let func = py_val::new(py_val::func(py_func {
                    name: name.clone(),
                    args: arguments.clone(),
                    stmt: body.clone(),
                }));
                self.set_env(local_env, &name, func);
                StmtResult::Next
            }
            Return(value) => StmtResult::Return(if value.is_none() {
                py_val::new(py_val::None)
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
                        ASTExpr::Name(n) => self.set_env(local_env, &n, val.clone()),
                        ASTExpr::Subscript(_val, ASTSlice::Index(_i)) => unimplemented!(),
                        _ => panic!("can't assign"),
                    };
                }
                StmtResult::Next
            }
            For(target, iter, body) => {
                let iterator = self.eval_expr(iter, local_env);
                let body_ref = body.iter().collect();
                // とりあえずアンパック代入はないことにする
                let target = match target {
                    ASTExpr::Name(name) => name,
                    _ => panic!(),
                };
                match *iterator {
                    py_val::list(ref elts) | py_val::tuple(ref elts) => {
                        for elt in elts.iter() {
                            self.set_env(local_env, &target, elt.clone());
                            match self.eval_stmt_vec(&body_ref, local_env) {
                                StmtResult::Next | StmtResult::Continue => (),
                                StmtResult::Break => break,
                                StmtResult::Return(val) => return StmtResult::Return(val),
                            };
                        }
                        StmtResult::Next
                    }
                    py_val::string(ref s) => {
                        for c in s.chars() {
                            self.set_env(
                                local_env,
                                &target,
                                py_val::new(py_val::string(c.to_string())),
                            );
                            match self.eval_stmt_vec(&body_ref, local_env) {
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
            While(test, body) => {
                let body_ref = body.iter().collect();
                while self.eval_expr(test, local_env).is_true() {
                    match self.eval_stmt_vec(&body_ref, local_env) {
                        StmtResult::Next | StmtResult::Continue => (),
                        StmtResult::Break => break,
                        StmtResult::Return(val) => return StmtResult::Return(val),
                    };
                }
                StmtResult::Next
            }
            If(test, body, orelse) => {
                if self.eval_expr(test, local_env).is_true() {
                    let body_ref = body.iter().collect();
                    self.eval_stmt_vec(&body_ref, local_env)
                } else {
                    let orelse_ref = orelse.iter().collect();
                    self.eval_stmt_vec(&orelse_ref, local_env)
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
                let f = self
                    .global_env
                    .get(if *nl { "__print_nl__" } else { "__print__" })
                    .unwrap()
                    .clone();
                let refs = values.iter().collect();
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

    fn eval_stmt_vec(&mut self, body: &Vec<&ASTStmt>, local_env: &mut LocalEnv) -> StmtResult {
        for stmt in body {
            match self.eval_stmt(stmt.clone(), local_env) {
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
        match *func {
            py_val::native_func(ref native_func) => {
                let args: Vec<py_val_t> = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg, local_env))
                    .collect();
                if native_func.arity != args.len() {
                    errors::type_error();
                }
                self.back_trace.push(native_func.name.clone());
                let res = (native_func.body)(args);
                self.back_trace.pop();
                res
            }
            py_val::func(ref py_func) => {
                let args: Vec<py_val_t> = args
                    .into_iter()
                    .map(|arg| self.eval_expr(arg, local_env))
                    .collect();
                if py_func.args.len() != args.len() {
                    errors::type_error();
                }

                // prepare for function call
                self.back_trace.push(py_func.name.clone());
                let mut new_local_env: LocalEnv = Some(
                    py_func
                        .args
                        .iter()
                        .zip(args.iter())
                        .map(|(v, r)| (v.clone(), Some(r.clone())))
                        .collect(),
                );
                // call
                let refs = py_func.stmt.iter().collect();
                let res = self.eval_stmt_vec(&refs, &mut new_local_env);
                // return from function
                self.back_trace.pop();

                match res {
                    StmtResult::Continue | StmtResult::Break => {
                        panic!("continue/break outside loop");
                    }
                    StmtResult::Return(v) => v,
                    StmtResult::Next => py_val::new(py_val::None),
                }
            }
            _ => panic!("this should not occur..."),
        }
    }

    fn set_env(&mut self, local_env: &mut LocalEnv, key: &str, value: py_val_t) {
        if let Some(local) = local_env {
            if let Some(v) = local.get(key) {
                if v.is_none() {
                    // `global`
                    self.global_env.insert(key.to_string(), value);
                    return;
                }
            }
            local.insert(key.to_string(), Some(value));
        } else {
            // Top level
            self.global_env.insert(key.to_string(), value);
        }
    }

    fn get_env(&self, local_env: &LocalEnv, key: &str) -> py_val_t {
        if let Some(local) = local_env {
            if let Some(val) = local.get(key) {
                match val {
                    Some(v) => v.clone(),                              // local variable
                    None => self.global_env.get(key).unwrap().clone(), // explicit global variable
                }
            } else {
                match self.global_env.get(key) {
                    Some(v) => v.clone(), // implicit global variable
                    None => errors::name_error(key),
                }
            }
        } else {
            match self.global_env.get(key) {
                Some(v) => v.clone(), // implicit global variable
                None => errors::name_error(key),
            }
        }
    }
}
