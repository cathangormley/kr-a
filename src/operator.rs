use crate::kr::Kr;

#[derive(Clone, Debug)]
pub struct Operator {
    pub text: Vec<u8>,
    pub dyadic: fn(&Kr, &Kr) -> Kr
}

impl Operator {
    /*
    pub fn new(text: Vec<u8>, dyadic:fn(&Kr, &Kr) -> Kr ) -> Self {
        Operator { text, dyadic }
    }
    */
    pub fn new(text: Vec<u8>) -> Self {
        let f: fn(&Kr, &Kr) -> Kr = match text[..] {
            [b'+'] => { kr_addition } ,
            [b'-'] => { kr_subtraction },
            [b'*'] => { kr_multiplication },
            [b'%'] => { kr_division}
            _ => { kr_dyad_default }
        };
        Operator { text: text, dyadic: f }
    }
}

pub fn kr_addition(x: &Kr, y: &Kr) -> Kr {
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x + y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x + y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x + y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x + y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_subtraction(x: &Kr, y: &Kr) -> Kr {
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x - y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x - y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x - y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x - y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_multiplication(x: &Kr, y: &Kr) -> Kr {
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x * y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x * y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x * y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x * y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_division(x: &Kr, y: &Kr) -> Kr {
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::E(*x as f32 / *y as f32),
        (Kr::J(x), Kr::J(y)) => Kr::F(*x as f64 / *y as f64),
        (Kr::E(x), Kr::E(y)) => Kr::E(x / y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x / y),
        (_, _) => Kr::F(0.0),
    }
}



pub fn kr_dyad_default(_x: &Kr, _y: &Kr) -> Kr {
    Kr::J(0)
}