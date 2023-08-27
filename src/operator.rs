use crate::kr::Kr;
use crate::init::Env;
use crate::Name;

#[derive(Clone, Debug)]
pub struct Operator {
    pub text: Vec<u8>,
    pub dyadic: fn(&mut Env, &Kr, &Kr) -> Kr
}

impl Operator {
    pub fn new(text: Vec<u8>) -> Self {
        let f: fn(&mut Env, &Kr, &Kr) -> Kr = match text[..] {
            [b'+'] => { kr_addition } ,
            [b'-'] => { kr_subtraction },
            [b'*'] => { kr_multiplication },
            [b'%'] => { kr_division },
            [b':'] => { kr_assign },
            _ => { kr_dyad_default }
        };
        Operator { text: text, dyadic: f }
    }
}

pub fn kr_addition(e: &mut Env, x: &Kr, y: &Kr) -> Kr {
    let x = e.value(x);
    let y = e.value(y);
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x + y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x + y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x + y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x + y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_subtraction(e: &mut Env, x: &Kr, y: &Kr) -> Kr {
    let x = e.value(x);
    let y = e.value(y);
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x - y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x - y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x - y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x - y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_multiplication(e: &mut Env, x: &Kr, y: &Kr) -> Kr {
    let x = e.value(x);
    let y = e.value(y);
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x * y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x * y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x * y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x * y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_division(e: &mut Env, x: &Kr, y: &Kr) -> Kr {
    let x = e.value(x);
    let y = e.value(y);
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::E(*x as f32 / *y as f32),
        (Kr::J(x), Kr::J(y)) => Kr::F(*x as f64 / *y as f64),
        (Kr::E(x), Kr::E(y)) => Kr::E(x / y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x / y),
        (_, _) => Kr::F(0.0),
    }
}

pub fn kr_assign(e: &mut Env, x: &Kr, y: &Kr) -> Kr {
    let y = e.value(y);
    match (x,y) {
        (Kr::Cv(a), b) => {e.var.insert(Name { text: a.to_vec() }, b.clone()); Kr::J(0)},
        (a, _b) => {println!("Cannot assign to {:?}", a); Kr::J(0)}
    }
}

pub fn kr_dyad_default(_e: &mut Env, _x: &Kr, _y: &Kr) -> Kr {
    Kr::J(0)
}