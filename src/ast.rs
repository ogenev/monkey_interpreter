use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement {}

pub struct LetStatement<'a> {
    pub token: Token<'a>,
    pub name: Option<Identifier<'a>>,
    pub value: Option<Experession>,
}

pub struct Experession {}

pub struct Identifier<'a> {
    token: Token<'a>,
    value: String,
}

impl Identifier<'_> {
    fn expression_node() {}
}

pub struct Program<'a> {
    pub statements: Vec<LetStatement<'a>>,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement<'_> {}

impl Node for Program<'_> {
    fn token_literal(&self) -> String {
        return if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        };
    }
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program { statements: vec![] }
    }
}
