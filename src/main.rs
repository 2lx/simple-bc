use calculator::Calculator;
use std::io::{self, Write};

mod calculator;
mod parser;

fn main() {
    let mut calc = Calculator::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let trimmed = buffer.trim();
        if trimmed == "quit" {
            break;
        }

        match parser::parse(&buffer) {
            Ok(nodes) => {
                // println!("Parsed: {}", nodes);
                // println!("Tokens: {:?}", nodes);
                println!("{}", calc.calculate(&nodes));
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
