use calculator::Calculator;
use parser::nodes::{Cmd, Statement, Statements};
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

        match parser::parse(&buffer) {
            Ok(nodes) => {
                let Statements(stts) = nodes;
                for stt in stts.iter() {
                    match stt {
                        Statement::Command(Cmd::Quit(_)) => return,
                        Statement::Command(Cmd::PrintVars(_)) => calc.print_vars(),
                        Statement::NodeTree(nodetree) => calc.process_statement(&nodetree),
                    }
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
