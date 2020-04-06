#[cfg(test)]
pub mod parser;
#[cfg(test)]
pub mod tokenizer;

#[test]
fn po() {
    use crate::evaluator::types::*;
    let a = py_val::make_int(4);
    let b = py_val::make_int(5);
    println!("{:p}", py_val::add_int(a, b));
    println!("{:#?}", a);
    assert_eq!(py_val::get_int(py_val::add_int(a, b)), 9);
    let c = py_val::make_char('a');
    assert_eq!(py_val::get_char(c), 'a');
}
