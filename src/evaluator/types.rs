#![allow(non_camel_case_types)]

use std::collections::HashMap;
use std::mem::transmute;
use std::boxed::Box;

use crate::ast::*;

pub struct py_func {
    name: String,
    args: Vec<String>,
    stmt: Vec<ASTStmt>,
}

pub struct py_native_func {
    name: String,
    arity: u32,
    f: fn() -> py_val, // function pointer. https://doc.rust-jp.rs/book/second-edition/ch19-05-advanced-functions-and-closures.html
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
pub enum py_val {
    empty,
    float(f64),
    string(String),
    func(py_func),
    native_func(py_native_func),
    list(Vec<py_val>),             // need tuning
    tuple(Vec<py_val>),            // need tuning
    dict(HashMap<py_val, py_val>), // need tuning?
}

pub type py_val_t = *const py_val;
pub type py_val_t_mut = *mut py_val;

impl py_val {
    pub fn make_none() -> py_val_t {
        unsafe { transmute(0b110isize) }
    }

    pub fn is_none(&self) -> bool {
        unsafe { transmute::<py_val_t, isize>(self) & 0b110 == 0b110 }
    }

    pub fn make_int(val: isize) -> py_val_t {
        unsafe { transmute(val) }
    }

    pub fn is_int(val: py_val_t) -> bool {
        unsafe { transmute::<py_val_t, isize>(val) & 1 == 1 }
    }

    pub fn get_int(val: py_val_t) -> isize {
        unsafe { transmute(val) }
    }

    pub fn add_int(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
        unsafe {
            let lhs = transmute::<py_val_t, isize>(lhs);
            let rhs = transmute::<py_val_t, isize>(rhs);
            transmute(lhs + rhs)
        }
    }

    pub fn make_char(val: char) -> py_val_t {
        unsafe { transmute(((val as isize) << 3) + 0b010) }
    }

    pub fn is_char(val: py_val_t) -> bool {
        unsafe { transmute::<py_val_t, isize>(val) & 0b010 == 0b010 }
    }

    pub fn get_char(val: py_val_t) -> char {
        unsafe { (transmute::<py_val_t, isize>(val) >> 3) as u8 as char }
    }

    pub fn make_string(val: String) -> py_val_t {
        Box::into_raw(Box::new(py_val::string(val)))
    }

    pub fn make_float(val: f64) -> py_val_t {
        Box::into_raw(Box::new(py_val::float(val)))
    }

    pub fn drop(val: py_val_t_mut) {
        unsafe { Box::from_raw(val) };
    }
}
