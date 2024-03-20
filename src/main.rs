use json::Lexer;

fn main() {
    let input = "\"hello\": -";
    let result = Lexer::lex(input);

    println!("{:?}", result);
}
