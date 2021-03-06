#![feature(box_patterns)]
extern crate futures;
extern crate hyper;
extern crate num_traits;
extern crate rand;
extern crate serde_urlencoded;

use futures::{Future, IntoFuture, Stream};
use hyper::server::{Http, Service, Request, Response};
use hyper::{Get, Post, StatusCode};
use num_traits::*;
use std::fmt::Debug;
use std::net::ToSocketAddrs;
use std::ops::{Add, Mul, Sub};
use std::str;
use rand::distributions::{Normal, IndependentSample};
use std::collections::HashMap;
use rand::{thread_rng, Rng};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr<T, U> {
    Variable(T),
    Constant(U),
    Plus(Box<Expr<T, U>>, Box<Expr<T, U>>),
    Times(Box<Expr<T, U>>, Box<Expr<T, U>>),
    Power(Box<Expr<T, U>>, Box<Expr<T, U>>),
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

fn differentiate<U: Clone + Zero + One + Sub<U, Output = U>>(e: Expr<char, U>, var: char) -> Expr<char, U> {
    match e {
        Expr::Variable(x) if x == var => Expr::Constant(one()),
        Expr::Variable(_) => Expr::Constant(zero()),
        Expr::Constant(_) => Expr::Constant(zero()),
        Expr::Plus(e1, e2) => differentiate(*e1, var) + differentiate(*e2, var),
        Expr::Times(e1, e2) => *e1.clone() * differentiate(*e2.clone(), var) + *e2.clone() * differentiate(*e1.clone(), var),
        Expr::Power(e1, e2) => *e2.clone() * (*e1.clone()).pow(*e2.clone() + Expr::Constant(zero::<U>() - one())) * differentiate(*e1.clone(), var),
    }
}
fn generate(depth: usize, var: char) -> Expr<char, i32> {
    let normal = Normal::new(2.0, 3.0);
    if depth == 0 {
        let v2 = normal.ind_sample(&mut thread_rng());
        if thread_rng().gen() {
            Expr::Variable(var)
        } else {
            Expr::Constant(v2 as i32)
        }
    } else {
        match thread_rng().gen::<u32>() % 3 {
            0 => generate(depth - 1, var) + generate(depth - 1, var),
            1 => generate(depth - 1, var) * generate(depth - 1, var),
            2 => generate(depth - 1, var).pow(Expr::Constant(thread_rng().gen::<i32>().abs() % 5 + 1)),
            _ => unreachable!(),
        }
    }
}
fn generate2(depth: usize, var: char) -> Expr<char, i32> {
    let normal = Normal::new(2.0, 3.0);
    let v2 = normal.ind_sample(&mut thread_rng());
    if depth == 0 {
        Expr::Constant(v2 as i32)
    } else {
        Expr::Constant(v2 as i32) * Expr::Variable(var).pow(Expr::Constant(depth as i32)) + generate2(depth - 1, var)
    }
}

fn ipow<T: Copy + Eq + Zero + One + Sub<T, Output = T>>(base: T, exponent: T) -> T {
    if exponent == zero() {
        one()
    } else {
        base * ipow(base, exponent - one())
    }
}
fn simplify<T: Clone + Eq, U: Copy + Eq + Zero + One + Sub<U, Output = U>>(e: Expr<T, U>) -> Expr<T, U> {
    use Expr::*;
    match e {
        // Constant evalutation
        Plus(box Constant(x), box Constant(y)) => Constant(x + y),
        Times(box Constant(x), box Constant(y)) => Constant(x * y),
        Times(box Power(box ref x, box ref y), box Power(box ref w, box ref z)) if x == w => x.clone().pow(y.clone() + z.clone()),
        Times(box Power(box ref x, box ref y), box ref w) if x == w => x.clone().pow(y.clone() + Constant(one())),

        Power(box Power(box ref x, box Constant(y)), box Constant(z)) => x.clone().pow(Constant(y * z)),
        Power(box ref x, box Constant(y)) if y == one() => x.clone(),
        Power(box _, box Constant(y)) if y == zero() => Constant(one()),
        Power(box Constant(x), box Constant(_)) if x == zero() => Constant(zero()),
        Power(box Constant(x), box Constant(_)) if x == one() => Constant(one()),
        Power(box Constant(x), box Constant(y)) => Constant(ipow(x, y)),

        Times(box ref x, box ref w) if x == w => x.clone().pow(Constant(one()) + Constant(one())),

        // Algebraic identities
        Plus(box Constant(x), box ref y) if x == zero() => y.clone(),
        Plus(box ref x, box Constant(y)) if y == zero() => x.clone(),
        Times(box Constant(x), box ref y) if x == one() => y.clone(),
        Times(box ref x, box Constant(y)) if y == one() => x.clone(),
        Times(box Constant(x), box _) if x == zero() => Constant(zero()),
        Times(box _, box Constant(y)) if y == zero() => Constant(zero()),

        // Recursion into subtrees
        Plus(box x, box y) => simplify(x) + simplify(y),
        Times(box x, box y) => simplify(x) * simplify(y),
        Power(box x, box y) => simplify(x).pow(simplify(y)),
        x => x,
    }
}
fn emit_latex<T: std::fmt::Display, U: std::fmt::Display>(e: Expr<T, U>) -> String {
    use Expr::*;
    match e {
        Constant(x) => format!("{}", x),
        Variable(x) => format!("{}", x),
        Plus(box x, box y) => format!("\\left({}\\right) + \\left({}\\right)", emit_latex(x), emit_latex(y)),
        Times(box x, box y) => format!("\\left({}\\right) * \\left({}\\right)", emit_latex(x), emit_latex(y)),
        Power(box x, box y) => format!("\\left({{{}}}\\right)^{{{}}}", emit_latex(x), emit_latex(y)),
    }
}

fn converge<T: Debug + Eq, F: Fn(&T) -> T>(f: F, x: T) -> T {
    let mut last = x;
    let mut i = 0;
    loop {
        println!("converge, i={}, last={:?}", i, last);
        i += 1;
        let next = f(&last);
        if next == last {
            return next;
        }
        last = next;
    }
}

struct Website;

impl Service for Website {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/") => {
                let s = str::from_utf8(include_bytes!("main.html")).unwrap().to_string();
                Box::new(Ok(Response::new().with_body(s)).into_future())
            }
            (&Post, "/generate") => {
                //update the s
                println!("success!");
                //render the main.html
                //let s = str::from_utf8(include_bytes!("main.html")).unwrap().to_string();
                let mut s = String::new();
                s += "<script type=\"text/x-mathjax-config\">
  MathJax.Hub.Config({
    extensions: [\"tex2jax.js\"],
    jax: [\"input/TeX\", \"output/HTML-CSS\"],
    tex2jax: {
      inlineMath: [ ['$','$'], [\"\\(\",\"\\)\"] ],
      displayMath: [ ['$$','$$'], [\"\\[\",\"\\]\"] ],
      processEscapes: true
    },
    \"HTML-CSS\": { availableFonts: [\"TeX\"] }
  });
</script>";
                s += "<script src='https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.0/MathJax.js?config=TeX-MML-AM_CHTML'></script>";
                let body = req.body();
                Box::new(body.concat2().and_then(move |chunk| {
                    let f = String::from_utf8_lossy(&chunk);
                    let params: Result<HashMap<String, usize>, _> = serde_urlencoded::from_str(&f);

                    println!("{:?}", params);
                    if let Ok(params) = params {
                        println!("{:?}", params.get("num"));
                        for _ in 0..*params.get("num").unwrap_or(&1) {
                            s += &format!("${}$", emit_latex(converge(|x| simplify(x.clone()), generate2(3, 'x'))));
                            s += "\n<br>";
                        }
                        Box::new(Ok(Response::new().with_body(s)).into_future())
                    } else {
                        Box::new(Ok(Response::new().with_body("Invalid Parameter")).into_future())
                    }
                }))
            }
            (&Get, "/assets/style.css") => Box::new(
                Ok(Response::new().with_body(str::from_utf8(include_bytes!("assets/style.css")).unwrap().to_string())).into_future(),
            ),
            (&Get, "/assets/index.js") => Box::new(
                Ok(Response::new().with_body(str::from_utf8(include_bytes!("assets/index.js")).unwrap().to_string())).into_future(),
            ),
            _ => Box::new(Ok(Response::new().with_status(StatusCode::NotFound)).into_future()),
        }
    }
}

fn test1(e: Expr<char, i32>) {
    println!("e: {:?}", e);
    let e1 = differentiate(e, 'x');
    println!("differentiated: {:?}", e1);
    let e2 = converge(|x| simplify(x.clone()), e1);
    println!("simplfied: {:?}", e2);
}

fn main() {
    use Expr::*;
    println!("{:?}", (Variable("x") + Constant(42)).pow(Constant(2) * Variable("y")));
    test1(Variable('x') + Variable('x') + Constant(2));
    test1(Variable('x') * Variable('x') + Constant(2));
    test1(Variable('x') + Variable('x').pow(Constant(2)) + Constant(2));
    for i in 0..5 {
        println!("Tree: {} {:?}", i, generate2(i, 'x'));
    }
    let addr = ("127.0.0.1", 8000).to_socket_addrs().unwrap().next().unwrap();
    Http::new().bind(&addr, || Ok(Website)).expect("Failed to initialize http server.").run().expect("An error occurred while running the server.");
}
