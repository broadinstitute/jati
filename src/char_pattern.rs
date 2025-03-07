use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum CharPattern {
    Char(char),
    Class(CharClass),
    Union(Vec<CharPattern>),
    End
}

#[derive(Clone, Copy)]
pub enum CharClass {
    Alphabetic,
    Alphanumeric,
}

impl CharClass {
    pub fn includes(&self, c: char) -> bool {
        match self {
            CharClass::Alphabetic => c.is_alphabetic(),
            CharClass::Alphanumeric => c.is_alphanumeric(),
        }
    }
}
impl Display for CharClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharClass::Alphabetic => write!(f, "alphabetic"),
            CharClass::Alphanumeric => write!(f, "alphanumeric"),
        }
    }
}

impl CharPattern {
    pub fn includes(&self, c: Option<char>) -> bool {
        match (self, c) {
            (CharPattern::Char(match_c), Some(c)) => c == *match_c,
            (CharPattern::Class(class), Some(c)) => class.includes(c),
            (CharPattern::Union(matches), _) =>
                matches.iter().any(|m| m.includes(c)),
            (CharPattern::End, None) => true,
            _ => false,
        }
    }
    pub fn for_char(c: char) -> CharPattern { CharPattern::Char(c) }
    pub fn for_class(class: CharClass) -> CharPattern { CharPattern::Class(class) }
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
            CharPattern::Class(class) => { write!(f, "{}", class) }
            CharPattern::Union(matches) => {
                match matches.len() {
                    0 => { write!(f, "none") }
                    1 => { write!(f, "{}", matches[0]) }
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
                }
            }
            CharPattern::End => { write!(f, "end of input") }
        }
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