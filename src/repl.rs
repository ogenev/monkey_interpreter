use crate::lexer::Lexer;
use crate::token::*;
use std::io::{self, Write};

const PROMPT: &str = ">>";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(stdout, "{}", PROMPT).unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut l = Lexer::new(&input[..]);

        loop {
            let tok = l.next_token();
            if tok.ttype == EOF {
                break;
            }
            write!(stdout, "{:?}\n", tok).unwrap();
            stdout.flush().unwrap();
        }
    }
}
