use crate::byte_set::{ByteSet, byte_pretty_print};
use std::fmt::{Display, Formatter};

const NONE_DISPLAY_STR: &str = "[none]";
const END_DISPLAY_STR: &str = "[end]";

pub(crate) struct ByteStateSet {
    includes_end: bool,
    bytes_set: ByteSet,
}

pub struct ByteState(Option<u8>);

impl ByteState {
    pub(crate) fn to_byte_opt(&self) -> Option<u8> {
        self.0
    }
}

impl From<Option<u8>> for ByteState {
    fn from(byte_opt: Option<u8>) -> Self {
        ByteState(byte_opt)
    }
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
        let ByteState(byte_opt) = self;
        match byte_opt {
            Some(byte) => { write!(f, "{}", byte_pretty_print(*byte)) }
            None => { write!(f, "{}", END_DISPLAY_STR) }
        }
    }
}

