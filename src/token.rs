//! # token
//! 字句解析後のトークン

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    EQ,             // =
    EQEQ,           // ==
    NEQ,            // !=
    GT,             // >
    GEQ,            // >=
    RSHIFT,         // >>
    LT,             // <
    LEQ,            // <=
    LSHIFT,         // <<
    PLUS,           // +
    MINUS,          // -
    MUL,            // *
    DIV,            // /
    MOD,            // %
    TILDE,          // ~
    XOR,            // ^
    AMP,            // &
    BAR,            // |
    LPAREN,         // (
    RPAREN,         // )
    LBRACE,         // {
    RBRACE,         // }
    LBRACKET,       // [
    RBRACKET,       // ]
    PERIOD,         // .
    COMMA,          // ,
    COLON,          // :
    NEWLINE,        // \n
    INT(isize),     // integer literal
    FLOAT(f64),     // floating point number literal
    STRING(String), // str literal
    ID(String),     // identifier
    OR,             // or
    AND,            // and
    NOT,            // not
    IS,             // is
    IN,             // in
    NONE,           // None
    BREAK,          // break
    CONTINUE,       // continue
    PASS,           // pass
    RETURN,         // return
    DEL,            // del
    PRINT,          // print
    GLOBAL,         // global
    IF,             // if
    ELIF,           // elif
    ELSE,           // else
    FOR,            // for
    WHILE,          // while
    DEF,            // def
    TRUE,           // True
    FALSE,          // False
    EOF,            // EOF
    INDENT,         // indent
    DEDENT,         // dedent
    EMPTY,          // for initial
}
