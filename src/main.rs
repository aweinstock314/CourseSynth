use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
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

fn differentiate(e: Expr<char, u32>, var: char) -> Expr<char, u32> {
    match e {
        Expr::Variable(x) if x == var => Expr::Constant(1),
        Expr::Variable(_) => Expr::Constant(0),
        Expr::Constant(_) => Expr::Constant(0),
        Expr::Plus(e1, e2) => differentiate(*e1, var) + differentiate(*e2, var),
        Expr::Times(e1,e2) => *e1.clone() * differentiate(*e2.clone(), var) + *e2 * differentiate(*e1, var),
        _ => unimplemented!(),
    }
}

fn main() {
    use Expr::*;
    println!("{:?}", (Variable("x") + Constant(42)).pow(Constant(2) * Variable("y")));
    let tmp = Variable('x') + Variable('x') + Constant(2);
    println!("Before: {:?}", tmp);
    println!("After: {:?}", differentiate(tmp, 'x'));
    let tmp = Variable('x') * Variable('x') + Constant(2);
    println!("Before: {:?}", tmp);
    println!("After: {:?}", differentiate(tmp, 'x'));
}
