use crate::buf::Pos;

#[derive(Copy, Clone, Eq, PartialEq)]
struct TokenKind {
    h1: usize,
    h2: usize
}

struct Token {
    kind: TokenKind,
    pos: Pos,
    len: usize,
    children: Vec<Token>
}

impl TokenKind {
    fn new(h1: usize, h2: usize) -> TokenKind {
        TokenKind { h1, h2 }
    }
}

impl Token {
    fn new_leaf(kind: TokenKind, pos: Pos, len: usize) -> Token {
        let children: Vec<Token> = Vec::new();
        Token { kind, pos, len, children }
    }
}

impl From<&str> for TokenKind {
    fn from(string: &str) -> Self {
        let mut h1: usize = 0;
        let mut h2: usize = 0;
        for byte in string.bytes() {
            h1 = h1.wrapping_add(byte as usize);
            h2 = h2.wrapping_mul(h2).wrapping_add( byte as usize);
        }
        TokenKind::new(h1, h2)
    }
}
