#![allow(non_camel_case_types)]

use std::collections::HashMap;
use std::mem::transmute;

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
pub enum py_val {
    float(f64),
    string(String),
    func(py_func),
    native_func(py_native_func),
    list(Vec<py_val>),             // need tuning
    tuple(Vec<py_val>),            // need tuning
    dict(HashMap<py_val, py_val>), // need tuning?
}

pub fn make_none() -> *const py_val {
    unsafe { transmute(0b110isize) }
}

pub fn is_none(val: *const py_val) -> bool {
    unsafe { transmute::<*const py_val, isize>(val) & 0b110 == 0b110 }
}

pub fn make_int(val: isize) -> *const py_val {
    unsafe { transmute(val) }
}

pub fn is_int(val: *const py_val) -> bool {
    unsafe { transmute::<*const py_val, isize>(val) & 1 == 1 }
}

pub fn get_int(val: *const py_val) -> isize {
    unsafe { transmute(val) }
}

pub fn add_int(lhs: *const py_val, rhs: *const py_val) -> *const py_val {
    unsafe {
        let lhs = transmute::<*const py_val, isize>(lhs);
        let rhs = transmute::<*const py_val, isize>(rhs);
        transmute(lhs + rhs)
    }
}

pub fn make_char(val: char) -> *const py_val {
    unsafe { transmute(((val as isize) << 3) + 0b010) }
}

pub fn is_char(val: *const py_val) -> bool {
    unsafe { transmute::<*const py_val, isize>(val) & 0b010 == 0b010 }
}

pub fn get_char(val: *const py_val) -> char {
    unsafe { (transmute::<*const py_val, isize>(val) >> 3) as u8 as char }
}
