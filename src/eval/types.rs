#![allow(non_camel_case_types)]

use std::boxed::Box;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::mem::transmute;

use crate::ast::*;
use crate::errors;

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

// constructors
pub fn make_none() -> py_val_t {
    unsafe { transmute(0b110isize) }
}

pub fn make_true() -> py_val_t {
    unsafe { transmute(0b01110isize) }
}

pub fn make_false() -> py_val_t {
    unsafe { transmute(0b11110isize) }
}

pub fn make_int(val: isize) -> py_val_t {
    unsafe { transmute((val << 2) + 0b01) }
}

pub fn make_char(val: char) -> py_val_t {
    unsafe { transmute(((val as isize) << 3) + 0b010) }
}

pub fn make_string(val: String) -> py_val_t {
    Box::into_raw(Box::new(py_val::string(val)))
}

pub fn make_float(val: f64) -> py_val_t {
    Box::into_raw(Box::new(py_val::float(val)))
}

pub fn make_list(val: Vec<py_val_t>) -> py_val_t {
    Box::into_raw(Box::new(py_val::list(val)))
}

pub fn make_tuple(val: Vec<py_val_t>) -> py_val_t {
    Box::into_raw(Box::new(py_val::tuple(val)))
}

pub fn make_set(val: Vec<py_val_t>) -> py_val_t {
    Box::into_raw(Box::new(py_val::set(HashSet::from_iter(val))))
}

pub fn make_dict(val: HashMap<py_val_t, py_val_t>) -> py_val_t {
    Box::into_raw(Box::new(py_val::dict(val)))
}

// type checkers
pub fn is_none(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) == 0b110 }
}

pub fn is_true(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) == 0b01110 }
}

// alias
#[inline]
pub fn is_truthy(val: py_val_t) -> bool {
    !is_falsy(val)
}

pub fn is_false(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) == 0b11110 }
}

pub fn is_falsy(val: py_val_t) -> bool {
    is_false(val)
        || is_none(val)
        || (is_variant(val)
            && unsafe {
                match &*val {
                    py_val::string(s) => s.len() == 0,
                    _ => false,
                }
            })
}

pub fn is_int(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) & 1 == 1 }
}

pub fn is_char(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) & 0b010 == 0b010 }
}

pub fn is_string(val: py_val_t) -> bool {
    is_char(val)
        || (is_variant(val)
            && unsafe {
                match *val {
                    py_val::string(_) => true,
                    _ => false,
                }
            })
}

pub fn is_variant(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) & 0b11 == 0b00 }
}

pub fn is_func(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::func(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_func_min(val: py_val_t) -> bool {
    match *val {
        py_val::func(_) => true,
        _ => false,
    }
}

pub fn is_native_func(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::native_func(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_native_func_min(val: py_val_t) -> bool {
    match *val {
        py_val::native_func(_) => true,
        _ => false,
    }
}

pub fn is_callable(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::func(_) | py_val::native_func(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_callable_min(val: py_val_t) -> bool {
    match *val {
        py_val::func(_) | py_val::native_func(_) => true,
        _ => false,
    }
}

// read value
pub fn get_int(val: py_val_t) -> isize {
    unsafe { transmute::<py_val_t, isize>(val) >> 2 }
}

pub fn get_char(val: py_val_t) -> char {
    unsafe { (transmute::<py_val_t, isize>(val) >> 3) as u8 as char }
}

// cast
pub fn itof(val: py_val_t) -> py_val_t {
    // if !is_int(val) {
    //     errors::type_error();
    // }
    let val = get_int(val);
    Box::into_raw(Box::new(py_val::float(val as f64)))
}

pub fn ftoi(val: py_val_t) -> py_val_t {
    // if is_none(val) || is_int(val) || is_char(val) {
    //     errors::type_error();
    // }
    if let py_val::float(f) = unsafe { (*val).clone() } {
        make_int(f.floor() as isize)
    } else {
        errors::type_error();
    }
}

// operation
pub fn add_int(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        let lhs = get_int(lhs);
        let rhs = get_int(rhs);
        transmute(lhs + rhs)
    }
}

pub fn sub_int(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        let lhs = get_int(lhs);
        let rhs = get_int(rhs);
        transmute(lhs - rhs)
    }
}

pub fn mul_int(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        let lhs = get_int(lhs);
        let rhs = get_int(rhs);
        transmute(lhs + rhs)
    }
}

pub fn div_int(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        let lhs = get_int(lhs);
        let rhs = get_int(rhs);
        transmute(lhs / rhs)
    }
}

pub fn mod_int(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        let lhs = get_int(lhs);
        let rhs = get_int(rhs);
        transmute(lhs % rhs)
    }
}

pub fn add_float(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match *lhs {
            py_val::float(l) => match *rhs {
                py_val::float(r) => make_float(l + r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn sub_float(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match *lhs {
            py_val::float(l) => match *rhs {
                py_val::float(r) => make_float(l - r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn mul_float(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match *lhs {
            py_val::float(l) => match *rhs {
                py_val::float(r) => make_float(l * r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn div_float(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match *lhs {
            py_val::float(l) => match *rhs {
                py_val::float(r) => make_float(l / r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn mod_float(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match *lhs {
            py_val::float(l) => match *rhs {
                py_val::float(r) => make_float(l % r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn add_string(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match &*lhs {
            py_val::string(l) => match &*rhs {
                py_val::string(r) => make_string(l.clone() + r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn add_list(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match &*lhs {
            py_val::list(l) => match &*rhs {
                py_val::list(r) => {
                    make_list(l.clone().into_iter().chain(r.clone().into_iter()).collect())
                }
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn add_tuple(lhs: py_val_t, rhs: py_val_t) -> py_val_t {
    unsafe {
        match &*lhs {
            py_val::list(l) => match &*rhs {
                py_val::list(r) => {
                    make_tuple(l.clone().into_iter().chain(r.clone().into_iter()).collect())
                }
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

// destructor
pub fn drop(val: py_val_t) {
    let _ = unsafe { Box::from_raw(val) };
}
