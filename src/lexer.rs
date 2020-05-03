use crate::token;
use crate::token::Token;
use std::convert::TryInto;

struct Lexer<'a> {
    input: &'a str,
    position: u32,      // current position in input (points to current char)
    read_position: u32, // current reading position in input (after current char)
    ch: char,           // current char under examination
}

impl<'a> Lexer <'a>{
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
    fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() as u32 {
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
    fn next_token(&mut self) -> Token {
        let tok: token::Token = match self.ch {
            '=' => self.new_token(token::ASSIGN, self.ch),
            ';' => self.new_token(token::SEMICOLON, self.ch),
            '(' => self.new_token(token::LPAREN, self.ch),
            ')' => self.new_token(token::RPAREN, self.ch),
            ',' => self.new_token(token::COMMA, self.ch),
            '+' => self.new_token(token::PLUS, self.ch),
            '{' => self.new_token(token::LBRACE, self.ch),
            '}' => self.new_token(token::RBRACE, self.ch),
            '\0' => Token {
                ttype: token::EOF,
                literal: String::from(""),
            },
            _ => self.new_token(token::ILLEGAL, self.ch),
        };
        self.read_char();
        tok
    }
    fn new_token(&self, ttype: &'a str, ch: char) -> token::Token<'a> {
        token::Token {
            ttype,
            literal: ch.to_string(),
        }
    }
}

//todo: Write the lexer test