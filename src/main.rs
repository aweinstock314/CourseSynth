extern crate futures;
extern crate hyper;

use futures::{Future, IntoFuture};
use hyper::server::{Http, Service, Request, Response};
use hyper::{Get, StatusCode};
use std::net::ToSocketAddrs;
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

struct Website;

impl Service for Website {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/") => Box::new(Ok(Response::new().with_body("Hello, world!".to_string())).into_future()),
            _ => Box::new(Ok(Response::new().with_status(StatusCode::NotFound)).into_future()),
        }
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

    let addr = ("127.0.0.1", 8000).to_socket_addrs().unwrap().next().unwrap();
    Http::new().bind(&addr, || Ok(Website)).expect("Failed to initialize http server.").run().expect("An error occurred while running the server.");
}
