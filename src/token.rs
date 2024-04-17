#[derive(Debug, PartialEq)]
pub enum TokenType {
    Lsquirly,
    Rsquirly,
    Lbrace,
    Rbrace,
    Colon,
    Comma,
    String,
    Number,
    Literal,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub val: &'a str,
    pub row: usize,
    pub col: usize,
}

impl<'a> Token<'a> {
    pub fn from_args(typ: TokenType, val: &'a str, row: usize, col: usize) -> Self {
        Self { typ, val, row, col }
    }
}
