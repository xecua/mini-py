// use crate::errors;
use crate::eval::types::*;
use ordered_float::OrderedFloat;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet};
#[allow(unused_imports)]
use std::iter::FromIterator;

// all arguments are assumed to be type checked

pub fn ntv_panic(_: Vec<py_val_t>) -> py_val_t {
    panic!();
}

// cast
pub fn ntv_itof(values: Vec<py_val_t>) -> py_val_t {
    match *values[0] {
        py_val::int(i) => py_val::new(py_val::float(OrderedFloat(i as f64))),
        _ => panic!(),
    }
}

pub fn ntv_ftoi(values: Vec<py_val_t>) -> py_val_t {
    match *values[0] {
        py_val::float(f) => py_val::new(py_val::int(f.floor() as i64)),
        _ => panic!(),
    }
}

pub fn ntv_repr_int(values: Vec<py_val_t>) -> py_val_t {
    match *values[0] {
        py_val::int(i) => py_val::new(py_val::string(format!("{}", i))),
        _ => panic!(),
    }
}

pub fn ntv_repr_float(values: Vec<py_val_t>) -> py_val_t {
    match *values[0] {
        py_val::float(f) => py_val::new(py_val::string(format!("{}", f))),
        _ => panic!(),
    }
}

// int. caller has checked values are int(although get_int also checks)
pub fn ntv_add_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i + j));
        }
    }
    panic!();
}

pub fn ntv_sub_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i - j));
        }
    }
    panic!();
}

pub fn ntv_mul_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i * j));
        }
    }
    panic!();
}

pub fn ntv_div_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i / j));
        }
    }
    panic!();
}

pub fn ntv_mod_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i % j));
        }
    }
    panic!();
}

pub fn ntv_cmp_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(if i < j {
                -1
            } else if i == j {
                0
            } else {
                1
            }));
        }
    }
    panic!();
}

pub fn ntv_eq_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(if i == j { py_val::True } else { py_val::False });
        }
    }
    panic!();
}

pub fn ntv_ne_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(if i != j { py_val::True } else { py_val::False });
        }
    }
    panic!();
}

pub fn ntv_gt_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(if i > j { py_val::True } else { py_val::False });
        }
    }
    panic!();
}

pub fn ntv_ge_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(if i >= j { py_val::True } else { py_val::False });
        }
    }
    panic!();
}

pub fn ntv_lt_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(if i < j { py_val::True } else { py_val::False });
        }
    }
    panic!();
}

pub fn ntv_le_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(if i <= j { py_val::True } else { py_val::False });
        }
    }
    panic!();
}

pub fn ntv_invert_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        return py_val::new(py_val::int(!i));
    }
    panic!();
}

pub fn ntv_and_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i & j));
        }
    }
    panic!();
}

pub fn ntv_or_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i | j));
        }
    }
    panic!();
}

pub fn ntv_xor_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i ^ j));
        }
    }
    panic!();
}

pub fn ntv_lshift_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i << j));
        }
    }
    panic!();
}

pub fn ntv_rshift_int(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        if let py_val::int(j) = *values[1] {
            return py_val::new(py_val::int(i >> j));
        }
    }
    panic!();
}

// float. caller has checked values are int(although get_int also checks)
pub fn ntv_add_float(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::float(i) = *values[0] {
        if let py_val::float(j) = *values[1] {
            return py_val::new(py_val::float(OrderedFloat(i.into_inner() + j.into_inner())));
        }
    }
    panic!();
}

pub fn ntv_sub_float(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::float(i) = *values[0] {
        if let py_val::float(j) = *values[1] {
            return py_val::new(py_val::float(OrderedFloat(i.into_inner() - j.into_inner())));
        }
    }
    panic!();
}

pub fn ntv_mul_float(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::float(i) = *values[0] {
        if let py_val::float(j) = *values[1] {
            return py_val::new(py_val::float(OrderedFloat(i.into_inner() * j.into_inner())));
        }
    }
    panic!();
}

pub fn ntv_div_float(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::float(i) = *values[0] {
        if let py_val::float(j) = *values[1] {
            return py_val::new(py_val::float(OrderedFloat(i.into_inner() / j.into_inner())));
        }
    }
    panic!();
}

pub fn ntv_mod_float(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::float(i) = *values[0] {
        if let py_val::float(j) = *values[1] {
            return py_val::new(py_val::float(OrderedFloat(i.into_inner() % j.into_inner())));
        }
    }
    panic!();
}

pub fn ntv_cmp_float(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::float(i) = *values[0] {
        if let py_val::float(j) = *values[1] {
            return py_val::new(py_val::int(if i < j {
                -1
            } else if i == j {
                0
            } else {
                1
            }));
        }
    }
    panic!();
}

pub fn ntv_len_string(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::string(ref s) = *values[0] {
        return py_val::new(py_val::int(s.len() as i64));
    }
    panic!();
}

pub fn ntv_add_string(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::string(ref s) = *values[0] {
        if let py_val::string(ref t) = *values[1] {
            return py_val::new(py_val::string(s.clone() + t));
        }
    }
    panic!();
}

pub fn ntv_getitem_string(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::string(ref s) = *values[0] {
        if let py_val::int(i) = *values[1] {
            if let Some(c) = s.get((i as usize)..(i as usize) + 1) {
                return py_val::new(py_val::string(c.to_string()));
            }
        }
    }
    panic!();
}

pub fn ntv_add_tuple(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::tuple(ref t) = *values[0] {
        if let py_val::tuple(ref u) = *values[1] {
            let mut res = t.clone();
            res.extend(u.clone());
            return py_val::new(py_val::tuple(res));
        }
    }
    panic!();
}

pub fn ntv_len_tuple(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::tuple(ref t) = *values[0] {
        return py_val::new(py_val::int(t.len() as i64));
    }
    panic!();
}

pub fn ntv_getitem_tuple(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::tuple(ref t) = *values[0] {
        if let py_val::int(i) = *values[1] {
            if let Some(v) = t.get(i as usize) {
                return v.clone();
            }
        }
    }
    panic!();
}

pub fn ntv_add_list(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::list(ref t) = *values[0] {
        if let py_val::list(ref u) = *values[1] {
            let mut res = t.clone();
            res.extend(u.clone());
            return py_val::new(py_val::tuple(res));
        }
    }
    panic!();
}

// dict, set unimplemented

// print
pub fn ntv_print_string(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::string(ref s) = *values[0] {
        print!("{}", s);
    } else {
        panic!();
    }
    py_val::new(py_val::None)
}

pub fn ntv_print_string_nl(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::string(ref s) = *values[0] {
        println!("{}", s);
    } else {
        panic!();
    }
    py_val::new(py_val::None)
}

pub fn ntv_range(values: Vec<py_val_t>) -> py_val_t {
    if let py_val::int(i) = *values[0] {
        py_val::new(py_val::list(
            (0..(i as usize))
                .map(|x| (py_val::new(py_val::int(x as i64))))
                .collect(),
        ))
    } else {
        panic!("TypeError");
    }
}

// type check
pub fn ntv_is_int(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::int(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_is_float(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::float(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_is_string(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::string(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_is_tuple(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::tuple(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_is_list(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::list(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_is_dict(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::dict(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_is_set(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if matches!(*values[0], py_val::set(_)) {
        py_val::True
    } else {
        py_val::False
    })
}

pub fn ntv_not(values: Vec<py_val_t>) -> py_val_t {
    py_val::new(if values[0].is_true() {
        py_val::False
    } else {
        py_val::True
    })
}
