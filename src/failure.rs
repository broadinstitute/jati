use crate::pos::Pos;
use crate::byte_state::{ByteState, ByteStateSet};

pub struct Failure {
    pos: Pos,
    actual: ByteState,
    expected: ByteStateSet
}

impl Failure {
    pub fn message(&self) -> String {
        format!("Parse failure at {}: found {}, expected {}", self.pos, self.actual, self.expected)
    }
}