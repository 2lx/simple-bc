use std::io::{self, Read};
use std::fmt;
use std::collections::HashMap;

use parser::nodes::{Statements, Statement, Node};

mod parser;

struct CalcError;

struct Calculator {
    variables: HashMap<String, i128>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            variables: HashMap::new(),
        }
    }

    fn proc_term(&mut self, term: &Node) -> Result<Option<i128>, CalcError> {
        match term {
            &Node::NumberLiteral(_, n) => Ok(Some(n)),
            &Node::Pi(_) => Ok(Some(3.14159 as i128)),
            &Node::UnaryMinus(_, ref e1) => {
                match self.proc_term(e1) {
                    Ok(Some(n1)) => Ok(Some(-n1)),
                    _ => Err(CalcError),
                }
            },
            &Node::Add(_, ref e1, ref e2) => {
                match (self.proc_term(e1), self.proc_term(e2)) {
                    (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(n1 + n2)),
                    _ => Err(CalcError),
                }
            },
            &Node::Multiply(_, ref e1, ref e2) => {
                match (self.proc_term(e1), self.proc_term(e2)) {
                    (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(n1 * n2)),
                    _ => Err(CalcError),
                }
            },
            &Node::Divide(_, ref e1, ref e2) => {
                match (self.proc_term(e1), self.proc_term(e2)) {
                    (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(n1 / n2)),
                    _ => Err(CalcError),
                }
            },
            &Node::Subtract(_, ref e1, ref e2) => {
                match (self.proc_term(e1), self.proc_term(e2)) {
                    (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(n1 - n2)),
                    _ => Err(CalcError),
                }
            },
            &Node::Power(_, ref e1, ref e2) => {
                match (self.proc_term(e1), self.proc_term(e2)) {
                    (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(n1.pow(n2 as u32))),
                    _ => Err(CalcError),
                }
            },
            _ => Err(CalcError)
        }
    }

    fn calculate(&mut self, nodes: &Statements) -> String {
        let mut result = String::new();
        self.variables.clear();

        let Statements(stts) = nodes;
        for stt in stts.iter() {
            let Statement(term) = stt;
            match self.proc_term(term) {
                Err(_) => fmt::write(&mut result, format_args!("<UNKNOWN ARG>; ")).unwrap(),
                Ok(Some(n)) => fmt::write(&mut result, format_args!("{}; ", n)).unwrap(),
                Ok(None) => {},
            };
        }

        if !result.is_empty() {
            result.pop();
        }
        result
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("\nInput: '{}'", buffer);
    let expr = parser::parse(&buffer);

    if expr.is_ok() {
        let nodes = expr.unwrap();
        let mut calc = Calculator::new();
        println!("Parsed: {}", nodes);
        // println!("Tokens: {:?}", nodes);
        println!("Results: {}", calc.calculate(&nodes));
    } else {
        println!("Error: {}", expr.unwrap_err());
    }
}
