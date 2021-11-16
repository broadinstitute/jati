use crate::pos_old::PosOld;
use crate::byte_state::{ByteState, ByteStateSet};

pub struct FailureOld {
    pos: PosOld,
    actual: ByteState,
    expected: ByteStateSet,
}

impl FailureOld {
    pub(crate) fn for_expected_end(actual: u8, pos: PosOld) -> FailureOld {
        let actual = ByteState::from(Some(actual));
        let expected = ByteStateSet::new_end();
        FailureOld { pos, actual, expected }
    }
    pub fn message(&self) -> String {
        format!("Parse failure at {}: found {}, expected {}", self.pos, self.actual, self.expected)
    }
}