use std::ops::Deref;
use jati::{first_parser, parse_string};
use jati::parse::PResult;

fn print_error<T>(result: PResult<T>) -> PResult<T> {
    if let Err(error) = &result {
        println!("{}", error)
    }
    result
}

#[test]
fn hello() {
    assert!(print_error(parse_string(first_parser, "Hello")).is_ok());
}

#[test]
fn hi() {
    assert!(print_error(parse_string(first_parser, "Hi")).is_err());
}

#[test]
fn good() {
    assert!(print_error(parse_string(first_parser, "Good day")).is_err());
}
