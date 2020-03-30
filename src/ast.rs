//! # syntree
//! 抽象構文木の構成要素

pub enum AST {
    Init, // for initialization
    File(
        Vec<Box<AST>>, // stmt
    ),
    FuncDef(
        // funcdef from compound_stmt
        String,   // name
        Box<AST>, // typedargslist from parameters
        Box<AST>, // func_body_suite
    ),
    Parameters(
        Option<Box<AST>>, // typedargslist
    ),
    TypedArgsList(
        Vec<String>, // NAME
    ),
    FuncBodySuite(
        Vec<AST>, // simple_stmt or stmt[s]
    ),
    Stmt(
        Box<AST>, // simple_stmt or if_stmt or while_stmt or for_stmt or funcdef
    ),
    SimpleStmt(
        Box<AST>, // small_stmt
    ),
    SmallStmt(
        // expr_stmt or del_stmt or pass_stmt or global_stmt or (break_stmt or continue_stmt, return_stmt (from flow_stmt))
        Box<AST>,
    ),
    ExprStmt(
        Box<AST>, // testlist_star_expr
        Vec<AST>, // annassign or testlist_star_expr
    ),
    Annassign(
        Box<AST>,         // test
        Option<Box<AST>>, // testlist_star_expr
    ),
    TestlistStarExpr(
        Vec<AST>, // (test or star_expr)[s]
    ),
    DelStmt(
        Box<AST>, // exprlist
    ),
    PassStmt,
    BreakStmt,                    // from flow_stmt
    ContinueStmt,                 // from flow_stmt
    ReturnStmt(Option<Box<AST>>), // testlist_star_expr // from flow_stmt
    GlobalStmt(Vec<String>),      // NAME
    IfStmt(
        // if_stmt from compound_stmt
        (Box<AST>, Box<AST>),         // if condition (test), suite
        Vec<(Box<AST>, Box<AST>)>,    // elif condition (test), suite
        Option<(Box<AST>, Box<AST>)>, // else condition (test), suite
    ),
    WhileStmt(
        // while_stmt from compound_stmt
        Box<AST>, // test
        Box<AST>, // suite
    ),
    ForStmt(
        // for_stmt from compound_stmt
        Box<AST>, // exprlist
        Box<AST>, // testlist
        Box<AST>, // suite
    ),
    Suite(
        Vec<Box<AST>>, // simple_stmt or stmt[s]
    ),
    Test(
        Box<AST>,         // or_test
        Option<Box<AST>>, // condition (or_test)
        Option<Box<AST>>, // else (test)
    ),
    OrTest(
        Vec<Box<AST>>, // and_test[s]
    ),
    AndTest(
        Vec<AST>, // not_test[s]
    ),
    NotTest(
        Box<AST>, // not_test or comparison
    ),
    Comparison(
        Box<AST>,                  // expr
        Vec<(Box<AST>, Box<AST>)>, // comp_op, expr
    ),
    Lt,    // <
    Gt,    // >
    Eq,    // ==
    Leq,   // <=
    Geq,   // >=
    Neq,   // !=
    In,    // in
    NotIn, // not in
    Is,    // is
    IsNot, // is not
    StarExpr(
        Box<AST>, // expr
    ),
    Expr(
        Vec<AST>, // xor_expr
    ),
    XorExpr(
        Vec<AST>, // and_expr
    ),
    AndExpr(
        Vec<AST>, // shift_expr
    ),
    ShiftExpr(
        Box<AST>,        // arith_expr
        Vec<(AST, AST)>, // << or >>, arith_expr
    ),
    Lshift, // <<
    Rshift, // >>
    ArithExpr(
        Box<AST>,        // term
        Vec<(AST, AST)>, // + or -, term
    ),
    Plus,  // +
    Minus, // -
    Term(
        Box<AST>,        // factor
        Vec<(AST, AST)>, // * or / or %, factor
    ),
    Mul, // *
    Div, // /
    Mod, // %
}
