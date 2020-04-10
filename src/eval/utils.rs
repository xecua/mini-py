use crate::ast::*;

pub fn operator_to_function_name(op: &ASTOperator) -> &'static str {
    use ASTOperator::*;
    match op {
        Add => "add",
        Sub => "sub",
        Mult => "mult",
        Div => "div",
        Mod => "mod",
        LShift => "lshift",
        RShift => "rshift",
        BitOr => "_or",
        BitXor => "xor",
        BitAnd => "_and",
    }
}

pub fn unary_operator_to_function_name(op: &ASTUnaryOp) -> &'static str {
    use ASTUnaryOp::*;
    match op {
        Invert => "invert",
        Not => "_not",
        UAdd => "plus",
        USub => "minus",
    }
}

pub fn compare_operator_to_function_name(op: &ASTCmpOp) -> &'static str {
    use ASTCmpOp::*;
    match op {
        ASTCmpOp::Eq => "eq",
        NotEq => "neq",
        Lt => "lt",
        LtE => "le",
        Gt => "gt",
        GtE => "ge",
        Is => "_is",
        IsNot => "is_not", // 余力があればnot(A is B)にしたい
        In => "in",
        NotIn => "not_in"
    }
}
