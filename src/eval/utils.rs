use crate::ast::*;
use crate::eval::types::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::mem::transmute;

pub fn operator_to_function_name(op: &ASTOperator) -> &'static str {
    use ASTOperator::*;
    match op {
        Add => "__add__",
        Sub => "__sub__",
        Mul => "__mul__",
        Div => "__div__",
        Mod => "__mod__",
        LShift => "__lshift__",
        RShift => "__rshift__",
        BitOr => "__or__",
        BitXor => "__xor__",
        BitAnd => "__and__",
    }
}

pub fn unary_operator_to_function_name(op: &ASTUnaryOp) -> &'static str {
    use ASTUnaryOp::*;
    match op {
        Invert => "__invert__",
        Not => "__not__",
        UAdd => "__plus__",
        USub => "__minus__",
    }
}

pub fn compare_operator_to_function_name(op: &ASTCmpOp) -> &'static str {
    use ASTCmpOp::*;
    match op {
        ASTCmpOp::Eq => "__eq__",
        NotEq => "__neq__",
        Lt => "__lt__",
        LtE => "__le__",
        Gt => "__gt__",
        GtE => "__ge__",
        Is => "__is__",
        IsNot => "__is_not__", // 余力があればnot(A is B)にしたい
        In => "__in__",
        NotIn => "__not_in__",
    }
}

// constructors
pub fn make_none() -> py_val_t {
    unsafe { transmute(0b110isize) }
}

pub fn make_true() -> py_val_t {
    make_int(1)
}

pub fn make_false() -> py_val_t {
    make_int(0)
}

pub fn make_bool(val: bool) -> py_val_t {
    if val {
        make_true()
    } else {
        make_false()
    }
}
pub fn make_int(val: isize) -> py_val_t {
    unsafe { transmute((val << 2) + 0b01) }
}

pub fn make_char(val: char) -> py_val_t {
    unsafe { transmute(((val as isize) << 3) + 0b010) }
}

pub fn make_string(mut val: String) -> py_val_t {
    if val.len() == 1 {
        make_char(val.remove(0))
    } else {
        Box::into_raw(Box::new(py_val::string(val)))
    }
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

pub fn make_py_func(name: String, arguments: &Vec<String>, body: &Vec<ASTStmt>) -> py_val_t {
    Box::into_raw(Box::new(py_val::func(py_func {
        name: name,
        args: arguments.clone(),
        stmt: body.clone(),
    })))
}

pub fn make_native_func(
    name: String,
    arity: usize,
    body: fn(Vec<py_val_t>) -> py_val_t,
) -> py_val_t {
    Box::into_raw(Box::new(py_val::native_func(py_native_func {
        name: name,
        arity: arity,
        body: body,
    })))
}

// type checkers
pub fn is_none(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) == 0b110 }
}

pub fn is_true(val: py_val_t) -> bool {
    is_int(val) && get_int(val) == 1
}

// alias
#[inline]
pub fn is_truthy(val: py_val_t) -> bool {
    !is_falsy(val)
}

pub fn is_false(val: py_val_t) -> bool {
    is_int(val) && get_int(val) == 0
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

#[inline]
pub fn is_variant(val: py_val_t) -> bool {
    unsafe { transmute::<py_val_t, isize>(val) & 0b11 == 0b00 }
}

// alias
#[inline]
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

pub unsafe fn is_string_min(val: py_val_t) -> bool {
    match *val {
        py_val::string(_) => true,
        _ => false,
    }
}

pub fn is_float(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::float(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_float_min(val: py_val_t) -> bool {
    match *val {
        py_val::float(_) => true,
        _ => false,
    }
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

pub fn is_list(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::list(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_list_min(val: py_val_t) -> bool {
    match *val {
        py_val::list(_) => true,
        _ => false,
    }
}

pub fn is_tuple(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::tuple(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_tuple_min(val: py_val_t) -> bool {
    match *val {
        py_val::tuple(_) => true,
        _ => false,
    }
}

pub fn is_dict(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::dict(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_dict_min(val: py_val_t) -> bool {
    match *val {
        py_val::dict(_) => true,
        _ => false,
    }
}

pub fn is_set(val: py_val_t) -> bool {
    is_variant(val)
        && unsafe {
            match *val {
                py_val::set(_) => true,
                _ => false,
            }
        }
}

pub unsafe fn is_set_min(val: py_val_t) -> bool {
    match *val {
        py_val::set(_) => true,
        _ => false,
    }
}

// read value(with assertion)
pub fn get_int(val: py_val_t) -> isize {
    if is_int(val) {
        unsafe { transmute::<py_val_t, isize>(val) >> 2 }
    } else {
        panic!();
    }
}

pub fn get_char(val: py_val_t) -> char {
    if is_char(val) {
        unsafe { (transmute::<py_val_t, isize>(val) >> 3) as u8 as char }
    } else {
        panic!();
    }
}

pub fn get_float(val: py_val_t) -> f64 {
    // should be is_float?
    if is_variant(val) {
        unsafe {
            match *val {
                py_val::float(v) => v,
                _ => panic!(),
            }
        }
    } else {
        panic!();
    }
}

pub fn get_string(val: py_val_t) -> String {
    if is_char(val) {
        String::from_utf8(vec![get_char(val) as u8]).unwrap()
    } else if is_string(val) {
        unsafe {
            match &*val {
                py_val::string(s) => s.clone(),
                _ => panic!(),
            }
        }
    } else {
        panic!();
    }
}

// destructor
pub fn drop(val: py_val_t) {
    let _ = unsafe { Box::from_raw(val) };
}
