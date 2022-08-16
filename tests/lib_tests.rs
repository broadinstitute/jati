use jati::facade;

#[test]
fn hello() {
    assert!(facade("Hello").is_ok());
}

#[test]
fn hi() {
    assert!(facade("Hi").is_err());
}

#[test]
fn good() {
    assert!(facade("Good day").is_err());
}
