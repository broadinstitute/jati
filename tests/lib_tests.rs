use std::fmt::Display;
use jati::facade;

fn print_error<T, E: Display>(result: Result<T, E>) -> Result<T, E> {
    if let Err(error) = &result {
        eprintln!("{}", error)
    }
    result
}

#[test]
fn hello() {
    assert!(print_error(facade("Hello")).is_ok());
}

#[test]
fn hi() {
    assert!(print_error(facade("Hi")).is_err());
}

#[test]
fn good() {
    assert!(print_error(facade("Good day")).is_err());
}
