use crate::error::KrEvalError;
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
    dyadic: fn(Env, &[Kr]) -> (Env, Result<Kr, KrEvalError>),
    text: Text,
    rank: usize,
}

impl Operator {
    pub fn new(op: Op) -> Self {
        let (f, t): (fn(Env, &[Kr]) -> (Env, Result<Kr, KrEvalError>), &str) = match op {
            Op::Addition => { (kr_addition, "+") },
            Op::Subtraction => { (kr_subtraction, "-") },
            Op::Multiplication => { (kr_multiplication, "*") },
            Op::Division => { (kr_division, "%") },
            Op::Assign => { (kr_assign, ":") },
            Op::Join => { (kr_join, ",") },
        };
        Operator { dyadic: f, text: Text::from_str(t), rank: 2 }
    }

    pub fn to_string(&self) -> String {
        self.text.to_string()
    }
    pub fn apply(&self, env: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
        if args.len() != self.rank { return (env, Err(KrEvalError::Rank)) };
        (self.dyadic)(env, args)
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


macro_rules! elementwise_operation {
    ($variant:ident, $xv:ident, $yv:ident, $op:expr) => {
        if $xv.len() == $yv.len() {
            Ok(Kr::$variant($xv.iter().zip($yv.iter()).map(|(a, b)| $op(a, b)).collect()))
        } else {
            Err(KrEvalError::Length)
        }
    };
}

macro_rules! generate_arithmetic_fn {
    ($name:ident, $operator:tt) => {
        fn $name(e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
            let [x, y] = args else { unreachable!() };
            let res = match (x, y) {
                (Kr::I(x), Kr::I(y)) => Ok(Kr::I(x $operator y)),
                (Kr::J(x), Kr::J(y)) => Ok(Kr::J(x $operator y)),
                (Kr::E(x), Kr::E(y)) => Ok(Kr::E(x $operator y)),
                (Kr::F(x), Kr::F(y)) => Ok(Kr::F(x $operator y)),
                (Kr::Iv(xv), Kr::Iv(yv)) => elementwise_operation!(Iv, xv, yv, |a, b| a $operator b),
                (Kr::Jv(xv), Kr::Jv(yv)) => elementwise_operation!(Jv, xv, yv, |a, b| a $operator b),
                (Kr::Ev(xv), Kr::Ev(yv)) => elementwise_operation!(Ev, xv, yv, |a, b| a $operator b),
                (Kr::Fv(xv), Kr::Fv(yv)) => elementwise_operation!(Fv, xv, yv, |a, b| a $operator b),
                (_, _) => Err(KrEvalError::Type),
            };
            (e, res)
        }
    };
}

generate_arithmetic_fn!(kr_addition, +);
generate_arithmetic_fn!(kr_subtraction, -);
generate_arithmetic_fn!(kr_multiplication, *);
generate_arithmetic_fn!(kr_division, /);

fn kr_join(e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    let [x, y] = args else { unreachable!() };
    let x = atom_to_vec(x);
    let y = atom_to_vec(y);
    let res = match (x,y) {
        (Kr::Iv(x), Kr::Iv(y)) => Ok(Kr::Iv([&x[..], &y[..]].concat())),
        (Kr::Jv(x), Kr::Jv(y)) => Ok(Kr::Jv([&x[..], &y[..]].concat())),
        (Kr::Ev(x), Kr::Ev(y)) => Ok(Kr::Ev([&x[..], &y[..]].concat())),
        (Kr::Fv(x), Kr::Fv(y)) => Ok(Kr::Fv([&x[..], &y[..]].concat())),
        (Kr::Cv(x), Kr::Cv(y)) => Ok(Kr::Cv([&x[..], &y[..]].concat())),
        (Kr::NN(x), Kr::NN(y)) => Ok(Kr::NN([&x[..], &y[..]].concat())),
        (_, _) => Err(KrEvalError::Type)
    };
    (e, res)
}

fn kr_assign(mut e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    let [x, y] = args else { unreachable!() };
    match (x,y) {
        (Kr::S(k), v) => {e.var.insert(k.clone(), v.clone());},
        (_, _) => return (e, Err(KrEvalError::Assign),)
    }
    (e, Ok(Kr::Null))
}

fn atom_to_vec(x: &Kr) -> Kr {
    match x {
        Kr::I(a) => Kr::Iv([*a].to_vec()),
        Kr::J(a) => Kr::Jv([*a].to_vec()),
        Kr::E(a) => Kr::Ev([*a].to_vec()),
        Kr::F(a) => Kr::Fv([*a].to_vec()),
        Kr::C(a) => Kr::Cv([*a].to_vec()),
        other => other.clone(),
    }
}