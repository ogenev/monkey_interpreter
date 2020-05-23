use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

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

    fn prefix_fn(&mut self) -> Option<Expression<'a>> {
        match self.cur_token.ttype {
            TokenType::IDENT(_) => Some(self.parse_identifier()),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Expression<'a> {
        Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_statement(&mut self) -> Option<Statement<'a>> {
        match self.cur_token.ttype {
            LET => self.parse_let_statement(),
            RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement<'a>> {
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
        Some(Statement::LetStatement(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Statement<'a>> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            return_value: None,
        };
        self.next_token();

        // TODO: We're skipping the expressions until we encounter a semicolon

        while self.cur_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Statement::ReturnStatement(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement<'a>> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(Precedence::LOWEST).unwrap(),
        };

        if self.peek_token_is(&SEMICOLON) {
            self.next_token();
        }
        Some(Statement::ExpressionStatement(stmt))
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Expression<'a>> {
        let prefix = self.prefix_fn();
        prefix
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
    fn test_let_statements() {
        let input = r"let x = 5;
                          let y = 10;
                          let foobar = 838383;";
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
            let stmt = match &program.statements[i] {
                Statement::LetStatement(x) => x,
                _ => panic!("Expected letStatement, found {:?}", &program.statements[1]),
            };

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

    #[test]
    fn test_return_statements() {
        let input = r"
                         return 5;
                         return 10;
                         return 993322
                         ";

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

        for stmt in &program.statements {
            match stmt {
                Statement::LetStatement(x) => panic!("stmt not ast.ReturnStatement, got={:?}", x),
                Statement::ReturnStatement(x) => assert_eq!(
                    x.token.literal,
                    String::from("return"),
                    "returnStmt.TokenLiteral not 'return', got {}",
                    x.token.literal
                ),
                _ => (),
            }
        }
    }

    #[test]
    fn identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parse_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "Program.statements does not contain 1 statements got={}",
            program.statements.len()
        );

        let stmt = program.statements.iter().nth(0).unwrap();

        match stmt {
            Statement::ExpressionStatement(x) => match &x.expression {
                Expression::Identifier(y) => {
                    if y.value != String::from("foobar") {
                        panic!("ident.Value not {}. got={}", "foobar", y.value)
                    } else if y.token.literal != String::from("foobar") {
                        panic!(
                            "ident.TokenLiteral not {}. got={}",
                            "foobar", y.token.literal
                        )
                    }
                }
                _ => panic!("exp not *ast.Identifier. got={:?}", x.expression),
            },
            _ => panic!(
                "program.Statements[0] is not ast.ExpressionStatement. got={:?}",
                stmt
            ),
        };
    }
}
