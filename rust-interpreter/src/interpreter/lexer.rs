use super::token::Token;

#[derive(Debug, Clone, Copy)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.read_position).unwrap())
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        let token = match self.ch {
            None => Token::Eof,
            Some(ch) => match ch {
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        Token::Equal
                    } else {
                        Token::Assign
                    }
                }
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' => {
                    if self.peek_char() == Some('=') {
                        self.read_char();
                        Token::NotEqual
                    } else {
                        Token::Bang
                    }
                }
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '<' => Token::LessThan,
                '>' => Token::GreaterThan,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                ch if ch.is_alphabetic() => {
                    let literal = self.read_identifier();
                    return Token::from(literal);
                }
                ch if ch.is_numeric() => {
                    let literal = self.read_number();
                    return Token::Integer(&literal);
                }
                _ => Token::Illegal,
            },
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &'a str {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !ch.is_alphabetic() {
                break;
            }
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !ch.is_numeric() {
                break;
            }
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if !ch.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::Eof {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };
    
            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;
            

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 == 10;
            10 != 9;
            "#;

        let mut lexer = Lexer::new(input.into());
        let tests = [
            Token::Let,
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RParen,
            Token::LCurly,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RCurly,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::LParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Integer("5".into()),
            Token::LessThan,
            Token::Integer("10".into()),
            Token::GreaterThan,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Integer("5".into()),
            Token::LessThan,
            Token::Integer("10".into()),
            Token::RParen,
            Token::LCurly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RCurly,
            Token::Else,
            Token::LCurly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RCurly,
            Token::Integer("10".into()),
            Token::Equal,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Integer("10".into()),
            Token::NotEqual,
            Token::Integer("9".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        for (i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                &tok, tt,
                "tests[{}] - token type wrong. expected={:#?}, got={:#?}",
                i, tt, tok
            );
        }
    }
}
