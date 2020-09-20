use std::io::{self, Read};

mod parser;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("\nInput: '{}'", buffer);
    let expr = parser::parse(&buffer);

    if expr.is_ok() {
        println!("\nResult: {}", expr.unwrap());
    } else {
        println!("\nError: {}", expr.unwrap_err());
    }
}
