#[derive(Clone, Debug)]
pub enum Token {
    OR, // or
    AND, // and
    NOT, // not
    IS, // is
    IN, // in
    EQ, // =
    EQEQ, // ==
    NEQ, // !=
    GT, // >
    GEQ, // >=
    LT, // <
    LEQ, // <=
    PLUS, // +
    MINUS, // -
    MUL, // *
    DIV, // /
    MOD, // %
    TILDE, // ~
    LSHIFT, // <<
    RSHIFT, // >>
    XOR, // ^
    AMP, // &
    BAR, // |
    NONE, // None
    BREAK, // break
    CONTINUE, // continue
    PASS, // pass
    RETURN, // return
    DEL, // del
    PRINT, // print
    GLOBAL, // global
    IF, // if
    ELIF, // elif
    ELSE, // else
    FOR, // for
    WHILE, // while
    DEF, // def
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    LBRACKET, // [
    RBRACKET, // ]
    PERIOD, // .
    COMMA, // ,
    COLON, // :
    INT(i32), // integer literal
    FLOAT(f32), // floating point number literal
    STRING(String), // str literal
    ID(String), // identifier
    NEWLINE, // \n
    EOF, // EOF
    INDENT, // indent
    DEDENT, // dedent
    EMPTY, // for initial
}
