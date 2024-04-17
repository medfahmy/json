use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::value::Value;
use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let curr = lexer.next();

        Self { lexer, curr }
    }

    fn next_token(&mut self) {
        self.curr = self.lexer.next();
    }

    fn error(&self, error: String) {
        eprintln!("parse error: {error}");
        std::process::exit(1);
    }

    pub fn parse(&mut self) -> Value<'a> {
        if let Some(curr) = self.curr.as_ref() {
            match curr.typ {
                TokenType::Literal | TokenType::String | TokenType::Number => {
                    Value::Literal(curr.val)
                }
                TokenType::Lsquirly => self.parse_obj(),
                TokenType::Lbrace => self.parse_list(),
                _ => unreachable!(),
            }
        } else {
            Value::Literal("null")
        }
    }

    fn parse_obj(&mut self) -> Value<'a> {
        let mut map = HashMap::new();
        let pos = self.lexer.pos - 1;
        self.next_token();

        while self.curr.is_some() {
            let key = self.curr.as_ref().unwrap().val;
            self.next_token();

            if let Some(curr) = &self.curr {
                if curr.typ == TokenType::Colon {
                    self.next_token();
                } else {
                    self.error(format!("parse error: expected ':', found {}", curr.val));
                }
            }

            let value = self.parse();
            map.insert(key, value);
            self.next_token();

            if let Some(curr) = &self.curr {
                if curr.typ == TokenType::Comma {
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        Value::Obj {
            slice: &self.lexer.input[pos..self.lexer.pos],
            map,
        }
    }

    fn parse_list(&mut self) -> Value<'a> {
        let mut items = Vec::new();
        let pos = self.lexer.pos - 1;
        self.next_token();

        while self.curr.is_some() {
            let next_item = self.parse();
            items.push(next_item);

            self.next_token();

            if let Some(curr) = &self.curr {
                if curr.typ == TokenType::Comma {
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        Value::List {
            slice: &self.lexer.input[pos..self.lexer.pos],
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Value<'_> {
        let mut parser = Parser::new(input);
        parser.parse()
    }

    #[test]
    fn primitives() {
        assert_eq!(parse("123"), Value::Literal("123"));
        assert_eq!(parse("\"123\""), Value::Literal("123"));
        assert_eq!(parse("false"), Value::Literal("false"));
        assert_eq!(parse("true"), Value::Literal("true"));
        assert_eq!(parse("null"), Value::Literal("null"));
    }

    #[test]
    fn plist() {
        assert_eq!(
            parse("[1, 2, 3]"),
            Value::List {
                slice: "[1, 2, 3]",
                items: vec![
                    Value::Literal("1"),
                    Value::Literal("2"),
                    Value::Literal("3")
                ]
            }
        );
    }

    #[test]
    fn pobj() {
        assert_eq!(
            parse("{ \"a\": false, \"b\": 2, \"c\": null }"),
            Value::Obj {
                slice: "{ \"a\": false, \"b\": 2, \"c\": null }",
                map: vec![
                    ("a", Value::Literal("false")),
                    ("b", Value::Literal("2")),
                    ("c", Value::Literal("null")),
                ]
                .into_iter()
                .collect()
            },
        );
    }
}
