use crate::ast::*;

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
