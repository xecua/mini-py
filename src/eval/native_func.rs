use crate::errors;
use crate::eval::{types::*, utils::*};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::iter::FromIterator;

// all arguments are assumed to be type checked

pub fn ntv_panic(_: Vec<py_val_t>) -> py_val_t {
    panic!();
}

// cast
pub fn ntv_itof(values: Vec<py_val_t>) -> py_val_t {
    let val = get_int(values[0]);
    make_float(val as f64)
}

pub fn ntv_ftoi(values: Vec<py_val_t>) -> py_val_t {
    let val = get_float(values[0]);
    make_int(val.floor() as isize)
}

pub fn ntv_repr_int(values: Vec<py_val_t>) -> py_val_t {
    let val = get_int(values[0]);
    make_string(format!("{}", val))
}

pub fn ntv_repr_float(values: Vec<py_val_t>) -> py_val_t {
    let val = get_float(values[0]);
    make_string(format!("{}", val))
}

// int. caller has checked values are int(although get_int also checks)
pub fn ntv_add_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs + rhs)
}

pub fn ntv_sub_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs - rhs)
}

pub fn ntv_mul_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs + rhs)
}

pub fn ntv_div_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs / rhs)
}

pub fn ntv_mod_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs % rhs)
}

pub fn ntv_cmp_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(if lhs < rhs {
        -1
    } else if lhs == rhs {
        0
    } else {
        1
    })
}

pub fn ntv_eq_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    if lhs == rhs {
        make_true()
    } else {
        make_false()
    }
}

pub fn ntv_ne_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    if lhs != rhs {
        make_true()
    } else {
        make_false()
    }
}

pub fn ntv_gt_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    if lhs > rhs {
        make_true()
    } else {
        make_false()
    }
}

pub fn ntv_ge_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    if lhs >= rhs {
        make_true()
    } else {
        make_false()
    }
}

pub fn ntv_lt_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    if lhs < rhs {
        make_true()
    } else {
        make_false()
    }
}

pub fn ntv_le_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    if lhs <= rhs {
        make_true()
    } else {
        make_false()
    }
}

pub fn ntv_invert_int(values: Vec<py_val_t>) -> py_val_t {
    let val = get_int(values[0]);
    make_int(!val)
}

pub fn ntv_and_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs & rhs)
}

pub fn ntv_or_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs | rhs)
}

pub fn ntv_xor_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs ^ rhs)
}

pub fn ntv_lshift_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs << rhs)
}

pub fn ntv_rshift_int(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_int(values[0]);
    let rhs = get_int(values[1]);
    make_int(lhs >> rhs)
}

// float. caller has checked values are int(although get_int also checks)
pub fn ntv_add_float(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_float(values[0]);
    let rhs = get_float(values[1]);
    make_float(lhs + rhs)
}

pub fn ntv_sub_float(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_float(values[0]);
    let rhs = get_float(values[1]);
    make_float(lhs - rhs)
}

pub fn ntv_mul_float(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_float(values[0]);
    let rhs = get_float(values[1]);
    make_float(lhs * rhs)
}

pub fn ntv_div_float(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_float(values[0]);
    let rhs = get_float(values[1]);
    make_float(lhs / rhs)
}

pub fn ntv_mod_float(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_float(values[0]);
    let rhs = get_float(values[1]);
    make_float(lhs % rhs)
}

pub fn ntv_cmp_float(values: Vec<py_val_t>) -> py_val_t {
    let lhs = get_float(values[0]);
    let rhs = get_float(values[1]);
    make_int(if lhs < rhs {
        -1
    } else if lhs == rhs {
        0
    } else {
        1
    })
}

pub fn ntv_len_string(values: Vec<py_val_t>) -> py_val_t {
    make_int(get_string(values[0]).len() as isize)
}

pub fn ntv_add_string(values: Vec<py_val_t>) -> py_val_t {
    // includes type check
    if !is_string(values[0]) || !is_string(values[1]) {
        errors::type_error();
    }
    unsafe {
        match &*values[0] {
            py_val::string(l) => match &*values[1] {
                py_val::string(r) => make_string(l.clone() + r),
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn ntv_getitem_string(values: Vec<py_val_t>) -> py_val_t {
    let i = get_int(values[1]) as usize;
    make_string(match get_string(values[0]).get(i..i + 1) {
        Some(s) => s.to_string(),
        None => panic!("IndexError: string index out of range"),
    })
}

pub fn ntv_add_tuple(values: Vec<py_val_t>) -> py_val_t {
    // includes type check
    if !is_tuple(values[0]) || !is_tuple(values[1]) {
        errors::type_error();
    }
    unsafe {
        match &*values[0] {
            py_val::list(l) => match &*values[1] {
                py_val::list(r) => {
                    make_tuple(l.clone().into_iter().chain(r.clone().into_iter()).collect())
                }
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

pub fn ntv_len_tuple(values: Vec<py_val_t>) -> py_val_t {
    unsafe {
        match &*values[0] {
            py_val::tuple(v) => make_int(v.len() as isize),
            _ => errors::type_error(),
        }
    }
}

pub fn ntv_getitem_tuple(values: Vec<py_val_t>) -> py_val_t {
    let i = get_int(values[1]) as usize;
    unsafe {
        match &*values[0] {
            py_val::tuple(v) => match v.get(i) {
                Some(t) => *t,
                _ => panic!("IndexError: tuple index out of range"),
            },
            _ => panic!(),
        }
    }
}

pub fn ntv_add_list(values: Vec<py_val_t>) -> py_val_t {
    // includes type check
    if !is_list(values[0]) || !is_list(values[1]) {
        errors::type_error();
    }
    unsafe {
        match &*values[0] {
            py_val::list(l) => match &*values[1] {
                py_val::list(r) => {
                    make_list(l.clone().into_iter().chain(r.clone().into_iter()).collect())
                }
                _ => errors::type_error(),
            },
            _ => errors::type_error(),
        }
    }
}

// dict, set unimplemented

// print
pub fn ntv_print_string(values: Vec<py_val_t>) -> py_val_t {
    println!("{}", get_string(values[0]));
    make_none()
}

pub fn ntv_range(values: Vec<py_val_t>) -> py_val_t {
    if is_int(values[0]) {
        make_list(
            (0..(get_int(values[0]) as usize))
                .map(|x| make_int(x as isize))
                .collect(),
        )
    } else {
        panic!("TypeError");
    }
}

// type check
pub fn ntv_is_int(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_int(values[0]))
}

pub fn ntv_is_float(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_float(values[0]))
}

pub fn ntv_is_tuple(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_tuple(values[0]))
}

pub fn ntv_is_list(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_list(values[0]))
}

pub fn ntv_is_dict(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_dict(values[0]))
}

pub fn ntv_is_set(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_set(values[0]))
}

pub fn ntv_not(values: Vec<py_val_t>) -> py_val_t {
    make_bool(is_truthy(values[0]))
}
