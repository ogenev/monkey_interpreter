pub const ILLEGAL: TokenType = TokenType::ILLEGAL("ILLEGAL");
pub const EOF: TokenType = TokenType::EOF("EOF");
pub const IDENT: TokenType = TokenType::IDENT("IDENT");
pub const INT: TokenType = TokenType::INT("INT");
pub const ASSIGN: TokenType = TokenType::ASSIGN("=");
pub const EQ: TokenType = TokenType::EQ("==");
pub const NOTEQ: TokenType = TokenType::NOTEQ("!=");
pub const PLUS: TokenType = TokenType::PLUS("+");
pub const MINUS: TokenType = TokenType::MINUS("-");
pub const BANG: TokenType = TokenType::BANG("!");
pub const ASTERISK: TokenType = TokenType::ASTERISK("*");
pub const SLASH: TokenType = TokenType::SLASH("/");
pub const LT: TokenType = TokenType::LT("<");
pub const GT: TokenType = TokenType::GT(">");
pub const COMMA: TokenType = TokenType::COMMA(",");
pub const SEMICOLON: TokenType = TokenType::SEMICOLON(";");
pub const LPAREN: TokenType = TokenType::LPAREN("(");
pub const RPAREN: TokenType = TokenType::RPARENT(")");
pub const LBRACE: TokenType = TokenType::LBRACE("{");
pub const RBRACE: TokenType = TokenType::RBRACE("}");
pub const FUNCTION: TokenType = TokenType::FUNCTION("FUNCTION");
pub const LET: TokenType = TokenType::LET("LET");
pub const TRUE: TokenType = TokenType::TRUE("TRUE");
pub const FALSE: TokenType = TokenType::FALSE("FALSE");
pub const IF: TokenType = TokenType::IF("IF");
pub const ELSE: TokenType = TokenType::ELSE("ELSE");
pub const RETURN: TokenType = TokenType::RETURN("RETURN");

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType<'a> {
    ILLEGAL(&'a str),
    EOF(&'a str),
    // Identifiers + literals
    IDENT(&'a str), // add, foobar, x, y, ..
    INT(&'a str),   // 1343456
    // Operators
    ASSIGN(&'a str),
    EQ(&'a str),
    NOTEQ(&'a str),
    PLUS(&'a str),
    MINUS(&'a str),
    BANG(&'a str),
    ASTERISK(&'a str),
    SLASH(&'a str),
    LT(&'a str),
    GT(&'a str),
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
    TRUE(&'a str),
    FALSE(&'a str),
    IF(&'a str),
    ELSE(&'a str),
    RETURN(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType<'a>,
    pub literal: String,
}

impl<'a> Token<'a> {
    pub fn new() -> Self {
        Token {
            ttype: TokenType::IDENT(""),
            literal: String::from(""),
        }
    }
    pub fn lookup_ident(ident: &String) -> TokenType<'a> {
        match ident.as_str() {
            "fn" => FUNCTION,
            "let" => LET,
            "true" => TRUE,
            "false" => FALSE,
            "if" => IF,
            "else" => ELSE,
            "return" => RETURN,
            _ => IDENT,
        }
    }
}
