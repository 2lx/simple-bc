use super::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

// non-terminals
#[derive(Debug)]
pub struct Statements(pub Vec<Statement>);

impl fmt::Display for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let Statements(stts) = self;
        if !stts.is_empty() {
            for stt in &stts[0..stts.len() - 1] {
                fmt::write(f, format_args!("{} ", stt))?
            }
            fmt::write(f, format_args!("{}", stts.last().unwrap()))?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Statement(pub Node);

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Statement(exprs) = self;
        write!(f, "{}{}", exprs, Token::Semicolon)
    }
}

#[derive(Debug)]
pub enum Node {
    Multiply(Loc, Box<Node>, Box<Node>),
    Divide(Loc, Box<Node>, Box<Node>),
    Add(Loc, Box<Node>, Box<Node>),
    Subtract(Loc, Box<Node>, Box<Node>),
    Power(Loc, Box<Node>, Box<Node>),
    Assignment(Loc, Box<Node>, Box<Node>),
    Variable(Loc, std::string::String),
    NumberLiteral(Loc, i128),
    UnaryMinus(Loc, Box<Node>),
    RoundBrackets(Loc, Box<Node>),
    Pi(Loc),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Node::*;
        match self {
            Multiply(_, l, r) => write!(f, "{} * {}", l, r),
            Divide(_, l, r) => write!(f, "{} / {}", l, r),
            Add(_, l, r) => write!(f, "{} + {}", l, r),
            Subtract(_, l, r) => write!(f, "{} - {}", l, r),
            Power(_, l, r) => write!(f, "{} ** {}", l, r),
            Assignment(_, l, r) => write!(f, "{} = {}", l, r),
            Variable(_, s) => write!(f, "{}", s),
            NumberLiteral(_, n) => write!(f, "{}", n),
            UnaryMinus(_, r) => write!(f, "-{}", r),
            RoundBrackets(_, r) => write!(f, "({})", r),
            Pi(_) => write!(f, "PI"),
        }
    }
}
