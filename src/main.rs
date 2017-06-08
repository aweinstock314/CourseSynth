use std::ops::{Add, Mul};

#[derive(Debug)]
enum Expr<T, U> {
    Variable(T),
    Constant(U),
    Plus(Box<Expr<T,U>>, Box<Expr<T,U>>),
    Times(Box<Expr<T,U>>, Box<Expr<T,U>>),
    Power(Box<Expr<T,U>>, Box<Expr<T,U>>),
}

impl<T, U> Add for Expr<T, U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::Plus(Box::new(self), Box::new(rhs))
    }
}

impl<T, U> Mul for Expr<T, U> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Times(Box::new(self), Box::new(rhs))
    }
}

impl<T, U> Expr<T, U> {
    fn pow(self, rhs: Self) -> Self {
        Expr::Power(Box::new(self), Box::new(rhs))
    }
}


fn main() {
    use Expr::*;
    println!("{:?}", (Variable("x") + Constant(42)).pow(Constant(2) * Variable("y")));
}
