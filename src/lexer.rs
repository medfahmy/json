use crate::token::{Token, TokenKind::*};

pub struct Lexer<'a> {
    input: &'a str,
    line: usize,
    row: usize,
}

impl<'a> Lexer<'a> {
    pub fn lex(input: &'a str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        if input.is_empty() {
            return Ok(tokens);
        }

        let mut lexer = Lexer {
            input,
            line: 0,
            row: 0,
        };

        loop {
            match lexer.next_token()? {
                Some(token) => tokens.push(token),
                None => return Ok(tokens),
            }
        }
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        let curr = match self.curr() {
            Some(curr) => curr,
            None => return Ok(None),
        };

        let token = match curr {
            '{' => Token::from_args(LBrace, self.line, self.row, 1),
            '}' => Token::from_args(RBrace, self.line, self.row, 1),
            '[' => Token::from_args(LBracket, self.line, self.row, 1),
            ']' => Token::from_args(RBracket, self.line, self.row, 1),
            ':' => Token::from_args(Colon, self.line, self.row, 1),
            ',' => Token::from_args(Comma, self.line, self.row, 1),
            't' | 'f' | 'n' => {
                return Ok(Some(self.read_literal()));
            }
            '"' => {
                return Ok(Some(self.read_string()));
            }
            ch if ch.is_digit(10) || ch == '.' => {
                return Ok(Some(self.read_number()));
            }
            ch => {
                return Err(format!(
                    "lexer: invalid character {} at {}:{}",
                    ch, self.line, self.row
                ))
            }
        };

        self.read_char();
        Ok(Some(token))
    }

    fn read_literal(&mut self) -> Token {
        let row = self.row;
        let mut len = 0;

        while let Some(curr) = self.curr() {
            if !curr.is_alphabetic() {
                break;
            }

            len += 1;
            self.read_char();
        }

        Token::from_args(Literal, self.line, row, len)
    }

    fn curr(&self) -> Option<char> {
        self.input
            .lines()
            .nth(self.line)
            .and_then(|line| line.chars().nth(self.row))
    }

    fn read_char(&mut self) {
        match self.curr() {
            Some('\n') => self.line += 1,
            Some(_) => self.row += 1,
            None => {},
        }
    }

    fn read_string(&mut self) -> Token {
        self.read_char();
        let row = self.row;

        while let Some(curr) = self.curr() {
            self.read_char();

            if curr == '"' || curr.is_whitespace() {
                break;
            }
        }

        Token::from_args(Str, self.line, row, self.row - row - 1)
    }

    fn read_number(&mut self) -> Token {
        let row = self.row;

        while let Some(curr) = self.curr() {
            if curr.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }

        Token::from_args(Num, self.line, row, self.row - row)
    }

    fn skip_whitespace(&mut self) {
        match self.curr() {
            Some(ch) if ch.is_whitespace() => self.read_char(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::lex(input).unwrap()
    }

    #[test]
    fn number() {
        assert_eq!(lex("123"), vec![Token::from_args(Num, 0, 0, 3)]);
    }

    #[test]
    fn str() {
        assert_eq!(lex("\"hello\""), vec![Token::from_args(Str, 0, 1, 5)]);
    }

    #[test]
    fn literal() {
        assert_eq!(lex("true"), vec![Token::from_args(Literal, 0, 0, 4)]);
        assert_eq!(lex("false"), vec![Token::from_args(Literal, 0, 0, 5)]);
        assert_eq!(lex("null"), vec![Token::from_args(Literal, 0, 0, 4)]);
        assert_eq!(lex("{ \"id\": true }"), vec![
            //          01 234 567890123
            Token::from_args(LBrace, 0, 0, 1),
            Token::from_args(Str, 0, 3, 2),
            Token::from_args(Colon, 0, 6, 1),
            Token::from_args(Literal, 0, 8, 4),
            Token::from_args(RBrace, 0, 13, 1),
        ]);
        assert_eq!(lex("{ \"id\": false }"), vec![
            //          01 234 567890123
            Token::from_args(LBrace, 0, 0, 1),
            Token::from_args(Str, 0, 3, 2),
            Token::from_args(Colon, 0, 6, 1),
            Token::from_args(Literal, 0, 8, 5),
            Token::from_args(RBrace, 0, 14, 1),
        ]);
        assert_eq!(lex("{ \"id\": null }"), vec![
            //          01 234 567890123
            Token::from_args(LBrace, 0, 0, 1),
            Token::from_args(Str, 0, 3, 2),
            Token::from_args(Colon, 0, 6, 1),
            Token::from_args(Literal, 0, 8, 4),
            Token::from_args(RBrace, 0, 13, 1),
        ]);
    }

    #[test]
    fn one() {
        assert_eq!(
            lex(r#"{ "id": 25 }"#),
            // 012345678901
            vec![
                Token::from_args(LBrace, 0, 0, 1),
                Token::from_args(Str, 0, 3, 2),
                Token::from_args(Colon, 0, 6, 1),
                Token::from_args(Num, 0, 8, 2),
                Token::from_args(RBrace, 0, 11, 1),
            ],
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            lex(r#"{ "id": 25, "name": "bob" }"#),
            // 012345678901234567890123456
            vec![
                Token::from_args(LBrace, 0, 0, 1),
                Token::from_args(Str, 0, 3, 2),
                Token::from_args(Colon, 0, 6, 1),
                Token::from_args(Num, 0, 8, 2),
                Token::from_args(Comma, 0, 10, 1),
                Token::from_args(Str, 0, 13, 4),
                Token::from_args(Colon, 0, 18, 1),
                Token::from_args(Str, 0, 21, 3),
                Token::from_args(RBrace, 0, 26, 1),
            ],
        );
    }

    #[test]
    fn three() {
        assert_eq!(
            lex(r#"{ "id": "", "name": 23423 }"#),
            //     012345678901234567890123456
            vec![
                Token::from_args(LBrace, 0, 0, 1),
                Token::from_args(Str, 0, 3, 2),
                Token::from_args(Colon, 0, 6, 1),
                Token::from_args(Str, 0, 9, 0),
                Token::from_args(Comma, 0, 10, 1),
                Token::from_args(Str, 0, 13, 4),
                Token::from_args(Colon, 0, 18, 1),
                Token::from_args(Num, 0, 20, 5),
                Token::from_args(RBrace, 0, 26, 1),
            ],
        );
    }

    #[test]
    fn list() {
        assert_eq!(
            lex(r#"{ "id": 25, "users": ["bob", "alice"] }"#),
            // 012345678901234567890123456789012345678
            vec![
                Token::from_args(LBrace, 0, 0, 1),
                Token::from_args(Str, 0, 3, 2),
                Token::from_args(Colon, 0, 6, 1),
                Token::from_args(Num, 0, 8, 2),
                Token::from_args(Comma, 0, 10, 1),
                Token::from_args(Str, 0, 13, 5),
                Token::from_args(Colon, 0, 19, 1),
                Token::from_args(LBracket, 0, 21, 1),
                Token::from_args(Str, 0, 23, 3),
                Token::from_args(Comma, 0, 27, 1),
                Token::from_args(Str, 0, 30, 5),
                Token::from_args(RBracket, 0, 36, 1),
                Token::from_args(RBrace, 0, 38, 1),
            ],
        );
    }
}
