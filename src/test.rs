#[cfg(test)]
pub mod parser;
#[cfg(test)]
pub mod tokenizer;

#[test]
fn po() {
    use crate::eval::types::*;
    let a = make_int(4);
    let b = make_int(5);
    println!("{:p}", add_int(a, b));
    println!("{:#?}", a);
    assert_eq!(get_int(add_int(a, b)), 9);
    let c = make_char('a');
    assert_eq!(get_char(c), 'a');
}
