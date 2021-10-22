use crate::byte_set::{ByteSet, byte_pretty_print};
use std::fmt::{Display, Formatter};

const NONE_DISPLAY_STR: &str = "[none]";
const END_DISPLAY_STR: &str = "[end]";

pub(crate) struct ByteStateSet {
    includes_end: bool,
    bytes_set: ByteSet,
}

pub enum ByteState {
    Byte(u8),
    End,
}

impl Display for ByteStateSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.includes_end {
            if self.bytes_set.is_empty() {
                write!(f, "{}", END_DISPLAY_STR)
            } else {
                write!(f, "{},{}", END_DISPLAY_STR, self.bytes_set)
            }
        } else if self.bytes_set.is_empty() {
            write!(f, "{}", NONE_DISPLAY_STR)
        } else {
            write!(f, "{}", self.bytes_set)
        }
    }
}

impl Display for ByteState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteState::Byte(byte) => { write!(f, "{}", byte_pretty_print(*byte)) }
            ByteState::End => { write!(f, "{}", END_DISPLAY_STR) }
        }
    }
}

