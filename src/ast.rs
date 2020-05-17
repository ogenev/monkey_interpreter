use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

struct Statement<'a> {
    token: Token<'a>,
    name: Identifier<'a>,
    value: Experession,
}

struct Experession {}

struct Identifier<'a> {
    token: Token<'a>,
    value: String,
}

impl Identifier<'_> {
    fn expression_node() {}
}

pub struct Program<'a> {
    statements: Vec<Statement<'a>>,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for Program<'_> {
    fn token_literal(&self) -> String {
        return if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        };
    }
}
