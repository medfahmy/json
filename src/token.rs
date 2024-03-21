#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Str,
    Num,
    Literal,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub row: usize,
    pub len: usize,
}

impl Token {
    pub fn from_args(kind: TokenKind, line: usize, row: usize, len: usize) -> Self {
        Self {
            kind,
            line,
            row,
            len,
        }
    }
}
