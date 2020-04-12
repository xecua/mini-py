#![allow(non_camel_case_types)]
#![allow(illegal_floating_point_literal_pattern)]

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::ast::*;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub struct py_func {
    pub name: String,
    pub args: Vec<String>,
    pub stmt: Vec<ASTStmt>,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub struct py_native_func {
    pub name: String,
    pub arity: usize,
    pub body: fn(Vec<py_val_t>) -> py_val_t,
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
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub enum py_val {
    int(i64),
    True,
    False,
    None,
    float(OrderedFloat<f64>),
    string(String),
    func(py_func),
    native_func(py_native_func),
    list(Vec<py_val_t>),                // need tuning
    tuple(Vec<py_val_t>),               // need tuning
    dict(BTreeMap<py_val_t, py_val_t>), // need tuning?
    set(BTreeSet<py_val_t>),            // need tuning?
}

pub type py_val_t = Rc<py_val>;

pub enum StmtResult {
    Return(py_val_t),
    Continue,
    Break,
    Next,
}

impl py_val {
    pub fn new(v: py_val) -> py_val_t {
        Rc::new(v)
    }

    pub fn is_none(&self) -> bool {
        match self {
            py_val::None => true,
            _ => false,
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            py_val::int(0)
            | py_val::float(OrderedFloat(0.0))
            | py_val::True
            | py_val::None => false,
            _ => true,
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            py_val::int(0)
            | py_val::float(OrderedFloat(0.0))
            | py_val::False
            | py_val::None => true,
            py_val::string(ref s) if s.len() == 0 => true,
            _ => false,
        }
    }
}
