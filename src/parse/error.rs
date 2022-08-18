use nom::error::{ErrorKind, ParseError};
use crate::parse::Span;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    i_byte: usize,
    i_col: usize,
    i_line: u32,
}

impl<'a> From<Span<'a>> for Pos {
    fn from(span: Span<'a>) -> Self {
        let i_byte = span.location_offset();
        let i_col = span.get_utf8_column();
        let i_line = span.location_line();
        Pos { i_byte, i_col, i_line }
    }
}

struct Fragment {
    string: String,
    pos: Pos,
}

impl<'a> From<Span<'a>> for Fragment {
    fn from(span: Span<'a>) -> Self {
        let string = String::from(*span.fragment());
        let pos = Pos::from(span);
        Fragment { string, pos}
    }
}

enum PathPartKind {
    Char(char),
    Kind(ErrorKind),
}

impl From<ErrorKind> for PathPartKind {
    fn from(kind: ErrorKind) -> Self { PathPartKind::Kind(kind) }
}

impl From<char> for PathPartKind {
    fn from(c: char) -> Self { PathPartKind::Char(c) }
}

struct PathPart {
    kind: PathPartKind,
    pos: Pos,
}

impl PathPart {
    fn new(kind: PathPartKind, pos: Pos) -> Self {
        PathPart { kind, pos }
    }
}

struct PathEntry {
    parts: Vec<PathPart>,
}

impl PathEntry {
    fn new(kind: PathPartKind, pos: Pos) -> Self {
        let parts = vec!(PathPart::new(kind, pos));
        PathEntry { parts }
    }
    fn pos_last(&self) -> Pos {
        self.parts.first().unwrap().pos
    }
}

pub(crate) struct PError {
    fragment: Fragment,
    path_entries: Vec<PathEntry>,
}

impl PError {
    fn new(fragment: Fragment, path_entry: PathEntry) -> Self {
        let path_entries = vec!(path_entry);
        PError { fragment, path_entries }
    }
    pub(crate) fn create_report(&self) -> String {
        todo!()
    }
}

impl<'a> ParseError<Span<'a>> for PError {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        let fragment = Fragment::from(input);
        let kind = PathPartKind::from(kind);
        let pos = fragment.pos;
        let path_entry = PathEntry::new(kind, pos);
        PError::new(fragment, path_entry)
    }

    fn append(input: Span<'a>, kind: ErrorKind, other: Self) -> Self {
        todo!()
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        let fragment = Fragment::from(input);
        let kind = PathPartKind::from(c);
        let pos = fragment.pos;
        let path_entry = PathEntry::new(kind, pos);
        PError::new(fragment, path_entry)
    }

    fn or(self, other: Self) -> Self {
        todo!()
    }
}