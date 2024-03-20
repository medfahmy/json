use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    next: Option<Token>,
}