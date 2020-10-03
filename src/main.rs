mod lexer;
mod parser;
use lexer::tokenize;
use parser::parse;
fn main() {
    println!("Hello, world!");
    // println!("{:?}",parse(tokenize("123123123").as_slice()));
}
