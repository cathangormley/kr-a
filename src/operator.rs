use crate::kr::Kr;

pub struct Operator {
    pub text: Vec<u8>,
    pub dyadic: fn(&Kr, &Kr) -> Kr
}

impl Operator {
    pub fn new(text: Vec<u8>, dyadic:fn(&Kr, &Kr) -> Kr ) -> Self {
        Operator { text, dyadic }
    }
}

pub fn kr_add(x: &Kr, y: &Kr) -> Kr {
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x + y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x + y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x + y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x + y),
        (_, _) => Kr::J(0),
    }
}

pub fn kr_subtract(x: &Kr, y: &Kr) -> Kr {
    match (x,y) {
        (Kr::I(x), Kr::I(y)) => Kr::I(x - y),
        (Kr::J(x), Kr::J(y)) => Kr::J(x - y),
        (Kr::E(x), Kr::E(y)) => Kr::E(x - y),
        (Kr::F(x), Kr::F(y)) => Kr::F(x - y),
        (_, _) => Kr::J(0),
    }
}

