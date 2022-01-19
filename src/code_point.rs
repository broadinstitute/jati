use crate::util::byte_util;
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub(crate) struct CodePoint {
    pub(crate) char: u32,
}

#[derive(Clone)]
pub(crate) struct Utf8Error {
    message: String,
    pub(crate) i_byte: u8
}

impl Utf8Error {
    pub(crate) fn new(message: String, i_byte: u8) -> Utf8Error {
        Utf8Error { message, i_byte }
    }
}

// impl From<String> for Utf8Error {
//     fn from(message: String) -> Self { Utf8Error { message } }
// }


impl From<u32> for CodePoint {
    fn from(char: u32) -> Self {
        CodePoint { char }
    }
}

impl CodePoint {
    fn unexpected_end_error(i_byte: u8) -> Utf8Error {
        Utf8Error::new(
            String::from("Input ended before UTF-8 code point was complete"), i_byte
        )
    }
    fn validate_non_initial_byte(byte: u8, i_byte: u8) -> Result<(), Utf8Error> {
        if byte >> 6 == 0b10 {
            Ok(())
        } else {
            Err(Utf8Error::new(format!(
                "Following byte of a code point needs to be between {} and {}, but is {}.",
                format!("{:02X}", 0b10000000), format!("{:02X}", 0b10111111),
                byte_util::byte_pretty_print(byte)
            ), i_byte))
        }
    }
    fn read_more(bytes_iter: &mut Box<dyn Iterator<Item=u8>>, byte0: u8)
                 -> Result<CodePoint, Utf8Error> {
        let char =
            if byte0 >> 7 == 0b0 {
                byte0 as u32
            } else if byte0 >> 5 == 0b110 {
                let byte1 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error(1) })?;
                CodePoint::validate_non_initial_byte(byte1, 1)?;
                (((byte0 & 0b11111) as u32) << 6) + ((byte1 & 0b111111) as u32)
            } else if byte0 >> 4 == 0b1110 {
                let byte1 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error(1) })?;
                CodePoint::validate_non_initial_byte(byte1, 1)?;
                let byte2 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error(2) })?;
                CodePoint::validate_non_initial_byte(byte2, 2)?;
                (((byte0 & 0b1111) as u32) << 12) + (((byte1 & 0b111111) as u32) << 6) +
                    ((byte2 & 0b111111) as u32)
            } else if byte0 >> 3 == 0b11110 {
                let byte1 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error(1) })?;
                CodePoint::validate_non_initial_byte(byte1, 1)?;
                let byte2 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error(2) })?;
                CodePoint::validate_non_initial_byte(byte2, 2)?;
                let byte3 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error(3) })?;
                CodePoint::validate_non_initial_byte(byte3, 3)?;
                (((byte0 & 0b1111) as u32) << 18) + (((byte1 & 0b111111) as u32) << 12) +
                    (((byte2 & 0b111111) as u32) << 6) + ((byte3 & 0b111111) as u32)
            } else {
                return Err(Utf8Error::new(
                    format!("A code point encoding cannot start with {:02X}.", byte0),
                    0
                ))
            };
        Ok(CodePoint::from(char))
    }
    pub(crate) fn read(bytes_iter: &mut Box<dyn Iterator<Item=u8>>)
        -> Option<Result<CodePoint, Utf8Error>> {
        bytes_iter.next().map(|byte0| CodePoint::read_more(bytes_iter, byte0))
    }
    pub(crate) fn n_utf8_bytes(&self) -> u8 {
        if self.char <= 127 {
            1
        } else if self.char <= 2047 {
            2
        } else if self.char <= 65535 {
            3
        } else {
            4
        }
    }
}

impl Display for Utf8Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
