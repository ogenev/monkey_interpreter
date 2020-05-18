use crate::ast;
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::*;

pub struct Parser<'a> {
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
    pub fn parse_program(&mut self) -> ast::Program<'a> {
        let mut program = Program::new();

        while self.cur_token.ttype != EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }
        program
    }
    fn parse_statement(&mut self) -> Option<ast::LetStatement<'a>> {
        match self.cur_token.ttype {
            LET => self.parse_let_statement(),
            _ => None,
        }
    }
    fn parse_let_statement(&mut self) -> Option<ast::LetStatement<'a>> {
        let stmt = ast::LetStatement {
            token: self.cur_token.clone(),
            name: None,
            value: None,
        };
        if self.expect_peek(ASSIGN) {
            return None;
        }
        // TODO: We're skipping the expressions until we encounter a semicolon
        while self.cur_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(stmt)
    }
    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.ttype == t
    }
    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.ttype == t
    }
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::lexer::Lexer;

    #[test]
    fn let_statements() {
        let input = r"let x = 5;
                          let y = 10;
                          let foobar = 838383;";
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let program = Some(p.parse_progam());
        assert_eq!(program, Some(program), "ParseProgram() returned nil");
    }
}
