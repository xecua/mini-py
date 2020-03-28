#[derive(Clone, Debug)]
pub enum Token {
    EQ,             // =
    EQEQ,           // ==
    NEQ,            // !=
    GT,             // >
    GEQ,            // >=
    LT,             // <
    LEQ,            // <=
    PLUS,           // +
    MINUS,          // -
    MUL,            // *
    DIV,            // /
    MOD,            // %
    TILDE,          // ~
    LSHIFT,         // <<
    RSHIFT,         // >>
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
    INT(i32),       // integer literal
    FLOAT(f32),     // floating point number literal
    STRING(String), // str literal
    ID(String),     // identifier
    EOF,            // EOF
    INDENT,         // indent
    UNINDENT,         // unindent
    EMPTY,          // for initial
}
