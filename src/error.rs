use crate::pos::Pos;

struct ParseError {
    pos: Pos
}

impl ParseError {
    pub fn message(&self) -> String {
        format!("Error at {}", self.pos)
    }

}