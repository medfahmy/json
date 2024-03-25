use crate::token::{Token, TokenType::*};
use std::process::exit;

pub struct Lexer<'a> {
    inp: &'a str,
    pos: usize,
    lin: usize,
    row: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let curr = match self.curr() {
            Some(curr) => curr,
            None => return None,
        };

        let token = match curr {
            '{' => Token::from_args(LSq, self.pos, 1, self.lin, self.row),
            '}' => Token::from_args(RSq, self.pos, 1, self.lin, self.row),
            '[' => Token::from_args(LBr, self.pos, 1, self.lin, self.row),
            ']' => Token::from_args(RBr, self.pos, 1, self.lin, self.row),
            ':' => Token::from_args(Col, self.pos, 1, self.lin, self.row),
            ',' => Token::from_args(Com, self.pos, 1, self.lin, self.row),
            't' | 'f' | 'n' => {
                return Some(self.read_literal());
            }
            '"' => {
                return Some(self.read_string());
            }
            ch if ch.is_ascii_digit() || ch == '.' => {
                return Some(self.read_number());
            }
            ch => {
                eprintln!(
                    "invalid character {} at {}:{}",
                    ch, self.lin, self.row
                );
                exit(1);
            }
        };

        self.read_char();
        Some(token)
       
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            inp: input,
            pos: 0,
            lin: 0,
            row: 0,
        }
    }

    fn curr(&self) -> Option<char> {
        self.inp.chars().nth(self.pos)
    }

    fn read_char(&mut self) {
        match self.curr() {
            Some('\n') => {
                self.pos += 1;
                self.lin += 1;
                self.row = 0;
            },
            Some(_) => {
                self.pos += 1;
                self.row += 1;
            },
            None => {},
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(curr) = self.curr() {
            if curr.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Token {
        self.read_char();
        let pos = self.pos;
        let row = self.row;

        while let Some(curr) = self.curr() {
            self.read_char();

            if curr == '"' || curr.is_whitespace() {
                break;
            }
        }

        Token::from_args(Str, pos, self.pos - pos - 1, self.lin, row)
    }

    fn read_number(&mut self) -> Token {
        let pos = self.pos;
        let row = self.row;

        while let Some(curr) = self.curr() {
            if curr.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        Token::from_args(Num, pos, self.pos - pos, self.lin, row)
    }

    fn read_literal(&mut self) -> Token {
        let pos = self.pos;
        let row = self.row;
        let mut len = 0;

        while let Some(curr) = self.curr() {
            if !curr.is_alphabetic() {
                break;
            }

            len += 1;
            self.read_char();
        }
        
        match &self.inp[pos..pos + len] {
            "true" | "false" | "null" => {},
            s => {
                eprintln!("invalid literal '{}' at position {}:{}", s, self.lin, row);
                exit(1);
            }
        }

        Token::from_args(Lit, pos, len, self.lin, row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str, expected_tokens: Vec<Token>) {
        let tokens: Vec<_> = Lexer::new(input).collect();
        assert_eq!(tokens, expected_tokens, "input: {}", input);
    }

    #[test]
    fn number() {
        lex("123", vec![Token::from_args(Num, 0, 3, 0, 0)]);
    }

    #[test]
    fn str() {
        lex("\"hello\"", vec![Token::from_args(Str, 1, 5, 0, 1)]);
    }

    #[test]
    fn one_item() {
        lex(
            r#"{ "id": 25 }"#,
            // 012345678901
            vec![
                Token::from_args(LSq, 0, 1, 0, 0),
                Token::from_args(Str, 3, 2, 0, 3),
                Token::from_args(Col, 6, 1, 0, 6),
                Token::from_args(Num, 8, 2, 0, 8),
                Token::from_args(RSq, 11, 1, 0, 11),
            ],
        );
    }

    #[test]
    fn two_items() {
        lex(
            r#"{ "id": 25, "name": "bob" }"#,
            // 012345678901234567890123456
            vec![
                Token::from_args(LSq, 0, 1, 0, 0),
                Token::from_args(Str, 3, 2, 0, 3),
                Token::from_args(Col, 6, 1, 0, 6),
                Token::from_args(Num, 8, 2, 0, 8),
                Token::from_args(Com, 10, 1, 0, 10),
                Token::from_args(Str, 13, 4, 0, 13),
                Token::from_args(Col, 18, 1, 0, 18),
                Token::from_args(Str, 21, 3, 0, 21),
                Token::from_args(RSq, 26, 1, 0, 26),
            ],
        );
    }

    #[test]
    fn empty_string() {
        lex(
            r#"{ "id": "", "name": 23423 }"#,
            //     012345678901234567890123456
            vec![
                Token::from_args(LSq, 0, 1, 0, 0),
                Token::from_args(Str, 3, 2, 0, 3),
                Token::from_args(Col, 6, 1, 0, 6),
                Token::from_args(Str, 9, 0, 0, 9),
                Token::from_args(Com, 10, 1, 0, 10),
                Token::from_args(Str, 13, 4, 0, 13),
                Token::from_args(Col, 18, 1, 0, 18),
                Token::from_args(Num, 20, 5, 0, 20),
                Token::from_args(RSq, 26, 1, 0, 26),
            ],
        );
    }

    #[test]
    fn literal() {
        lex("true", vec![Token::from_args(Lit, 0, 4, 0, 0)]);
        lex("false", vec![Token::from_args(Lit, 0, 5, 0, 0)]);
        lex("null", vec![Token::from_args(Lit, 0, 4, 0, 0)]);
        lex(r#"{ "id": true }"#, vec![
            // 01234567890123
            Token::from_args(LSq, 0, 1, 0, 0),
            Token::from_args(Str, 3, 2, 0, 3),
            Token::from_args(Col, 6, 1, 0, 6),
            Token::from_args(Lit, 8, 4, 0, 8),
            Token::from_args(RSq, 13, 1, 0, 13),
        ]);
        lex(r#"{ "id": false }"#, vec![
            // 012345678901234
            Token::from_args(LSq, 0, 1, 0, 0),
            Token::from_args(Str, 3, 2, 0, 3),
            Token::from_args(Col, 6, 1, 0, 6),
            Token::from_args(Lit, 8, 5, 0, 8),
            Token::from_args(RSq, 14, 1, 0, 14),
        ]);
        lex(r#"{ "id": null }"#, vec![
            // 01234567890123
            Token::from_args(LSq, 0, 1, 0, 0),
            Token::from_args(Str, 3, 2, 0, 3),
            Token::from_args(Col, 6, 1, 0, 6),
            Token::from_args(Lit, 8, 4, 0, 8),
            Token::from_args(RSq, 13, 1, 0, 13),
        ]);
    }

    #[test]
    fn list() {
        lex(
            r#"{ "id": 25, "users": ["bob", "alice"] }"#,
            // 012345678901234567890123456789012345678
            vec![
                Token::from_args(LSq, 0, 1, 0, 0),
                Token::from_args(Str, 3, 2, 0, 3),
                Token::from_args(Col, 6, 1, 0, 6),
                Token::from_args(Num, 8, 2, 0, 8),
                Token::from_args(Com, 10, 1, 0, 10),
                Token::from_args(Str, 13, 5, 0, 13),
                Token::from_args(Col, 19, 1, 0, 19),
                Token::from_args(LBr, 21, 1, 0, 21),
                Token::from_args(Str, 23, 3, 0, 23),
                Token::from_args(Com, 27, 1, 0, 27),
                Token::from_args(Str, 30, 5, 0, 30),
                Token::from_args(RBr, 36, 1, 0, 36),
                Token::from_args(RSq, 38, 1, 0, 38),
            ],
        );
    }
}
