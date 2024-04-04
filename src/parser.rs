use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::value::Value;

pub struct Parser<'a> {
    input: &'a str,
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
    peek: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let curr = lexer.next();
        let peek = lexer.next();

        let mut parser = Self {
            input,
            lexer,
            curr,
            peek,
        };

        parser
    }

    fn next_token(&mut self) {
        self.curr = self.peek.take();
        self.peek = self.lexer.next();
    }

    pub fn parse(&mut self) -> Result<Value<'a>, String> {
        let value = if let Some(curr) = self.curr.as_ref() {
            match curr.typ {
                TokenType::Bool => Value::Bool(curr.val),                
                TokenType::Null => Value::Bool(curr.val),
                TokenType::Lsquirly => return self.parse_obj(),
                _ => todo!(),
            }
        } else {
            Value::Null
        };

        Ok(value)
    }


    fn parse_obj(&mut self) -> Result<Value<'a>, String> {
        todo!()
    }
}

