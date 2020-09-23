use std::collections::HashMap;
use std::fmt;

use crate::parser::nodes::Node;

pub enum CalcError {
    UnknownVariable(String),
    WrongNodeTree,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CalcError::*;
        match self {
            UnknownVariable(v) => write!(f, "Computation error: using unknown variable '{}'", v),
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
        match self.process_node(l) {
            Ok(Some(r)) => Ok(Some(f(r))),
            err => err,
        }
    }

    fn binary_op(&mut self, l: &Node, r: &Node, f: &dyn Fn(i128, i128) -> i128) -> CalcResult {
        match (self.process_node(l), self.process_node(r)) {
            (Ok(Some(n1)), Ok(Some(n2))) => Ok(Some(f(n1, n2))),
            (Ok(_), Ok(_)) => Err(CalcError::WrongNodeTree),
            (err @ Err(_), _) => err,
            (_, err @ Err(_)) => err,
        }
    }

    fn assign_value(&mut self, var: &Node, val: &Node) -> CalcResult {
        match (var, self.process_node(val)) {
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
            None => Err(CalcError::UnknownVariable(name.to_string())),
        }
    }

    fn process_node(&mut self, node: &Node) -> CalcResult {
        match node {
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
            &Node::RoundBrackets(_, ref n) => self.process_node(n),
        }
    }

    pub fn process_statement(&mut self, node: &Node) {
        match self.process_node(node) {
            Ok(Some(n)) => println!("{} = {}; ", node, n),
            Ok(None) => {}
            Err(err) => println!("{}. Context: '{}' ", err, node),
        }
    }

    pub fn print_vars(&self) {
        for (key, value) in &self.variables {
            println!("{}: {}", key, value);
        }
    }
}
