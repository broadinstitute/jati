use crate::error::Error;

#[derive(Clone)]
pub struct Utf8Chars<B: Iterator<Item=Result<u8, Error>> + Clone> {
    bytes: B,
}

impl<B: Iterator<Item=Result<u8, Error>> + Clone> Utf8Chars<B> {
    pub fn new(bytes: B) -> Self {
        Utf8Chars { bytes }
    }
    pub fn try_char(&mut self) -> Result<Option<char>, Error> {
        let byte1 = self.bytes.next().transpose()?;
        match byte1 {
            None => { Ok(None) }
            Some(byte1) => {
                if byte1 & 0b1000_0000 == 0b0000_0000 {
                    Ok(Some(byte1 as char))
                } else if byte1 & 0b1110_0000 == 0b1100_0000 {
                    let bits1 = (byte1 & 0b0001_1111) as u32;
                    let bits2 = extract_utf8_bits(self.bytes.next())? as u32;
                    let unicode = (bits1 << 6) | bits2;
                    Ok(Some(unsafe { std::char::from_u32_unchecked(unicode) }))
                } else if byte1 & 0b1111_0000 == 0b1110_0000 {
                    let bits1 = (byte1 & 0b0000_1111) as u32;
                    let bits2 = extract_utf8_bits(self.bytes.next())? as u32;
                    let bits3 = extract_utf8_bits(self.bytes.next())? as u32;
                    let unicode = (bits1 << 12) | (bits2 << 6) | bits3;
                    Ok(Some(unsafe { std::char::from_u32_unchecked(unicode) }))
                } else if byte1 & 0b1111_1000 == 0b1111_0000 {
                    let bits1 = (byte1 & 0b0000_0111) as u32;
                    let bits2 = extract_utf8_bits(self.bytes.next())? as u32;
                    let bits3 = extract_utf8_bits(self.bytes.next())? as u32;
                    let bits4 = extract_utf8_bits(self.bytes.next())? as u32;
                    let unicode = (bits1 << 18) | (bits2 << 12) | (bits3 << 6) | bits4;
                    Ok(Some(unsafe { std::char::from_u32_unchecked(unicode) }))
                } else {
                    Err(Error::from(format!("Invalid UTF-8 lead byte '{:x}'.", byte1)))
                }
            }
        }
    }
}

fn extract_utf8_bits(byte: Option<Result<u8, Error>>) -> Result<u8, Error> {
    let byte =
        byte.transpose()?
            .ok_or_else(|| Error::from("Input ended before UTF-8 complete."))?;
    if byte & 0b1100_0000 == 0b1000_0000 {
        Ok(byte & 0b0011_1111)
    } else {
        Err(Error::from(format!("Invalid UTF-8 continuation byte '{:x}'.", byte)))
    }
}

impl<B: Iterator<Item=Result<u8, Error>> + Clone> Iterator for Utf8Chars<B> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_char().transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_decode_valid_utf8() {
        assert_round_trip("");
        assert_round_trip("  ");
        assert_round_trip("\t\n\r");
        assert_round_trip("Hello, world!");
        assert_round_trip("Görüşürüz!");
        assert_round_trip("很高兴见到你!");
        assert_round_trip("🦃😨💒👒💍™️🙉");
    }

    fn assert_round_trip(string: &str) {
        let bytes = string.bytes().map(Ok);
        let utf8_chars = Utf8Chars::new(bytes);
        let decoded: String = utf8_chars.map(Result::unwrap).collect();
        assert_eq!(string, decoded);
    }
    #[test]
    fn handle_invalid_utf8() {
        const ASCII: u8 = 0b0001_0111;
        const CONTI: u8 = 0b1000_0111;
        const LEAD2: u8 = 0b1100_0111;
        const LEAD3: u8 = 0b1110_0111;
        const LEAD4: u8 = 0b1111_0111;
        assert_error(&[CONTI], "Invalid UTF-8 lead byte '87'.");
        assert_error(&[LEAD2], "Input ended before UTF-8 complete.");
        for lead in [LEAD2, LEAD3, LEAD4].iter() {
            assert_error(&[*lead], "Input ended before UTF-8 complete.");
            for non_conti in [ASCII, LEAD2, LEAD3, LEAD4].iter() {
                assert_error(&[*lead, *non_conti],
                             &format!("Invalid UTF-8 continuation byte '{:x}'.",
                                      non_conti));
            }
        }
        for lead in [LEAD3, LEAD4].iter() {
            assert_error(&[*lead, CONTI], "Input ended before UTF-8 complete.");
            for non_conti in [ASCII, LEAD2, LEAD3, LEAD4].iter() {
                assert_error(&[*lead, CONTI, *non_conti],
                             &format!("Invalid UTF-8 continuation byte '{:x}'.",
                                      non_conti));
            }
        }
        assert_error(&[LEAD4, CONTI, CONTI], "Input ended before UTF-8 complete.");
        for non_conti in [ASCII, LEAD2, LEAD3, LEAD4].iter() {
            assert_error(&[LEAD4, CONTI, CONTI, *non_conti],
                         &format!("Invalid UTF-8 continuation byte '{:x}'.",
                                  non_conti));
        }
    }
    fn assert_error(bytes: &[u8], expected_error: &str) {
        let utf8_chars = Utf8Chars::new(bytes.iter().copied().map(Ok));
        let error = utf8_chars.map(Result::unwrap_err).next().unwrap();
        assert_eq!(expected_error, error.to_string());
    }
}