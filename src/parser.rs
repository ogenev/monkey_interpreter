use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;

pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Parser<'a> {
        let mut p: Parser = Parser {
            l,
            cur_token: Token::new(),
            peek_token: Token::new(),
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }
    fn errors(&self) -> &Vec<String> {
        &self.errors
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
    pub fn parse_program(&mut self) -> Program<'a> {
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
    fn parse_statement(&mut self) -> Option<LetStatement<'a>> {
        match self.cur_token.ttype {
            LET => self.parse_let_statement(),
            _ => None,
        }
    }
    fn parse_let_statement(&mut self) -> Option<LetStatement<'a>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: None,
            value: None,
        };

        if !self.expect_peek(IDENT) {
            return None;
        }

        stmt.name = Some(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if !self.expect_peek(ASSIGN) {
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
    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.ttype == *t
    }
    fn peek_error(&mut self, t: &TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.ttype
        );
        self.errors.push(msg);
    }
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(&t);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_statements() {
        let input = r"let x 5;
                          let = 10;
                          let 838383;";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parse_errors(p);

        assert_eq!(
            program.statements.len(),
            3,
            "Program.statements does not contain 3 statements got={}",
            program.statements.len()
        );

        struct ExpectedIdentifier<'a> {
            value: &'a str,
        }

        let tests = vec![
            ExpectedIdentifier { value: "x" },
            ExpectedIdentifier { value: "y" },
            ExpectedIdentifier { value: "foobar" },
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];

            assert_eq!(
                stmt.token.literal,
                String::from("let"),
                "s.token.literal not 'let'. got={}",
                stmt.token.literal
            );

            assert_eq!(
                stmt.name.as_ref().unwrap().value.to_string(),
                tt.value,
                "stmt.Name.Value not'{}'.got={}",
                tt.value,
                stmt.name.as_ref().unwrap().value.to_string()
            );

            assert_eq!(
                stmt.name.as_ref().unwrap().token.literal.to_string(),
                tt.value,
                "s.name not '{}'. got={}",
                tt.value,
                stmt.name.as_ref().unwrap().token.literal.to_string(),
            )
        }
    }
    fn check_parse_errors(p: Parser) {
        let errors = p.errors();

        if errors.len() == 0 {
            return;
        }

        for err in errors {
            println!("parser error: {}", err)
        }
        panic!("parser has {} errors", errors.len())
    }
}
