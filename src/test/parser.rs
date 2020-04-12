use crate::ast::*;
use crate::parser::Parser;
use ordered_float::OrderedFloat;

#[test]
fn test_test_1() {
    let mut parser = Parser::new("testcase/test_1.py").unwrap();
    let ast = parser.parse();
    assert_eq!(
        ast,
        vec![ASTStmt::Expr(ASTExpr::Call(
            Box::new(ASTExpr::Name(String::from("print"))),
            vec![ASTExpr::BinOp(
                Box::new(ASTExpr::Constant(ASTConstant::Int(1))),
                ASTOperator::Add,
                Box::new(ASTExpr::Constant(ASTConstant::Int(2)))
            )],
        ))]
    );
}

#[test]
fn test_test_2() {
    let mut parser = Parser::new("testcase/test_2.py").unwrap();
    let ast = parser.parse();
    assert_eq!(
        ast,
        vec![
            ASTStmt::FuncDef(
                String::from("test"),
                vec![String::from("poi")],
                vec![
                    ASTStmt::Assign(
                        vec![ASTExpr::Name(String::from("a"))],
                        ASTExpr::Constant(ASTConstant::Float(OrderedFloat(1.0)))
                    ),
                    ASTStmt::Expr(ASTExpr::Call(
                        Box::new(ASTExpr::Name(String::from("print"))),
                        vec![ASTExpr::Name(String::from("a"))]
                    ))
                ]
            ),
            ASTStmt::If(
                ASTExpr::Compare(
                    Box::new(ASTExpr::Name(String::from("__name__"))),
                    vec![ASTCmpOp::Eq],
                    vec![ASTExpr::Constant(ASTConstant::String(String::from(
                        "__main__"
                    )))]
                ),
                vec![ASTStmt::Expr(ASTExpr::Call(
                    Box::new(ASTExpr::Name(String::from("test"))),
                    vec![ASTExpr::Constant(ASTConstant::Int(2))]
                ))],
                Vec::new()
            )
        ]
    )
}

#[test]
fn test_fizzbuzz() {
    let mut parser = Parser::new("testcase/fizzbuzz.py").unwrap();
    let ast = parser.parse();
    assert_eq!(
        ast,
        vec![ASTStmt::For(
            ASTExpr::Name(String::from("i")),
            ASTExpr::Call(
                Box::new(ASTExpr::Name(String::from("range"))),
                vec![ASTExpr::Constant(ASTConstant::Int(100))]
            ),
            vec![ASTStmt::If(
                ASTExpr::Compare(
                    Box::new(ASTExpr::BinOp(
                        Box::new(ASTExpr::Name(String::from("i"))),
                        ASTOperator::Mod,
                        Box::new(ASTExpr::Constant(ASTConstant::Int(15)))
                    )),
                    vec![ASTCmpOp::Eq],
                    vec![ASTExpr::Constant(ASTConstant::Int(0))]
                ),
                vec![ASTStmt::Expr(ASTExpr::Call(
                    Box::new(ASTExpr::Name(String::from("print"))),
                    vec![ASTExpr::Constant(ASTConstant::String(String::from(
                        "fizzbuzz"
                    )))],
                ))],
                vec![ASTStmt::If(
                    ASTExpr::Compare(
                        Box::new(ASTExpr::BinOp(
                            Box::new(ASTExpr::Name(String::from("i"))),
                            ASTOperator::Mod,
                            Box::new(ASTExpr::Constant(ASTConstant::Int(5)))
                        )),
                        vec![ASTCmpOp::Eq],
                        vec![ASTExpr::Constant(ASTConstant::Int(0))]
                    ),
                    vec![ASTStmt::Expr(ASTExpr::Call(
                        Box::new(ASTExpr::Name(String::from("print"))),
                        vec![ASTExpr::Constant(ASTConstant::String(String::from("fizz")))],
                    ))],
                    vec![ASTStmt::If(
                        ASTExpr::Compare(
                            Box::new(ASTExpr::BinOp(
                                Box::new(ASTExpr::Name(String::from("i"))),
                                ASTOperator::Mod,
                                Box::new(ASTExpr::Constant(ASTConstant::Int(3)))
                            )),
                            vec![ASTCmpOp::Eq],
                            vec![ASTExpr::Constant(ASTConstant::Int(0))]
                        ),
                        vec![ASTStmt::Expr(ASTExpr::Call(
                            Box::new(ASTExpr::Name(String::from("print"))),
                            vec![ASTExpr::Constant(ASTConstant::String(String::from("buzz")))],
                        ))],
                        vec![ASTStmt::Expr(ASTExpr::Call(
                            Box::new(ASTExpr::Name(String::from("print"))),
                            vec![ASTExpr::Name(String::from("i"))]
                        ))]
                    )]
                )],
            )]
        )]
    );
}

#[test]
fn test_test_3() {
    let mut parser = Parser::new("testcase/test_3.py").unwrap();
    let ast = parser.parse();
    assert_eq!(
        ast,
        vec![ASTStmt::Expr(ASTExpr::Call(
            Box::new(ASTExpr::Name(String::from("print"))),
            vec![ASTExpr::List(vec![
                ASTExpr::Constant(ASTConstant::Int(1)),
                ASTExpr::Constant(ASTConstant::Int(2)),
                ASTExpr::Constant(ASTConstant::Int(3))
            ])],
        ))]
    );
}
