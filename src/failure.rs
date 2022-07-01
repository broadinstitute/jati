use std::fmt::{Display, Formatter};
use crate::kind::Kind;

pub(crate) struct Failure {
    actual: Kind,
    expected: Vec<Kind>,
}

impl Failure {
    pub(crate) fn new(actual: Kind, expected: Vec<Kind>) -> Failure {
        Failure { actual, expected }
    }
}


impl Display for Failure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let expected_iter = self.expected.iter();
        let expected =
            expected_iter.map(|expected| { expected.as_str() }).collect::<Vec<&str>>()
                .join(", ");
        write!(f, "Found {}, expected one of {}", self.actual, expected)
    }
}