use std::cmp::{max, min, Ordering};
use nom::error::{ErrorKind, ParseError};
use crate::parse::Span;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    i_byte: usize,
    i_col: usize,
    i_line: u32,
}

impl PartialOrd<Self> for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering { self.i_byte.cmp(&other.i_byte) }
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

impl Fragment {
    fn i_byte_end(&self) -> usize {
        self.pos.i_byte + self.string.len()
    }
    fn combine(&self, other: &Fragment) -> Fragment {
        let pos = min(self.pos, other.pos);
        let end = max(self.i_byte_end(), other.i_byte_end());
        let len = end - pos.i_byte;
        let mut string = str::repeat(".", len);
        string.replace_range(self.pos.i_byte..self.i_byte_end(), &self.string);
        string.replace_range(other.pos.i_byte..other.i_byte_end(), &other.string);
        Fragment { string, pos }
    }
}

impl<'a> From<Span<'a>> for Fragment {
    fn from(span: Span<'a>) -> Self {
        let string = String::from(*span.fragment());
        let pos = Pos::from(span);
        Fragment { string, pos }
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

struct PathNode {
    kind: PathPartKind,
    pos: Pos,
    children: Vec<PathNode>,
}

impl PathNode {
    fn new(kind: PathPartKind, pos: Pos, children: Vec<PathNode>) -> Self {
        PathNode { kind, pos, children }
    }
    fn new_leaf(kind: PathPartKind, pos: Pos) -> Self {
        let children: Vec<PathNode> = Vec::new();
        PathNode { kind, pos, children }
    }
}

pub(crate) struct PError {
    fragment: Fragment,
    paths: Vec<PathNode>,
}

impl PError {
    fn new(fragment: Fragment, paths: Vec<PathNode>) -> Self {
        PError { fragment, paths }
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
        let paths = vec!(PathNode::new_leaf(kind, pos));
        PError::new(fragment, paths)
    }

    fn append(input: Span<'a>, kind: ErrorKind, other: Self) -> Self {
        let fragment_new = Fragment::from(input);
        let pos = fragment_new.pos;
        let fragment = fragment_new.combine(&other.fragment);
        let kind = PathPartKind::from(kind);
        let paths = vec!(PathNode::new(kind, pos, other.paths));
        PError::new(fragment, paths)
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        let fragment = Fragment::from(input);
        let kind = PathPartKind::from(c);
        let pos = fragment.pos;
        let paths = vec!(PathNode::new_leaf(kind, pos));
        PError::new(fragment, paths)
    }

    fn or(self, other: Self) -> Self {
        let PError { fragment: frag_self, paths: mut paths_self } = self;
        let PError { fragment: frag_other, paths: mut paths_other } = other;
        let fragment = frag_self.combine(&frag_other);
        paths_self.append(&mut paths_other);
        let paths = paths_self;
        PError::new(fragment, paths)
    }
}