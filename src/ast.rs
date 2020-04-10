//! # syntree
//! 抽象構文木の構成要素
//! https://docs.python.org/ja/3/library/ast.html#abstract-grammar からパクってきた(自作は無理)

// represents file(module).
pub type AST = Vec<ASTStmt>;

// ASDL(Abstract-type and Scheme-Definition Language) builtin types
// ID
pub type ASTIdentifier = String;
// メタ情報?
pub type ASTInt = i32;
pub type ASTString = String;
// ハードコードされた値?
// 色々アレなので数値は32bitの範囲で...
#[derive(Debug, PartialEq, Clone)]
pub enum ASTConstant {
    Int(isize),
    Float(f64),
    String(String),
    None,
    True,
    False,
}
// ???
// pub type ASTObject;

// AST parts
#[derive(Debug, PartialEq, Clone)]
pub enum ASTStmt {
    FuncDef(
        String,       // name
        ASTArguments, // arguments
        Vec<ASTStmt>, // body
    ),
    Return(
        Option<ASTExpr>, // value
    ),
    Delete(
        Vec<ASTExpr>, // targets
    ),
    Assign(
        Vec<ASTExpr>, // targets
        ASTExpr,      // value
    ),
    For(
        ASTExpr,      // target
        ASTExpr,      // iter
        Vec<ASTStmt>, // body
    ),
    While(
        ASTExpr,      // test
        Vec<ASTStmt>, // body
    ),
    If(
        ASTExpr,      // test
        Vec<ASTStmt>, // body
        Vec<ASTStmt>, // orelse
    ),
    Global(
        Vec<ASTIdentifier>, // names
    ),
    Print(
        Vec<ASTExpr>, // values
        bool,         // nl(ends with comma then not end with \n)
    ),
    Expr(
        ASTExpr, // value
    ),
    Pass,
    Break,
    Continue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTExpr {
    BoolOp(
        ASTBoolOp,    // op
        Vec<ASTExpr>, // values
    ),
    BinOp(
        Box<ASTExpr>, // left
        ASTOperator,  // op
        Box<ASTExpr>, // right
    ),
    UnaryOp(
        ASTUnaryOp,   // op
        Box<ASTExpr>, // operand
    ),
    IfExp(
        Box<ASTExpr>, // test
        Box<ASTExpr>, // body
        Box<ASTExpr>, // orelse
    ),
    Dict(
        Vec<ASTExpr>, // keys
        Vec<ASTExpr>, // values
    ),
    Set(
        Vec<ASTExpr>, // elts
    ),
    // そもそも内包表記あるんか????
    // ListComp(
    //     Box<ASTExpr>,          // elt
    //     Vec<ASTComprehension>, // generators
    // ),
    // SetComp(
    //     Box<ASTExpr>,          // elt
    //     Vec<ASTComprehension>, // generators
    // ),
    // DictComp(
    //     Box<ASTExpr>,          // key
    //     Box<ASTExpr>,          // value
    //     Vec<ASTComprehension>, // generators
    // ),
    Compare(
        Box<ASTExpr>,  // left
        Vec<ASTCmpOp>, // ops
        Vec<ASTExpr>,  // comparators
    ),
    Call(
        Box<ASTExpr>, // func
        Vec<ASTExpr>, // args
                      // omit keywords
    ),
    // ???
    // FormattedValue(
    //     Box<ASTExpr>,         // value,
    //     Option<ASTInt>,       // conversion,
    //     Option<Box<ASTExpr>>, // format_spec
    // ),
    // JoinedStr(
    //     Vec<ASTExpr>, // values
    // ),
    Constant(
        ASTConstant, // value
                     // Option<ASTString>, // kind
    ),
    //  -- the following expression can appear in assignment context
    // ???
    // Attribute(
    //     Box<ASTExpr>, // value
    //     ASTIdentifier, // attr,
    //                   // ASTExprContext, // ctx <- ???
    // ),
    Subscript(
        Box<ASTExpr>, // value
        ASTSlice,     // slice
                      // ASTExprContext, // ctx <- ???
    ),
    // Starred(
    //     Box<ASTExpr>, // value
    //                   // ASTExprContext, // ctx <- ???
    // ),
    Name(
        ASTIdentifier, // id
                       // ASTExprContext, // ctx <- ???
    ),
    List(
        Vec<ASTExpr>, // elts
                      // ASTExprContext, // ctx <- ???
    ),
    Tuple(
        Vec<ASTExpr>, // elts
                      // ASTExprContext, // ctx <- ???
    ),
}

// https://stackoverflow.com/questions/6679171/python-ast-several-semantics-unclear-e-g-expr-context
// 変数の位置
// pub enum ASTExprContext {
//     Load,     // 左辺値
//     Store,    // 右辺値
//     Del,      // del文の対象
//     AugLoad,  // 実引数?
//     AugStore, // 仮引数?
// }

#[derive(Debug, PartialEq, Clone)]
pub enum ASTSlice {
    Slice(
        Option<Box<ASTExpr>>, // lower
        Option<Box<ASTExpr>>, // upper
        Option<Box<ASTExpr>>, // step
    ),
    // ExtSlice(
    //     Vec<ASTSlice>, // dims
    // ),
    Index(
        Box<ASTExpr>, // value
    ),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTBoolOp {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTOperator {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTUnaryOp {
    Invert, // ~
    Not,    // `not`
    UAdd,
    USub,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTCmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

pub type ASTComprehension = (
    ASTExpr,      // target
    ASTExpr,      // iter
    Vec<ASTExpr>, // ifs
);

pub type ASTArguments = Vec<ASTArg>; // args

// arguments = (
//   arg* posonlyargs, : positional only arguments(before /)
//   arg* args, : normal arguments
//   arg? vararg, : variable length argument
//   arg* kwonlyargs, : keyword only arguments(after *)
//   expr* kw_defaults, : keyword only arguments default value
//   arg? kwarg, : kwargs
//   expr* defaults : default values
// )
// def hoge(positional_only_argument1, positional_only_argument2, /, argument1, argument2, *[vararg], keyword_only_argument1, keyword_only_argument2, **kwarg)
// 今回は普通にargsのみ利用可能とする

pub type ASTArg = ASTIdentifier; // arg
