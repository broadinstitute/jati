use crate::bytes::Bytes;
use crate::error::Error;
use crate::utf8::Utf8Chars;
use std::io::Read;
use std::iter::Map;
use std::slice::Iter;

type Utf8FromSlice<'a> = Utf8Chars<Map<Iter<'a, u8>, fn(&u8) -> Result<u8, Error>>>;
type Utf8FromRead = Utf8Chars<Bytes<Box<dyn Read>>>;
#[derive(Clone)]
pub enum Chars<'a> {
    Str(std::str::Chars<'a>),
    Bytes(Utf8FromSlice<'a>),
    Read(Utf8FromRead),
}

impl Iterator for Chars<'_> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Chars::Str(chars) => chars.next().map(Ok),
            Chars::Bytes(chars) => chars.next(),
            Chars::Read(chars) => chars.next(),
        }
    }
}
impl<'a> From<&'a str> for Chars<'a> {
    fn from(s: &'a str) -> Self {
        Chars::Str(s.chars())
    }
}

impl<'a> From<&'a [u8]> for Chars<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Chars::Bytes(Utf8Chars::new(bytes.iter().map(|&b| Ok(b))))
    }
}
