use std::{fs::read_to_string, io::{stdin, stdout, Write}};
use json::Parser;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();
    let path = std::env::args().nth(1).expect("no file path provided");
    let file = read_to_string(path).expect("error opening file");
    let mut parser = Parser::new(&file);
    let value = parser.parse().expect("error parsing json");

    loop {
        print!(">> ");
        let _ = stdout.flush();
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("error reading input");

        println!("{}", value.query(buf));
    }
}
