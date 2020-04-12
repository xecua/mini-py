def range(n):
    return ntv_range(n)

def __add__(lhs, rhs):
    if ntv_is_int(lhs):
        if ntv_is_int(rhs):
            return ntv_add_int(lhs, rhs)
        elif ntv_is_float(rhs):
            return ntv_add_float(ntv_itof(lhs), rhs)
        else:
            ntv_panic()
    elif ntv_is_float(lhs):
        if ntv_is_int(rhs):
            return ntv_add_float(lhs, ntv_itof(rhs))
        elif ntv_is_float(rhs):
            return ntv_add_float(lhs, rhs)
        else:
            ntv_panic()
    elif ntv_is_string(lhs):
        if ntv_is_string(rhs):
            return ntv_add_string(lhs, rhs)
        else:
            ntv_panic()
    elif ntv_is_tuple(lhs):
        if ntv_is_tuple(rhs):
            return ntv_add_tuple(lhs, rhs)
        else:
            ntv_panic()
    elif ntv_is_list(lhs):
        if ntv_is_list(rhs):
            return ntv_add_list(lhs, rhs)
        else:
            ntv_panic()
    else:
        ntv_panic()


def __sub__(lhs, rhs):
    if ntv_is_int(lhs):
        if ntv_is_int(rhs):
            return ntv_sub_int(lhs, rhs)
        elif ntv_is_float(rhs):
            return ntv_sub_float(ntv_itof(lhs), rhs)
        else:
            ntv_panic()
    elif ntv_is_float(lhs):
        if ntv_is_int(rhs):
            return ntv_sub_float(lhs, ntv_itof(rhs))
        elif ntv_is_float(rhs):
            return ntv_sub_float(lhs, rhs)
        else:
            ntv_panic()
    else:
        ntv_panic()

def __mul__(lhs, rhs):
    if ntv_is_int(lhs):
        if ntv_is_int(rhs):
            return ntv_mul_int(lhs, rhs)
        elif ntv_is_float(rhs):
            return ntv_mul_float(ntv_itof(lhs), rhs)
        else:
            ntv_panic()
    elif ntv_is_float(lhs):
        if ntv_is_int(rhs):
            return ntv_mul_float(lhs, ntv_itof(rhs))
        elif ntv_is_float(rhs):
            return ntv_mul_float(lhs, rhs)
        else:
            ntv_panic()
    elif ntv_is_string(lhs):
        if ntv_is_int(rhs):
            res = ""
            for i in range(rhs):
                res + lhs
            return res
        else:
            ntv_panic()
    elif ntv_is_list(lhs):
        if ntv_is_int(rhs):
            # repeat
            ntv_panic()
        else:
            ntv_panic()
    elif ntv_is_string(lhs):
        if ntv_is_int(rhs):
            # repeat
            ntv_panic()
        else:
            ntv_panic()
    else:
        ntv_panic()

def __div__(lhs, rhs):
    if ntv_is_int(lhs):
        if ntv_is_int(rhs):
            return ntv_div_int(lhs, rhs)
        elif ntv_is_float(rhs):
            return ntv_div_float(ntv_itof(lhs), rhs)
        else:
            ntv_panic()
    elif ntv_is_float(lhs):
        if ntv_is_int(rhs):
            return ntv_div_float(lhs, ntv_itof(rhs))
        elif ntv_is_float(rhs):
            return ntv_div_float(lhs, rhs)
        else:
            ntv_panic()
    else:
        ntv_panic()

def __mod__(lhs, rhs):
    if ntv_is_int(lhs):
        if ntv_is_int(rhs):
            return ntv_mod_int(lhs, rhs)
        elif ntv_is_float(rhs):
            return ntv_mod_float(ntv_itof(lhs), rhs)
        else:
            ntv_panic()
    elif ntv_is_float(lhs):
        if ntv_is_int(rhs):
            return ntv_mod_float(lhs, ntv_itof(rhs))
        elif ntv_is_float(rhs):
            return ntv_mod_float(lhs, rhs)
        else:
            ntv_panic()
    elif ntv_is_string(lhs):
        # format string
        ntv_panic()
    else:
        ntv_panic()

def __lshift__(lhs, rhs):
    if ntv_is_int(lhs) and ntv_is_int(rhs):
        return ntv_lshift_int(lhs, rhs)
    else:
        ntv_panic()

def __rshift__(lhs, rhs):
    if ntv_is_int(lhs) and ntv_is_int(rhs):
        return ntv_rshift_int(lhs, rhs)
    else:
        ntv_panic()

def __or__(lhs, rhs):
    if ntv_is_int(lhs) and ntv_is_int(rhs):
        return ntv_or_int(lhs, rhs)
    else:
        ntv_panic()

def __and__(lhs, rhs):
    if ntv_is_int(lhs) and ntv_is_int(rhs):
        return ntv_and_int(lhs, rhs)
    else:
        ntv_panic()

def __xor__(lhs, rhs):
    if ntv_is_int(lhs) and ntv_is_int(rhs):
        return ntv_xor_int(lhs, rhs)
    else:
        ntv_panic()

def __invert__(val):
    if ntv_is_int(val):
        return ntv_invert_int(val)
    else:
        ntv_panic()

def __not__(val):
    return ntv_not(val)

def __plus__(val):
    if ntv_is_int(val) or ntv_is_float(val):
        return val
    else:
        ntv_panic()

def __minus__(val):
    if ntv_is_int(val) or ntv_is_float(val):
        return val * -1
    else:
        ntv_panic()

def __eq__(lhs, rhs):
    if ntv_is_int(lhs):
        if ntv_is_int(rhs):
            return ntv_eq_int(lhs, rhs)
        elif ntv_is_float(lhs):
            return ntv_cmp_float(ntv_itof(lhs), rhs) == 0
        else:
            ntv_panic()
    elif ntv_is_float(lhs):
        if ntv_is_int(rhs):
            return ntv_cmp_float(lhs, ntv_itof(rhs)) == 0
        elif ntv_is_float(rhs):
            return ntv_cmp_float(lhs, rhs) == 0
        else:
            ntv_panic()
    elif ntv_is_string(lhs) and ntv_is_string(rhs):
        # unimplemented
        ntv_panic()
    elif ntv_is_list(lhs) and ntv_is_list(rhs):
        # unimplemented
        ntv_panic()
    elif ntv_is_tuple(lhs) and ntv_is_tuple(rhs):
        # unimplemented
        ntv_panic()
    elif ntv_is_dict(lhs) and ntv_is_dict(rhs):
        # unimplemented
        ntv_panic()
    elif ntv_is_set(lhs) and ntv_is_set(rhs):
        # unimplemented
        ntv_panic()
    else:
        ntv_panic()

def print(val):
    if ntv_is_int(val):
        ntv_print_string(ntv_repr_int(val))
    elif ntv_is_float(val):
        ntv_print_string(ntv_repr_float(val))
    elif ntv_is_string(val):
        ntv_print_string(val)
    else:
        ntv_panic()
