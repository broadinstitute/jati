use crate::util::byte_util;

#[derive(PartialEq)]
pub(crate) struct CodePoint {
    pub(crate) char: u32,
}

pub(crate) struct Utf8Error {
    message: String,
}

impl From<String> for Utf8Error {
    fn from(message: String) -> Self { Utf8Error { message } }
}


impl From<u32> for CodePoint {
    fn from(char: u32) -> Self {
        CodePoint { char }
    }
}

impl CodePoint {
    fn unexpected_end_error() -> Utf8Error {
        Utf8Error::from(
            String::from("Input ended before UTF-8 code point was complete")
        )
    }
    fn check_following_byte(byte: u8) -> Result<(), Utf8Error> {
        if byte >> 6 == 0b10 {
            Ok(())
        } else {
            Err(Utf8Error::from(format!(
                "Following byte of a code point needs to be between {} and {}, but is {}.",
                format!("{:02X}", 0b10000000), format!("{:02X}", 0b10111111),
                format!("{}", byte_util::byte_pretty_print(byte))
            )))
        }
    }
    fn read_more(bytes_iter: &mut Box<dyn Iterator<Item=u8>>, byte0: u8)
                 -> Result<CodePoint, Utf8Error> {
        let char =
            if byte0 >> 7 == 0b0 {
                byte0 as u32
            } else if byte0 >> 5 == 0b110 {
                let byte1 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error() })?;
                CodePoint::check_following_byte(byte1)?;
                (((byte0 & 0b11111) as u32) << 6) + ((byte1 & 0b111111) as u32)
            } else if byte0 >> 4 == 0b1110 {
                let byte1 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error() })?;
                CodePoint::check_following_byte(byte1)?;
                let byte2 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error() })?;
                CodePoint::check_following_byte(byte2)?;
                (((byte0 & 0b1111) as u32) << 12) + (((byte1 & 0b111111) as u32) << 6) +
                    ((byte2 & 0b111111) as u32)
            } else if byte0 >> 3 == 0b11110 {
                let byte1 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error() })?;
                CodePoint::check_following_byte(byte1)?;
                let byte2 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error() })?;
                CodePoint::check_following_byte(byte2)?;
                let byte3 =
                    bytes_iter.next().ok_or_else(|| { CodePoint::unexpected_end_error() })?;
                CodePoint::check_following_byte(byte3)?;
                (((byte0 & 0b1111) as u32) << 18) + (((byte1 & 0b111111) as u32) << 12) +
                    (((byte2 & 0b111111) as u32) << 6) + ((byte3 & 0b111111) as u32)
            } else {
                return Err(Utf8Error::from(
                    format!("A code point encoding cannot start with {:02X}.", byte0)
                ))
            };
        Ok(CodePoint::from(char))
    }
    pub(crate) fn read(bytes_iter: &mut Box<dyn Iterator<Item=u8>>) -> Option<Result<CodePoint, Utf8Error>> {
        match bytes_iter.next() {
            None => { None }
            Some(byte0) => { Some(CodePoint::read_more(bytes_iter, byte0)) }
        }
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
