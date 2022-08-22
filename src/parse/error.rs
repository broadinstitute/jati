use std::cmp::Ordering;
use std::collections::HashMap;
use nom::error::{ContextError, ErrorKind, ParseError};
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

enum PathPartKind {
    Char(char),
    Kind(ErrorKind),
    Context(String)
}

impl PathPartKind {
    fn get_string(&self) -> String {
        match self {
            PathPartKind::Char(c) => { String::from(*c) }
            PathPartKind::Kind(kind) => { String::from(kind.description()) }
            PathPartKind::Context(string) => { string.clone() }
        }
    }
}

impl From<ErrorKind> for PathPartKind {
    fn from(kind: ErrorKind) -> Self { PathPartKind::Kind(kind) }
}

impl From<char> for PathPartKind {
    fn from(c: char) -> Self { PathPartKind::Char(c) }
}

impl From<String> for PathPartKind {
    fn from(string: String) -> Self { PathPartKind::Context(string) }
}

impl From<&str> for PathPartKind {
    fn from(string: &str) -> Self { PathPartKind::Context(String::from(string)) }
}

struct PathNode {
    kind: PathPartKind,
    pos: Pos,
    children: Vec<PathNode>,
}

struct Label {
    i_col: usize,
    string: String,
}

impl PathNode {
    fn new(kind: PathPartKind, pos: Pos, children: Vec<PathNode>) -> Self {
        PathNode { kind, pos, children }
    }
    fn new_leaf(kind: PathPartKind, pos: Pos) -> Self {
        let children: Vec<PathNode> = Vec::new();
        PathNode { kind, pos, children }
    }
    fn get_label(&self) -> Label {
        let i_col = self.pos.i_col;
        let string = self.kind.get_string();
        Label { i_col, string }
    }
    fn get_labels(&self) -> Vec<Label> {
        let mut labels: Vec<Label> = Vec::new();
        for child in &self.children {
            labels.append(&mut child.get_labels());
        }
        labels.push(self.get_label());
        labels
    }
}

type Lines = HashMap<u32, String>;

pub(crate) struct PError {
    lines: Lines,
    paths: Vec<PathNode>,
}

impl PError {
    fn new(lines: Lines, paths: Vec<PathNode>) -> Self { PError { lines, paths } }
    fn from_single(i_line: u32, line: String, node: PathNode) -> Self {
        let mut lines: Lines = HashMap::new();
        lines.insert(i_line, line);
        let paths = vec!(node);
        Self::new(lines, paths)
    }
    fn append_node(input: Span, kind: PathPartKind, other: Self) -> Self {
        let PError { lines: mut other_lines, paths: other_paths } = other;
        let pos = Pos::from(input);
        let i_line = pos.i_line;
        other_lines.entry(i_line).or_insert_with(|| { get_line(input) });
        let lines = other_lines;
        let paths = vec!(PathNode::new(kind, pos, other_paths));
        PError::new(lines, paths)
    }
    pub(crate) fn create_report(&self) -> String {
        todo!()
    }
}

fn get_line(span: Span) -> String {
    String::from_utf8_lossy(span.get_line_beginning()).to_string()
}

impl<'a> ParseError<Span<'a>> for PError {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        let line = get_line(input);
        let kind = PathPartKind::from(kind);
        let pos = Pos::from(input);
        let i_line = pos.i_line;
        let path = PathNode::new_leaf(kind, pos);
        PError::from_single(i_line, line, path)
    }

    fn append(input: Span<'a>, kind: ErrorKind, other: Self) -> Self {
        let kind = PathPartKind::from(kind);
        PError::append_node(input, kind, other)
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        let line = get_line(input);
        let kind = PathPartKind::from(c);
        let pos = Pos::from(input);
        let i_line = pos.i_line;
        let path = PathNode::new_leaf(kind, pos);
        PError::from_single(i_line, line, path)
    }

    fn or(self, other: Self) -> Self {
        let PError { lines: mut lines_self, paths: mut paths_self } = self;
        let PError { lines: lines_other, paths: mut paths_other } = other;
        for (i_line, line) in lines_other {
            lines_self.insert(i_line, line);
        }
        let lines = lines_self;
        paths_self.append(&mut paths_other);
        let paths = paths_self;
        PError::new(lines, paths)
    }
}

impl<'a> ContextError<Span<'a>> for PError {
    fn add_context(input: Span<'a>, ctx: &'static str, other: Self) -> Self {
        let kind = PathPartKind::from(ctx);
        PError::append_node(input, kind, other)
    }
}
