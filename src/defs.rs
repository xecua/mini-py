use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INT_PATTERN: Regex = Regex::new(r"0|[1-9][0-9]*").unwrap();
    static ref FLOAT_PATTERN: Regex = Regex::new(r"([0-9]+\.[0-9]*|\.[0-9]+)").unwrap();
    static ref STRING_PATTERN: Regex = Regex::new(r#""([^\"]|\\.)*""#).unwrap();
    static ref ID_PATTERN: Regex = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
}