use crate::token::*;
use std::convert::TryInto;

struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    // Read the current character
    fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    // Skip the whitespace characters in the input
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    // Main method which return the next token from the input.
    fn next_token(&mut self) -> Token {
        self.skip_whitespace(); // We need to skip the whitespace and the new lines from the input

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut tok = self.new_token(EQ, ch);
                    let mut new_literal = ch.to_string();
                    new_literal.push(self.ch);
                    tok.literal = new_literal;
                    self.read_char();
                    tok
                } else {
                    let tok = self.new_token(ASSIGN, self.ch);
                    self.read_char();
                    tok
                }
            }
            ';' => {
                let tok = self.new_token(SEMICOLON, self.ch);
                self.read_char();
                tok
            }
            '(' => {
                let tok = self.new_token(LPAREN, self.ch);
                self.read_char();
                tok
            }
            ')' => {
                let tok = self.new_token(RPAREN, self.ch);
                self.read_char();
                tok
            }
            ',' => {
                let tok = self.new_token(COMMA, self.ch);
                self.read_char();
                tok
            }
            '+' => {
                let tok = self.new_token(PLUS, self.ch);
                self.read_char();
                tok
            }
            '*' => {
                let tok = self.new_token(ASTERISK, self.ch);
                self.read_char();
                tok
            }
            '-' => {
                let tok = self.new_token(MINUS, self.ch);
                self.read_char();
                tok
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut tok = self.new_token(NOTEQ, ch);
                    let mut new_literal = ch.to_string();
                    new_literal.push(self.ch);
                    tok.literal = new_literal;
                    self.read_char();
                    tok
                } else {
                    let tok = self.new_token(BANG, self.ch);
                    self.read_char();
                    tok
                }
            }
            '/' => {
                let tok = self.new_token(SLASH, self.ch);
                self.read_char();
                tok
            }
            '<' => {
                let tok = self.new_token(LT, self.ch);
                self.read_char();
                tok
            }
            '>' => {
                let tok = self.new_token(GT, self.ch);
                self.read_char();
                tok
            }
            '{' => {
                let tok = self.new_token(LBRACE, self.ch);
                self.read_char();
                tok
            }
            '}' => {
                let tok = self.new_token(RBRACE, self.ch);
                self.read_char();
                tok
            }
            // This happens when there is no more characters i.e. end of the input
            '\0' => Token {
                ttype: EOF,
                literal: String::from(""),
            },
            _ => {
                let mut tok = Token::new();

                if self.ch.is_alphabetic() {
                    // Some words are specific for the language(keywords) and we need to distinguish
                    // that from the identifiers chosen by the user(function names, variables, etc).
                    // We need to lookup every word if it matches any of the keywords
                    tok.literal = self.read_identifier();
                    tok.ttype = Token::lookup_ident(&tok.literal);
                    tok
                } else if self.ch.is_numeric() {
                    // Any consecutive digits(0-9) are matched as single INT token
                    tok.ttype = INT;
                    tok.literal = self.read_number();
                    tok
                } else {
                    // Map any unrecognizable char as illegal
                    self.new_token(ILLEGAL, self.ch)
                }
            }
        }
    }

    // Return a number if consecutive digits(0-9) are found
    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.is_numeric() {
            self.read_char()
        }

        let result = String::from(self.input);
        result[position..self.position].to_string()
    }

    // Underscore is also treated as a letter
    fn is_letter(&self, ch: char) -> bool {
        if ch.is_ascii_alphabetic() || ch == '_' {
            true
        } else {
            false
        }
    }

    // Read consecutive letters and return identifier
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        loop {
            if self.is_letter(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        let result = String::from(self.input);
        result[position..self.position].to_string()
    }

    fn new_token(&self, ttype: TokenType<'a>, ch: char) -> Token<'a> {
        Token {
            ttype,
            literal: ch.to_string(),
        }
    }
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.chars().count() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::*;

    // Testing the monkey language tokens
    #[test]
    fn next_token() {
        let input = r#"let five = 5;
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

        let mut l = Lexer::new(&input);
        let token_types = vec![
            LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT,
            ASSIGN, FUNCTION, LPAREN, IDENT, COMMA, IDENT, RPAREN, LBRACE, IDENT, PLUS, IDENT,
            SEMICOLON, RBRACE, SEMICOLON, LET, IDENT, ASSIGN, IDENT, LPAREN, IDENT, COMMA, IDENT,
            RPAREN, SEMICOLON, BANG, MINUS, SLASH, ASTERISK, INT, SEMICOLON, INT, LT, INT, GT, INT,
            SEMICOLON, IF, LPAREN, INT, LT, INT, RPAREN, LBRACE, RETURN, TRUE, SEMICOLON, RBRACE,
            ELSE, LBRACE, RETURN, FALSE, SEMICOLON, RBRACE, INT, EQ, INT, SEMICOLON, INT, NOTEQ,
            INT, SEMICOLON, EOF
        ];

        for token_type in token_types {
            let tok = l.next_token();
            println!("{:?}", tok.ttype);
            assert_eq!(tok.ttype, token_type)
        }
    }
}
