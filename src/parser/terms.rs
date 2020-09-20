use std::fmt;

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

pub struct Block(pub Term);

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Block(exprs) = self;
        write!(f, "[{:?}]", exprs)
    }
}

pub enum Term {
    Multiply(Loc, Box<Term>, Box<Term>),
    Divide(Loc, Box<Term>, Box<Term>),
    Add(Loc, Box<Term>, Box<Term>),
    Subtract(Loc, Box<Term>, Box<Term>),
    Power(Loc, Box<Term>, Box<Term>),
    StringLiteral(Loc, std::string::String),
    NumberLiteral(Loc, i128),
    UnaryMinus(Loc, Box<Term>),
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Term::*;
        match self {
            Multiply(_, l, r) => write!(f, "({:?} * {:?})", l, r),
            Divide(_, l, r) => write!(f, "({:?} / {:?})", l, r),
            Add(_, l, r) => write!(f, "({:?} + {:?})", l, r),
            Subtract(_, l, r) => write!(f, "({:?} - {:?})", l, r),
            Power(_, l, r) => write!(f, "({:?} ** {:?})", l, r),
            StringLiteral(_, s) => write!(f, "{}", s),
            NumberLiteral(_, n) => write!(f, "{}", n),
            UnaryMinus(_, r) => write!(f, "-{:?}", r),
        }
    }
}
