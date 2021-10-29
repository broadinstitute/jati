use crate::pos::Pos;
use crate::byte_state::{ByteState, ByteStateSet};

pub struct Failure {
    pos: Pos,
    actual: ByteState,
    expected: ByteStateSet,
}

impl Failure {
    pub(crate) fn for_expected_end(actual: u8, pos: Pos) -> Failure {
        let actual = ByteState::from(Some(actual));
        let expected = ByteStateSet::new_end();
        Failure { pos, actual, expected }
    }
    pub fn message(&self) -> String {
        format!("Parse failure at {}: found {}, expected {}", self.pos, self.actual, self.expected)
    }
}