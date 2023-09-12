use crate::kr::Kr;
use crate::init::Env;
use crate::text::Text;

#[derive(Debug, Clone)]
pub struct Operator {
    pub text: Text,
    pub dyadic: fn(Env, Vec<Kr>) -> (Env, Kr),
}

impl Operator {
    pub fn new(text: Text) -> Self {
        let f: fn(Env, Vec<Kr>) -> (Env, Kr) = match text.0[..] {
            [b'+'] => { kr_addition },
            [b'-'] => { kr_subtraction },
            [b'*'] => { kr_multiplication },
            [b'%'] => { kr_division },
            [b':'] => { kr_assign },
            [b','] => { kr_join },
            _ => { kr_dyad_default },
        };
        Operator { text: text, dyadic: f }
    }
    pub fn to_kr(&self) -> Kr {
        Kr::Op(self.clone())
    }
}


// Still can't get this to work..
// Maybe I should use a macro..
/*
// Define a type alias for the dyadic function's return type
type DyadicFn = dyn Fn(Env, Vec<Kr>) -> (Env, Kr);

fn dyadic<F, T>(f: F) -> Box<DyadicFn>
where
    F: Fn(Kr, Kr) -> Kr + 'static,
{
    let f = move |e: Env, args: Vec<Kr>| {
        if args.len() != 2 { return (e, Kr::Null) };
        let x = e.val(&args[0]);
        let y = e.val(&args[1]);
        (e, f(x,y))
    };
    Box::new(f)
}
*/

pub fn kr_addition(e: Env, args: Vec<Kr>) -> (Env, Kr) {
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

pub fn kr_subtraction(e: Env, args: Vec<Kr>) -> (Env, Kr) {
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

pub fn kr_multiplication(e: Env, args: Vec<Kr>) -> (Env, Kr) {
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

pub fn kr_division(e: Env, args: Vec<Kr>) -> (Env, Kr) {
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

pub fn kr_join(e: Env, args: Vec<Kr>) -> (Env, Kr) {
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

pub fn kr_assign(mut e: Env, args: Vec<Kr>) -> (Env, Kr) {
    if args.len() != 2 { return (e, Kr::Null) };
    let y = e.val(&args[1]);
    match (&args[0],y) {
        (Kr::S(a), b) => {e.var.insert(a.clone(), b.clone());},
        (a, _b) => {println!("Cannot assign to {:?}", a)}
    }
    (e, Kr::Null)
}

pub fn kr_dyad_default(e: Env, _args: Vec<Kr>) -> (Env, Kr) {
    (e, Kr::Null)
}