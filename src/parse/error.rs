use crate::error::ErrorKind;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

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

enum PathPartKind {
    Char(char),
    Kind(ErrorKind),
    Context(String),
}

impl PathPartKind {
    fn get_string(&self) -> String {
        match self {
            PathPartKind::Char(c) => { String::from(*c) }
            PathPartKind::Kind(kind) => { kind.to_string() }
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

pub struct PError {
    lines: Lines,
    node: PathNode,
}

impl PError {
    fn get_labels(&self) -> LabelsByLine {
        let mut labels = LabelsByLine::new();
        labels.append(self.node.get_labels());
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
    const HAVE_PREFIX: &'static str = "Have: ";
    const NEED_PREFIX: &'static str = "Need: ";
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
        line = format!("{}{}", PError::NEED_PREFIX, line);
        line
    }
    fn create_line_report(line: &str, labels: Vec<Label>) -> String {
        let mut label_lines =
            PError::arrange_labels(labels)
                .into_iter()
                .map(PError::labels_to_line)
                .collect();
        let mut lines = vec!(format!("{}{}", PError::HAVE_PREFIX, line));
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

impl Display for PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}", self.create_report())
    }
}

impl Debug for PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}", self.create_report())
    }
}

impl std::error::Error for PError {}