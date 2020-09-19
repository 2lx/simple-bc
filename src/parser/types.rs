use std::fmt;

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

pub struct Block(pub Expr);

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Block(exprs) = self;
        write!(f, "[{:?}]", exprs)
    }
}

pub enum Expr {
    Multiply(Loc, Box<Expr>, Box<Expr>),
    Divide(Loc, Box<Expr>, Box<Expr>),
    Add(Loc, Box<Expr>, Box<Expr>),
    Subtract(Loc, Box<Expr>, Box<Expr>),
    StringLiteral(Loc, std::string::String),
    UnaryMinus(Loc, Box<Expr>),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;
        match self {
            Multiply(_, l, r) => write!(f, "({:?} * {:?})", l, r),
            Divide(_, l, r) => write!(f, "({:?} / {:?})", l, r),
            Add(_, l, r) => write!(f, "({:?} + {:?})", l, r),
            Subtract(_, l, r) => write!(f, "({:?} - {:?})", l, r),
            StringLiteral(_, s) => write!(f, "{}", s),
            UnaryMinus(_, r) => write!(f, "-{:?}", r),
        }
    }
}
