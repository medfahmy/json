use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::value::{Value, ValueType};

pub struct Parser<'a> {
    input: &'a str,
    lexer: Lexer<'a>,
    curr: Option<Token>,
    peek: Option<Token>,
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
        while let Some(curr) = self.curr.as_ref() {
            match curr.typ {
                TokenType::LSq => return self.parse_obj(),
                _ => todo!(),
            }
        }

        Ok(Value::from_args(self.input, ValueType::Null, 0, 0))
    }


    fn parse_obj(&mut self) -> Result<Value<'a>, String> {
        todo!()
    }
}

