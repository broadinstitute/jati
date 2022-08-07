use std::fmt::{Display, Formatter};
use crate::kind::Kind;
use crate::util::ne_set::NonEmptySet;

pub(crate) struct Failure {
    actual: Kind,
    expected: NonEmptySet<Kind>,
}

impl Failure {
    pub(crate) fn new(actual: Kind, expected: NonEmptySet<Kind>) -> Failure {
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