use json::Parser;
use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!(">> ");
        let _ = stdout.flush();
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("error reading input");

        let mut parser = Parser::new(&buf);
        let value = parser.parse();
        println!("{:?}", value);
    }
}
