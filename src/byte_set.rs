use std::fmt::{Display, Formatter};

pub(crate) struct ByteSet {
    ranges: Vec<ByteRange>,
}

struct ByteRange {
    from: u8,
    to: u8,
}

impl ByteSet {
    pub(crate) fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

impl Display for ByteSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut range_iter = self.ranges.iter();
        match range_iter.next() {
            None => { write!(f, "[empty]")?; }
            Some(range0) => {
                write!(f, "{}", range0)?;
                for range in range_iter {
                    write!(f, ", {}", range)?;
                }
            }
        }
        Ok(())
    }
}

impl Display for ByteRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.from == self.to {
            write!(f, "{}", byte_pretty_print(self.from))
        } else {
            write!(f, "{}-{}", byte_pretty_print(self.from), byte_pretty_print(self.to))
        }
    }
}

pub(crate) fn byte_pretty_print(byte: u8) -> String {
    match byte {
        0 => String::from("\\0"),
        7 => String::from("\\a"),
        8 => String::from("\\b"),
        9 => String::from("\\t"),
        10 => String::from("\\n"),
        11 => String::from("\\v"),
        12 => String::from("\\f"),
        13 => String::from("\\r"),
        27 => String::from("\\e"),
        28 => String::from("^\\"),
        29 => String::from("^]"),
        30 => String::from("^^"),
        31 => String::from("^_"),
        32 => String::from("' '"),
        _ => {
            if byte < 32 {
                format!("^{}", (b'A' + (byte - 1)) as char)
            } else if byte < 128 {
                format!("{}", byte as char)
            } else {
                format!("{:02X}", byte)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_pretty_print() {
        assert_eq!(byte_pretty_print(0), String::from("\\0"));
        assert_eq!(byte_pretty_print(1), String::from("^A"));
        assert_eq!(byte_pretty_print(2), String::from("^B"));
        assert_eq!(byte_pretty_print(9), String::from("\\t"));
        assert_eq!(byte_pretty_print(b' '), String::from("' '"));
        assert_eq!(byte_pretty_print(b'a'), String::from("a"));
        assert_eq!(byte_pretty_print(128), String::from("80"));
        assert_eq!(byte_pretty_print(255), String::from("FF"));
    }
}