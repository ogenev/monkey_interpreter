pub const ILLEGAL: TokenType = TokenType::ILLEGAL("ILLEGAL");
pub const EOF: TokenType = TokenType::EOF("EOF");
pub const IDENT: TokenType = TokenType::IDENT("IDENT");
pub const INT: TokenType = TokenType::INT("INT");
pub const ASSIGN: TokenType = TokenType::ASSIGN("=");
pub const PLUS: TokenType = TokenType::PLUS("+");
pub const COMMA: TokenType = TokenType::COMMA(",");
pub const SEMICOLON: TokenType = TokenType::SEMICOLON(";");
pub const LPAREN: TokenType = TokenType::LPAREN("(");
pub const RPAREN: TokenType = TokenType::RPARENT(")");
pub const LBRACE: TokenType = TokenType::LBRACE("{");
pub const RBRACE: TokenType = TokenType::RBRACE("}");
pub const FUNCTION: TokenType = TokenType::FUNCTION("FUNCTION");
pub const LET: TokenType = TokenType::LET("LET");

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType<'a> {
    ILLEGAL(&'a str),
    EOF(&'a str),
    // Identifiers + literals
    IDENT(&'a str), // add, foobar, x, y, ..
    INT(&'a str),   // 1343456
    // Operators
    ASSIGN(&'a str),
    PLUS(&'a str),
    // Delimiters
    COMMA(&'a str),
    SEMICOLON(&'a str),
    LPAREN(&'a str),
    RPARENT(&'a str),
    LBRACE(&'a str),
    // Keywords
    RBRACE(&'a str),
    FUNCTION(&'a str),
    LET(&'a str),
}
pub struct Token<'a> {
    pub ttype: TokenType<'a>,
    pub literal: String,
}

impl<'a> Token<'a> {
    pub fn lookup_ident(ident: &String) -> TokenType<'a> {
        match ident.as_str() {
            "fn" => FUNCTION,
            "let" => LET,
            _ => IDENT,
        }
    }
}
