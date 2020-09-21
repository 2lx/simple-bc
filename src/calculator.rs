use std::collections::HashMap;
use std::fmt;

use crate::parser::nodes::{Node, Statement, Statements};

pub enum CalcError {
    UnknownVariable,
    WrongNodeTree,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CalcError::*;
        match self {
            UnknownVariable => write!(f, "Computation error: using unknown variable"),
            WrongNodeTree => write!(f, "Computation error: wrong token sequence"),
        }
    }
}

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
            err => err,
        }
    }

    fn binary_op(&mut self, l: &Node, r: &Node, f: &dyn Fn(i128, i128) -> i128) -> CalcResult {
        match (self.process_term(l), self.process_term(r)) {
            (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(f(n1, n2))),
            (Ok(_), Ok(_)) => Err(CalcError::WrongNodeTree),
            (err @ Err(_), _) => err,
            (_, err @ Err(_)) => err,
        }
    }

    fn assign_value(&mut self, var: &Node, val: &Node) -> CalcResult {
        match (var, self.process_term(val)) {
            (&Node::Variable(_, ref name), Ok(Some(n))) => {
                *self.variables.entry(name.to_string()).or_insert(n) = n;
                Ok(None)
            }
            (_, Ok(_)) => Err(CalcError::WrongNodeTree),
            (_, err) => err,
        }
    }

    fn get_value(&self, name: &String) -> CalcResult {
        match self.variables.get(name) {
            Some(&val) => Ok(Some(val)),
            None => Err(CalcError::UnknownVariable),
        }
    }

    fn process_term(&mut self, term: &Node) -> CalcResult {
        match term {
            &Node::NumberLiteral(_, n) => Ok(Some(n)),
            &Node::Pi(_) => Ok(Some(3.14159 as i128)),
            &Node::UnaryMinus(_, ref n1) => self.unary_op(n1, &|l| -l),
            &Node::Add(_, ref n1, ref n2) => self.binary_op(n1, n2, &|l, r| l + r),
            &Node::Multiply(_, ref n1, ref n2) => self.binary_op(n1, n2, &|l, r| l * r),
            &Node::Divide(_, ref n1, ref n2) => self.binary_op(n1, n2, &|l, r| l / r),
            &Node::Subtract(_, ref n1, ref n2) => self.binary_op(n1, n2, &|l, r| l - r),
            &Node::Power(_, ref n1, ref n2) => self.binary_op(n1, n2, &|l, r| l.pow(r as u32)),
            &Node::Variable(_, ref name) => self.get_value(name),
            &Node::Assignment(_, ref var, ref val) => self.assign_value(var, val),
            &Node::RoundBrackets(_, ref n) => self.process_term(n),
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
                Err(err) => {
                    fmt::write(&mut result, format_args!("{}. Context: '{}' ", err, term)).unwrap()
                }
            };
        }

        if !result.is_empty() {
            result.pop();
        }
        result
    }
}
