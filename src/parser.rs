use crate::ast;
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token<'a>,
    peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Parser<'a> {
        let mut p: Parser = Parser {
            l,
            cur_token: Token::new(),
            peek_token: Token::new(),
        };
        p.next_token();
        p.next_token();
        p
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
    fn parse_program() -> ast::Program<'a> {
        todo!()
    }
}
