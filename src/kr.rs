use crate::operator::Operator;

#[derive(Clone, Debug)]
pub enum Kr {
    I(i32), Iv(Vec<i32>),   // Integer
    J(i64), Jv(Vec<i64>),   // Long 
    E(f32), Ev(Vec<f32>),   // Real
    F(f64), Fv(Vec<f64>),   // Float
    C(u8),  Cv(Vec<u8>),    // Character
    S(Vec<u8>),
    Op(Operator),           // Operator
    Null,                   // Null
    NN(Vec<Kr>),            // General list of variables
}

impl Kr {
    pub fn print(&self) -> String {
        match self {
            Kr::I(n) => n.to_string(),
            Kr::J(n) => n.to_string(),
            Kr::E(n) => n.to_string(),
            Kr::F(n) => n.to_string(),
            Kr::C(c) => "\"".to_string() + &c.to_string().to_owned() + "\"",
            Kr::S(sym) => "`".to_string() + &String::from_utf8(sym.to_vec()).unwrap(),
            Kr::Null => "".to_string(),
            _ => "Cannot display".to_owned()
        }
    }
}