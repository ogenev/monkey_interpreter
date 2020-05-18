use crate::repl::start;
use std::io::{self, Write};

fn main() {
    io::stdout()
        .write_all(b"Hello! This is the Monkey programming language!\n")
        .unwrap();
    io::stdout()
        .write_all(b"Feel free to type in commands\n")
        .unwrap();
    start()
}

#[cfg(test)]
mod tests {
    use crate::base;
    #[test]
    #[ignore]
    fn repl() {
        base::main();
    }
}
