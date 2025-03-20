use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum CharPattern {
    Char(char),
    Class(CharClass),
    End,
    Union(Vec<CharPattern>),
}

#[derive(Clone, Copy)]
pub enum CharClass {
    Alphabetic,
    Alphanumeric,
    Ascii,
    AsciiAlphabetic,
    AsciiAlphanumeric,
    AsciiControl,
    AsciiDigit,
    AsciiGraphic,
    AsciiHexDigit,
    AsciiLowercase,
    AsciiPunctuation,
    AsciiUppercase,
    AsciiWhitespace,
    Control,
    Digit { radix: u32 },
    Lowercase,
    Numeric,
    Uppercase,
    Whitespace,
}

impl CharClass {
    pub fn includes(&self, c: char) -> bool {
        match self {
            CharClass::Alphabetic => c.is_alphabetic(),
            CharClass::Alphanumeric => c.is_alphanumeric(),
            CharClass::Ascii => c.is_ascii(),
            CharClass::AsciiAlphabetic => c.is_ascii_alphabetic(),
            CharClass::AsciiAlphanumeric => c.is_ascii_alphanumeric(),
            CharClass::AsciiControl => c.is_ascii_control(),
            CharClass::AsciiDigit => c.is_ascii_digit(),
            CharClass::AsciiGraphic => c.is_ascii_graphic(),
            CharClass::AsciiHexDigit => c.is_ascii_hexdigit(),
            CharClass::AsciiLowercase => c.is_ascii_lowercase(),
            CharClass::AsciiPunctuation => c.is_ascii_punctuation(),
            CharClass::AsciiUppercase => c.is_ascii_uppercase(),
            CharClass::AsciiWhitespace => c.is_ascii_whitespace(),
            CharClass::Control => c.is_control(),
            CharClass::Digit { radix } => c.is_digit(*radix),
            CharClass::Lowercase => c.is_lowercase(),
            CharClass::Numeric => c.is_numeric(),
            CharClass::Uppercase => c.is_uppercase(),
            CharClass::Whitespace => c.is_whitespace(),
        }
    }
}
impl Display for CharClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharClass::Alphabetic => write!(f, "alphabetic"),
            CharClass::Alphanumeric => write!(f, "alphanumeric"),
            CharClass::Ascii => write!(f, "ASCII"),
            CharClass::AsciiAlphabetic => write!(f, "ASCII alphabetic"),
            CharClass::AsciiAlphanumeric => write!(f, "ASCII alphanumeric"),
            CharClass::AsciiControl => write!(f, "ASCII control"),
            CharClass::AsciiDigit => write!(f, "ASCII digit"),
            CharClass::AsciiGraphic => write!(f, "ASCII graphic"),
            CharClass::AsciiHexDigit => write!(f, "ASCII hex digit"),
            CharClass::AsciiLowercase => write!(f, "ASCII lowercase"),
            CharClass::AsciiPunctuation => write!(f, "ASCII punctuation"),
            CharClass::AsciiUppercase => write!(f, "ASCII uppercase"),
            CharClass::AsciiWhitespace => write!(f, "ASCII whitespace"),
            CharClass::Control => write!(f, "control"),
            CharClass::Digit { radix } => write!(f, "digit (base {})", radix),
            CharClass::Lowercase => write!(f, "lowercase"),
            CharClass::Numeric => write!(f, "numeric"),
            CharClass::Uppercase => write!(f, "uppercase"),
            CharClass::Whitespace => write!(f, "whitespace"),
        }
    }
}

impl CharPattern {
    pub fn includes(&self, c: Option<char>) -> bool {
        match (self, c) {
            (CharPattern::Char(match_c), Some(c)) => c == *match_c,
            (CharPattern::Class(class), Some(c)) => class.includes(c),
            (CharPattern::End, None) => true,
            (CharPattern::Union(matches), _) => matches.iter().any(|m| m.includes(c)),
            _ => false,
        }
    }
    pub fn for_char(c: char) -> CharPattern {
        CharPattern::Char(c)
    }
    pub fn for_class(class: CharClass) -> CharPattern {
        CharPattern::Class(class)
    }
    pub fn union(self, rhs: CharPattern) -> CharPattern {
        match (self, rhs) {
            (CharPattern::Union(items1), CharPattern::Union(items2)) => {
                let mut items = items1;
                items.extend(items2);
                CharPattern::Union(items)
            }
            (CharPattern::Union(mut items), item) => {
                items.push(item);
                CharPattern::Union(items)
            }
            (item, CharPattern::Union(mut items)) => {
                items.insert(0, item);
                CharPattern::Union(items)
            }
            (item1, item2) => CharPattern::Union(vec![item1, item2]),
        }
    }
}
impl Display for CharPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharPattern::Char(c) => {
                write!(f, "'")?;
                fmt_char(*c, f)?;
                write!(f, "'")
            }
            CharPattern::Class(class) => {
                write!(f, "{}", class)
            }
            CharPattern::Union(matches) => match matches.len() {
                0 => {
                    write!(f, "none")
                }
                1 => {
                    write!(f, "{}", matches[0])
                }
                _ => {
                    for (i, char_match) in matches.iter().enumerate() {
                        if i == matches.len() - 1 {
                            write!(f, "or {}", char_match)?
                        } else {
                            write!(f, "{}, ", char_match)?;
                        }
                    }
                    Ok(())
                }
            },
            CharPattern::End => {
                write!(f, "end of input")
            }
        }
    }
}

impl Debug for CharClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Debug for CharPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub(crate) fn fmt_char(c: char, f: &mut Formatter<'_>) -> std::fmt::Result {
    match c {
        '\\' => write!(f, "\\\\"),
        '\'' => write!(f, "\\'"),
        '\n' => write!(f, "\\n"),
        '\r' => write!(f, "\\r"),
        '\t' => write!(f, "\\t"),
        _ if c.is_ascii() && c.is_control() => write!(f, "\\x{:02x}", c as u8),
        _ => write!(f, "{}", c),
    }
}
