use crate::kr::Kr;
use crate::init::Env;

#[derive(Clone, Debug)]
pub struct Operator {
    pub text: Vec<u8>,
    pub dyadic: fn(Env, &Kr, &Kr) -> (Env, Kr)
}

impl Operator {
    pub fn new(text: Vec<u8>) -> Self {
        let f: fn(Env, &Kr, &Kr) -> (Env, Kr) = match text[..] {
            [b'+'] => { kr_addition } ,
            [b'-'] => { kr_subtraction },
            [b'*'] => { kr_multiplication },
            [b'%'] => { kr_division },
            [b':'] => { kr_assign },
            [b','] => { kr_join },
            _ => { kr_dyad_default }
        };
        Operator { text: text, dyadic: f }
    }
    pub fn to_kr(&self) -> Kr {
        Kr::Op(self.clone())
    }
}

// When I get this working I can wrap the addition, subtraction, .., functions to reduce repitition
/*
fn wrap<F>(e: Env, f: F) -> impl Fn(Env, &Kr, &Kr) -> (Env, Kr)
where F: Fn(&Kr, &Kr) -> Kr {
    move |e: Env, x: &Kr, y: &Kr| {
        let x = e.value(x);
        let y = e.value(y);
        (e, f(x,y))
    }
}
*/

pub fn kr_addition(e: Env, x: &Kr, y: &Kr) -> (Env, Kr) {
    let x = e.value(x);
    let y = e.value(y);
    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x + y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x + y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x + y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x + y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_subtraction(e: Env, x: &Kr, y: &Kr) -> (Env, Kr) {
    let x = e.value(x);
    let y = e.value(y);
    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x - y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x - y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x - y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x - y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_multiplication(e: Env, x: &Kr, y: &Kr) -> (Env, Kr) {
    let x = e.value(x);
    let y = e.value(y);
    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x * y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x * y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x * y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x * y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_division(e: Env, x: &Kr, y: &Kr) -> (Env, Kr) {
    let x = e.value(x);
    let y = e.value(y);
    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::E(*x as f32 / *y as f32),
        (Kr::J(x), Kr::J(y)) => Kr::F(*x as f64 / *y as f64),
        (Kr::E(x), Kr::E(y)) => Kr::E(x / y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x / y),
        (_, _) => Kr::Null,
    };
    (e, res)
}

pub fn kr_join(e: Env, x: &Kr, y: &Kr) -> (Env, Kr) {
    let x = e.value(x);
    let y = e.value(y);
    let res = match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::Iv(vec![*x,*y]),
        (Kr::J(x), Kr::J(y)) => Kr::Jv(vec![*x,*y]),
        (Kr::E(x), Kr::E(y)) => Kr::Ev(vec![*x,*y]),
        (Kr::F(x), Kr::F(y)) => Kr::Fv(vec![*x,*y]),
        (Kr::Jv(x), Kr::Jv(y)) => Kr::Jv( [&x[..], &y[..]].concat()),
        (_, _) => Kr::Null
    };
    (e, res)
}

pub fn kr_assign(mut e: Env, x: &Kr, y: &Kr) -> (Env, Kr) {
    let y = e.value(y);
    match (x,y) {
        (Kr::S(a), b) => {e.var.insert(a.to_vec(), b.clone());},
        (a, _b) => {println!("Cannot assign to {:?}", a)}
    }
    (e, Kr::Null)
}

pub fn kr_dyad_default(e: Env, _x: &Kr, _y: &Kr) -> (Env, Kr) {
    (e, Kr::Null)
}