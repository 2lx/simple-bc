use std::collections::HashMap;
use std::fmt;

use crate::parser::nodes::{Node, Statement, Statements};

struct CalcError;

pub struct Calculator {
    variables: HashMap<String, i128>,
}

type CalcResult = Result<Option<i128>, CalcError>;

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            variables: HashMap::new(),
        }
    }

    fn unary_op(&mut self, l: &Node, f: &dyn Fn(i128) -> i128) -> CalcResult {
        match self.process_term(l) {
            Ok(Some(r)) => Ok(Some(f(r))),
            _ => Err(CalcError),
        }
    }

    fn binary_op(&mut self, l: &Node, r: &Node, f: &dyn Fn(i128, i128) -> i128) -> CalcResult {
        match (self.process_term(l), self.process_term(r)) {
            (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(f(n1, n2))),
            _ => Err(CalcError),
        }
    }

    fn assign_value(&mut self, var: &Node, val: &Node) -> CalcResult {
        match (var, self.process_term(val)) {
            (&Node::Variable(_, ref name), Ok(Some(n))) => {
                *self.variables.entry(name.to_string()).or_insert(n) = n;
                Ok(None)
            }
            _ => Err(CalcError),
        }
    }

    fn process_term(&mut self, term: &Node) -> CalcResult {
        match term {
            &Node::NumberLiteral(_, n) => Ok(Some(n)),
            &Node::Pi(_) => Ok(Some(3.14159 as i128)),
            &Node::UnaryMinus(_, ref e1) => self.unary_op(e1, &|l| -l),
            &Node::Add(_, ref e1, ref e2) => self.binary_op(e1, e2, &|l, r| l + r),
            &Node::Multiply(_, ref e1, ref e2) => self.binary_op(e1, e2, &|l, r| l * r),
            &Node::Divide(_, ref e1, ref e2) => self.binary_op(e1, e2, &|l, r| l / r),
            &Node::Subtract(_, ref e1, ref e2) => self.binary_op(e1, e2, &|l, r| l - r),
            &Node::Power(_, ref e1, ref e2) => self.binary_op(e1, e2, &|l, r| l.pow(r as u32)),
            &Node::Variable(_, ref name) => match self.variables.get(name) {
                Some(&val) => Ok(Some(val)),
                None => Err(CalcError),
            },
            &Node::Assignment(_, ref var, ref val) => self.assign_value(var, val),
        }
    }

    pub fn calculate(&mut self, nodes: &Statements) -> String {
        let mut result = String::new();

        let Statements(stts) = nodes;
        for stt in stts.iter() {
            let Statement(term) = stt;
            match self.process_term(term) {
                Ok(Some(n)) => fmt::write(&mut result, format_args!("{} = {}; ", term, n)).unwrap(),
                Ok(None) => {}
                Err(_) => fmt::write(&mut result, format_args!("Error; ")).unwrap(),
            };
        }

        if !result.is_empty() {
            result.pop();
        }
        result
    }
}
