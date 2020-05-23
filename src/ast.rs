use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Eq, PartialEq, Debug)]
pub enum Statement<'a> {
    LetStatement(LetStatement<'a>),
    ReturnStatement(ReturnStatement<'a>),
    ExpressionStatement(ExpressionStatement<'a>),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
}

//ToDo: remove the Options after handling the expressions
#[derive(Eq, PartialEq, Debug)]
pub struct LetStatement<'a> {
    pub token: Token<'a>,
    pub name: Option<Identifier<'a>>,
    pub value: Option<Expression<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ReturnStatement<'a> {
    pub token: Token<'a>,
    pub return_value: Option<Expression<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExpressionStatement<'a> {
    pub token: Token<'a>,
    pub expression: Expression<'a>,
}

impl Node for Statement<'_> {
    fn token_literal(&self) -> String {
        match self {
            Statement::ReturnStatement(x) => x.token.literal.clone(),
            Statement::LetStatement(x) => x.token.literal.clone(),
            Statement::ExpressionStatement(x) => x.token.literal.clone(),
        }
    }
    fn string(&self) -> String {
        match self {
            Statement::ReturnStatement(x) => {
                let mut out = String::new();
                &out.push_str(&self.token_literal());
                &out.push_str(" ");

                if x.return_value.is_some() {
                    let expression = x.return_value.as_ref().unwrap();
                    match expression {
                        Expression::Identifier(x) => out.push_str(&x.value),
                    }
                }
                &out.push_str(";");
                out
            }
            Statement::LetStatement(x) => {
                let mut out = String::new();
                &out.push_str(&self.token_literal());
                &out.push_str(" ");
                &out.push_str(&x.name.as_ref().unwrap().value);
                &out.push_str(" = ");

                if x.value.is_some() {
                    let expression = x.value.as_ref().unwrap();
                    match expression {
                        Expression::Identifier(x) => out.push_str(&x.value),
                    }
                }
                &out.push_str(";");
                out
            }
            Statement::ExpressionStatement(x) => match &x.expression {
                Expression::Identifier(x) => x.value.clone(),
            },
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: String,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.clone()
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
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            &out.push_str(&s.string());
        }
        out
    }
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program { statements: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::LetStatement(LetStatement {
                token: Token {
                    ttype: LET,
                    literal: String::from("let"),
                },
                name: Some(Identifier {
                    token: Token {
                        ttype: IDENT,
                        literal: String::from("myVar"),
                    },
                    value: String::from("myVar"),
                }),
                value: Some(Expression::Identifier(Identifier {
                    token: Token {
                        ttype: IDENT,
                        literal: String::from("anotherVar"),
                    },
                    value: String::from("anotherVar"),
                })),
            })],
        };

        assert_eq!(
            program.string(),
            String::from("let myVar = anotherVar;"),
            "program.String() wrong. got={}",
            program.string()
        )
    }
}
