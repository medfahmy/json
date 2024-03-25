#[derive(Debug, PartialEq)]
pub enum TokenType {
    LSq,
    RSq,
    LBr,
    RBr,
    Col,
    Com,
    Str,
    Num,
    Lit,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub pos: usize,
    pub len: usize,
    pub lin: usize,
    pub row: usize,
}

impl Token {
    pub fn from_args(typ: TokenType, pos: usize, len: usize, lin: usize, row: usize) -> Self {
        Self { typ, pos, lin, row, len }
    }
}
