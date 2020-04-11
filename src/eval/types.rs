#![allow(non_camel_case_types)]

use std::collections::{HashMap, HashSet};

use crate::ast::*;

#[derive(Debug, PartialEq, Clone)]
pub struct py_func {
    pub name: String,
    pub args: Vec<String>,
    pub stmt: Vec<ASTStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct py_native_func {
    pub name: String,
    pub arity: usize,
    pub body: fn(Vec<py_val_t>) -> py_val_t, // 呼び出し時に詰めるしかねぇ...
}

// represents python values
// except for:
//  int: ...01
//  char(str with length 1): ...010
//  None: ...0110
// Boxは所有権があるので直接渡すとそれ以降使えない
// ※&BoxはBoxのアドレスなので使おうとするとセグフォ
// →基本(所有権のない)生ポインタ使うしかない?
// ただし、Drop traitなどがないので自分で処理する必要がある
// https://qiita.com/tatsuya6502/items/b9801d92f71e24874c9d#ffi-%E3%81%AB%E3%81%8A%E3%81%91%E3%82%8B%E6%A7%8B%E9%80%A0%E4%BD%93%E3%81%AE%E6%AD%A3%E3%81%97%E3%81%84%E6%B8%A1%E3%81%97%E3%81%8B%E3%81%9F
// 作るときはBox::newしてからBox::into_rawで生ポを作成
// Drop traitはBox::from_rawすればok
#[derive(Debug, PartialEq, Clone)]
pub enum py_val {
    float(f64),
    string(String),
    func(py_func),
    native_func(py_native_func),
    list(Vec<py_val_t>),               // need tuning
    tuple(Vec<py_val_t>),              // need tuning
    dict(HashMap<py_val_t, py_val_t>), // need tuning?
    set(HashSet<py_val_t>),            // need tuning?
}

pub type py_val_t = *mut py_val;

pub enum StmtResult {
    Return(py_val_t),
    Continue,
    Break,
    Next,
}
