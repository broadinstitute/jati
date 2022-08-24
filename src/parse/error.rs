use std::cmp::Ordering;
use std::collections::BTreeMap;
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
    Context(String),
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
    description: String,
}

impl Label {
    fn new(i_col: usize, description: String) -> Label {
        Label { i_col, description }
    }
    fn display_string(&self) -> String { format!("|{}", self.description) }
    fn display_len(&self) -> usize { self.description.len() + 1 }
    fn i_col_end(&self) -> usize { self.i_col + self.display_len() }
}

struct LabelsByLine {
    labels: BTreeMap<u32, Vec<Label>>,
}

impl LabelsByLine {
    fn new() -> LabelsByLine {
        let labels = BTreeMap::<u32, Vec<Label>>::new();
        LabelsByLine { labels }
    }
    fn push(&mut self, i_line: u32, label: Label) {
        match self.labels.get_mut(&i_line) {
            None => {
                self.labels.insert(i_line, vec!(label));
            }
            Some(labels_for_line) => {
                labels_for_line.push(label)
            }
        }
    }
    fn append(&mut self, other: Self) {
        for (i_line, mut other_labels) in other.labels.into_iter() {
            match self.labels.get_mut(&i_line) {
                None => {
                    self.labels.insert(i_line, other_labels);
                }
                Some(labels_for_line) => {
                    labels_for_line.append(&mut other_labels)
                }
            }
        }
    }
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
        let description = self.kind.get_string();
        Label::new(i_col, description)
    }
    fn get_labels(&self) -> LabelsByLine {
        let mut labels = LabelsByLine::new();
        for child in &self.children {
            labels.append(child.get_labels());
        }
        labels.push(self.pos.i_line, self.get_label());
        labels
    }
}

type Lines = BTreeMap<u32, String>;

pub(crate) struct PError {
    lines: Lines,
    paths: Vec<PathNode>,
}

impl PError {
    fn new(lines: Lines, paths: Vec<PathNode>) -> Self { PError { lines, paths } }
    fn from_single(i_line: u32, line: String, node: PathNode) -> Self {
        let mut lines = Lines::new();
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
    fn get_labels(&self) -> LabelsByLine {
        let mut labels = LabelsByLine::new();
        for path in &self.paths {
            labels.append(path.get_labels())
        }
        labels
    }
    fn fits_in(label: &Label, labels: &[Label]) -> bool {
        for other_label in labels {
            if label.i_col <= other_label.i_col_end() + 1 ||
                other_label.i_col <= label.i_col_end() + 1 {
                return false;
            }
        }
        true
    }
    fn arrange_labels(labels: Vec<Label>) -> Vec<Vec<Label>> {
        let mut lines = Vec::<Vec<Label>>::new();
        for label in labels {
            match lines.iter_mut()
                .find(|labels| { PError::fits_in(&label, labels) }) {
                None => { lines.push(vec!(label)); }
                Some(labels) => { labels.push(label) }
            }
        }
        lines
    }
    fn labels_to_line(labels: Vec<Label>) -> String {
        let mut labels = labels;
        labels.sort_by_key(|label| { label.i_col });
        let mut line = String::new();
        for label in labels {
            if line.len() < label.i_col - 1 {
                line.push_str(&" ".repeat(label.i_col - line.len() - 1))
            }
            line.push_str(&label.display_string())
        }
        line
    }
    fn create_line_report(line: &str, labels: Vec<Label>) -> String {
        let mut label_lines =
            PError::arrange_labels(labels)
                .into_iter()
                .map(PError::labels_to_line)
                .collect();
        let mut lines = vec!(String::from(line));
        lines.append(&mut label_lines);
        lines.join("\n")
    }
    pub(crate) fn create_report(&self) -> String {
        let mut labels = self.get_labels();
        let mut report = String::new();
        for (i_line, line) in self.lines.iter() {
            if let Some(labels_for_line) = labels.labels.remove(i_line) {
                report.push_str(&PError::create_line_report(line, labels_for_line))
            }
        }
        report
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
