use jati::{FirstParser, parse_string};
use jati::parse::PResult;

fn print_error<T>(result: PResult<T>) -> PResult<T> {
    if let Err(error) = &result {
        println!("{}", error)
    }
    result
}

#[test]
fn hello() {
    let first_parser = FirstParser::new();
    assert!(print_error(parse_string(first_parser, "Hello")).is_ok());
}

#[test]
fn hello_world() {
    let first_parser = FirstParser::new();
    assert!(print_error(parse_string(first_parser, "Hello, world!")).is_err());
}

#[test]
fn hi() {
    let first_parser = FirstParser::new();
    assert!(print_error(parse_string(first_parser, "Hi")).is_err());
}

#[test]
fn good() {
    let first_parser = FirstParser::new();
    assert!(print_error(parse_string(first_parser, "Good day")).is_err());
}

