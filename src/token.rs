use crate::buf::Pos;
use crate::kind::Kind;

struct Token {
    kind: Kind,
    pos: Pos,
    len: usize,
}

impl Token {
    fn new(kind: Kind, pos: Pos, len: usize) -> Token {
        Token { kind, pos, len }
    }
}
