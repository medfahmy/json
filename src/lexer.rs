use crate::token::{Token, TokenType, TokenType::*};
use std::process::exit;

pub struct Lexer<'a> {
    pub input: &'a str,
    pub pos: usize,
    pub row: usize,
    pub col: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let curr = match self.curr() {
            Some(curr) => curr,
            None => return None,
        };

        match curr {
            '{' => self.symbol(Lsquirly),
            '}' => self.symbol(Rsquirly),
            '[' => self.symbol(Lbrace),
            ']' => self.symbol(Rbrace),
            ':' => self.symbol(Colon),
            ',' => self.symbol(Comma),
            't' | 'f' | 'n' => Some(self.read_literal()),
            '"' => Some(self.read_string()),
            ch if ch.is_ascii_digit() || ch == '.' => Some(self.read_number()),
            ch => {
                eprintln!("invalid character {} at {}:{}", ch, self.row, self.col);
                exit(1);
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            pos: 0,
            col: 1,
            row: 1,
        }
    }

    fn curr(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn read_char(&mut self) {
        match self.curr() {
            Some('\n') => {
                self.pos += 1;
                self.col = 0;
                self.row += 1;
            }
            Some(_) => {
                self.pos += 1;
                self.col += 1;
            }
            None => {}
        }
    }

    fn symbol(&mut self, typ: TokenType) -> Option<Token<'a>> {
        let token = Token::from_args(typ, &self.input[self.pos..self.pos + 1], self.row, self.col);
        self.read_char();

       Some(token)
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

    fn read_string(&mut self) -> Token<'a> {
        self.read_char();
        let pos = self.pos;
        let col = self.col;

        while let Some(curr) = self.curr() {
            self.read_char();

            if curr == '"' || curr.is_whitespace() {
                break;
            }
        }

        Token::from_args(Str, &self.input[pos..self.pos - 1], self.row, col)
    }

    fn read_number(&mut self) -> Token<'a> {
        let pos = self.pos;
        let col = self.col;

        while let Some(curr) = self.curr() {
            if curr.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        Token::from_args(Num, &self.input[pos..self.pos], self.row, col)
    }

    fn read_literal(&mut self) -> Token<'a> {
        let pos = self.pos;
        let col = self.col;

        while let Some(curr) = self.curr() {
            if !curr.is_alphabetic() {
                break;
            }

            self.read_char();
        }

        let lit = &self.input[pos..self.pos];

        // self.read_char();

        match lit {
            "null" => Token::from_args(Null, lit, self.row, col),
            "true" | "false" => Token::from_args(Bool, lit, self.row, col),
            s => {
                eprintln!("invalid literal '{}' at position {}:{}", s, self.row, col);
                exit(1);
            }
        }
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
    fn symbols() {
        lex("{}[]:,", vec![
            Token::from_args(Lsquirly, "{", 1, 1),
            Token::from_args(Rsquirly, "}", 1, 2),
            Token::from_args(Lbrace, "[", 1, 3),
            Token::from_args(Rbrace, "]", 1, 4),
            Token::from_args(Colon, ":", 1, 5),
            Token::from_args(Comma, ",", 1, 6),
        ]);
    }

    #[test]
    fn number() {
        lex("123", vec![Token::from_args(Num, "123", 1, 1)]);
    }

    #[test]
    fn str() {
        lex("\"hello\"", vec![Token::from_args(Str, "hello", 1, 2)]);
        lex("\"world\"", vec![Token::from_args(Str, "world", 1, 2)]);
    }

    #[test]
    fn one_item() {
        lex(
            r#"{ "id": 25 }"#,
            // 123456789012345678
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Num, "25", 1, 9),
                Token::from_args(Rsquirly, "}", 1, 12),
            ],
        );
    }

    #[test]
    fn two_items() {
        lex(
            r#"{ "id": 25, "name": "bob" }"#,
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Num, "25", 1, 9),
                Token::from_args(Comma, ",", 1, 11),
                Token::from_args(Str, "name", 1, 14),
                Token::from_args(Colon, ":", 1, 19),
                Token::from_args(Str, "bob", 1, 22),
                Token::from_args(Rsquirly, "}", 1, 27),
            ],
        );
    }

    #[test]
    fn empty_string() {
        lex(
            r#"{ "id": "", "name": 23423 }"#,
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Str, "", 1, 10),
                Token::from_args(Comma, ",", 1, 11),
                Token::from_args(Str, "name", 1, 14),
                Token::from_args(Colon, ":", 1, 19),
                Token::from_args(Num, "23423", 1, 21),
                Token::from_args(Rsquirly, "}", 1, 27),
            ],
        );
    }

    #[test]
    fn literal() {
        lex("true", vec![Token::from_args(Bool, "true", 1, 1)]);
        lex("false", vec![Token::from_args(Bool, "false", 1, 1)]);
        lex("null", vec![Token::from_args(Null, "null", 1, 1)]);

        lex(
            r#"{ "id": true }"#,
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Bool, "true", 1, 9),
                Token::from_args(Rsquirly, "}", 1, 14),
            ],
        );

        lex(
            r#"{ "id": false }"#,
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Bool, "false", 1, 9),
                Token::from_args(Rsquirly, "}", 1, 15),
            ],
        );

        lex(
            r#"{ "id": null }"#,
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Null, "null", 1, 9),
                Token::from_args(Rsquirly, "}", 1, 14),
            ],
        );
    }

    #[test]
    fn llist() {
        lex(
            r#"{ "id": 25, "users": ["bob", "alice"] }"#,
            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "id", 1, 4),
                Token::from_args(Colon, ":", 1, 7),
                Token::from_args(Num, "25", 1, 9),
                Token::from_args(Comma, ",", 1, 11),
                Token::from_args(Str, "users", 1, 14),
                Token::from_args(Colon, ":", 1, 20),
                Token::from_args(Lbrace, "[", 1, 22),
                Token::from_args(Str, "bob", 1, 24),
                Token::from_args(Comma, ",", 1, 28),
                Token::from_args(Str, "alice", 1, 31),
                Token::from_args(Rbrace, "]", 1, 37),
                Token::from_args(Rsquirly, "}", 1, 39),
            ],
        );
    }

    #[test]
    fn lobj() {
        lex(
            r#"{ "a": false, "b": 2, "c": null }"#,
          //   123456789012345678901234567890123456789

            vec![
                Token::from_args(Lsquirly, "{", 1, 1),
                Token::from_args(Str, "a", 1, 4),
                Token::from_args(Colon, ":", 1, 6),
                Token::from_args(Bool, "false", 1, 8),
                Token::from_args(Comma, ",", 1, 13),
                Token::from_args(Str, "b", 1, 16),
                Token::from_args(Colon, ":", 1, 18),
                Token::from_args(Num, "2", 1, 20),
                Token::from_args(Comma, ",", 1, 21),
                Token::from_args(Str, "c", 1, 24),
                Token::from_args(Colon, ":", 1, 26),
                Token::from_args(Null, "null", 1, 28),
                Token::from_args(Rsquirly, "}", 1, 33),
            ],
        );
    }
}