use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Eq, PartialEq, Debug)]
pub enum Statement<'a> {
    LetStatement(LetStatement<'a>),
    ReturnStatement(ReturnStatement<'a>),
}

//ToDo: remove the Options after handling the expressions
#[derive(Eq, PartialEq, Debug)]
pub struct LetStatement<'a> {
    pub token: Token<'a>,
    pub name: Option<Identifier<'a>>,
    pub value: Option<Experession>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ReturnStatement<'a> {
    pub token: Token<'a>,
    pub return_value: Option<Experession>,
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> String {
        match self {
            Statement::ReturnStatement(x) => x.token.literal.clone(),
            Statement::LetStatement(x) => x.token.literal.clone(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Experession {}

#[derive(Eq, PartialEq, Debug)]
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: String,
}

impl Identifier<'_> {
    fn expression_node() {}
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
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

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program { statements: vec![] }
    }
}
