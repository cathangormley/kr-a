use crate::kr::Kr;
use crate::init::Env;
use crate::text::Text;

use std::fmt::Debug;

#[derive(Clone, Debug, Copy)]
pub enum Op {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Assign,
    Join,
}

#[derive(Clone, Debug)]
pub struct Operator {
    pub dyadic: fn(Env, &[Kr]) -> (Env, Kr),
    pub text: Text,
}

impl Operator {
    pub fn new(op: Op) -> Self {
        let (f, t): (fn(Env, &[Kr]) -> (Env, Kr), &str) = match op {
            Op::Addition => { (kr_addition, "+") },
            Op::Subtraction => { (kr_subtraction, "-") },
            Op::Multiplication => { (kr_multiplication, "*") },
            Op::Division => { (kr_division, "%") },
            Op::Assign => { (kr_assign, ":") },
            Op::Join => { (kr_join, ",") },
        };
        Operator { dyadic: f, text: Text::from_str(t) }
    }

    pub fn to_string(&self) -> String {
        self.text.to_string()
    }
}


// Still can't get this to work..
// Maybe I should use a macro..
/*
// Define a type alias for the dyadic function's return type
type DyadicFn = dyn Fn(Env, &[Kr]) -> (Env, Kr);

fn dyadic<F, T>(f: F) -> Box<DyadicFn>
where
    F: Fn(Kr, Kr) -> Kr + 'static,
{
    let f = move |e: Env, args: &[Kr]| {
        if args.len() != 2 { return (e, Kr::Null) };
        let x = e.val(&args[0]);
        let y = e.val(&args[1]);
        (e, f(x,y))
    };
    Box::new(f)
}
*/

pub fn kr_addition(e: Env, args: &[Kr]) -> (Env, Kr) {
    if args.len() != 2 {return (e, Kr::Null) };
    let x = e.val(&args[0]);
    let y = e.val(&args[1]);

    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x + y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x + y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x + y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x + y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_subtraction(e: Env, args: &[Kr]) -> (Env, Kr) {
    if args.len() != 2 { return (e, Kr::Null) };
    let x = e.val(&args[0]);
    let y = e.val(&args[1]);

    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x - y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x - y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x - y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x - y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_multiplication(e: Env, args: &[Kr]) -> (Env, Kr) {
    if args.len() != 2 { return (e, Kr::Null) };
    let x = e.val(&args[0]);
    let y = e.val(&args[1]);

    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x * y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x * y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x * y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x * y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_division(e: Env, args: &[Kr]) -> (Env, Kr) {
    if args.len() != 2 { return (e, Kr::Null) };
    let x = e.val(&args[0]);
    let y = e.val(&args[1]);

    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::E(x as f32 / y as f32),
        (Kr::J(x), Kr::J(y)) => Kr::F(x as f64 / y as f64),
        (Kr::E(x), Kr::E(y)) => Kr::E(x / y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x / y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_join(e: Env, args: &[Kr]) -> (Env, Kr) {
    if args.len() != 2 { return (e, Kr::Null) };
    let x = e.val(&args[0]);
    let y = e.val(&args[1]);

    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::Iv(vec![x,y]),
        (Kr::J(x), Kr::J(y)) => Kr::Jv(vec![x,y]),
        (Kr::E(x), Kr::E(y)) => Kr::Ev(vec![x,y]),
        (Kr::F(x), Kr::F(y)) => Kr::Fv(vec![x,y]),
        (Kr::Jv(x), Kr::Jv(y)) => Kr::Jv( [&x[..], &y[..]].concat()),
        (Kr::Cv(x), Kr::Cv(y)) => {
            let mut v: Vec<u8> = Vec::new();
            v.extend_from_slice(&x);
            v.extend_from_slice(&y);
            Kr::Cv(v)
        },
        (_, _) => Kr::Null
    };
    (e, res)
}

pub fn kr_assign(mut e: Env, args: &[Kr]) -> (Env, Kr) {
    if args.len() != 2 { return (e, Kr::Null) };
    let y = e.val(&args[1]);
    match (&args[0],y) {
        (Kr::S(a), b) => {e.var.insert(a.clone(), b.clone());},
        (a, _b) => {println!("Cannot assign to {:?}", a)}
    }
    (e, Kr::Null)
}