use super::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

// non-terminals
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

impl fmt::Debug for Statements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct Statement(pub Term);

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Statement(exprs) = self;
        write!(f, "{}{}", exprs, Token::Semicolon)
    }
}

pub enum Term {
    Multiply(Loc, Box<Term>, Box<Term>),
    Divide(Loc, Box<Term>, Box<Term>),
    Add(Loc, Box<Term>, Box<Term>),
    Subtract(Loc, Box<Term>, Box<Term>),
    Power(Loc, Box<Term>, Box<Term>),
    Assignment(Loc, Box<Term>, Box<Term>),
    Variable(Loc, std::string::String),
    NumberLiteral(Loc, i128),
    UnaryMinus(Loc, Box<Term>),
    Pi(Loc),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Term::*;
        match self {
            Multiply(_, l, r) => write!(f, "({} * {})", l, r),
            Divide(_, l, r) => write!(f, "({} / {})", l, r),
            Add(_, l, r) => write!(f, "({} + {})", l, r),
            Subtract(_, l, r) => write!(f, "({} - {})", l, r),
            Power(_, l, r) => write!(f, "({} ** {})", l, r),
            Assignment(_, l, r) => write!(f, "let {} = {}", l, r),
            Variable(_, s) => write!(f, "{}", s),
            NumberLiteral(_, n) => write!(f, "{}", n),
            UnaryMinus(_, r) => write!(f, "-{}", r),
            Pi(_) => write!(f, "PI"),
        }
    }
}
