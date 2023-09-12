use crate::operator::Operator;
use crate::text::Text;
use crate::init::Env;

#[derive(Clone, Debug)]
pub enum Kr {
    I(i32), Iv(Vec<i32>),   // Integer
    J(i64), Jv(Vec<i64>),   // Long 
    E(f32), Ev(Vec<f32>),   // Real
    F(f64), Fv(Vec<f64>),   // Float
    C(u8),  Cv(Vec<u8>),    // Character
    S(Text),
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
            Kr::S(sym) => { "`".to_string() + &sym.to_string()}, // "`".to_string() + &String::from_utf8(sym.to_vec()).unwrap(),
            Kr::Null => "".to_string(),
            Kr::Cv(cv) => {"\"".to_owned() + &String::from_utf8(cv.to_vec()).unwrap() + "\""},
            _ => "Cannot display".to_owned()
        }
    }
}

impl Kr {
    pub fn apply(&self, env: Env, args: Vec<Kr>) -> (Env, Kr) {
        match self {
            Kr::Op(op) => (op.dyadic)(env, args),
            Kr::Null => {
                if let Some(Kr::S(s)) = args.first() {
                    let v = env.val(&Kr::S(s.clone()));
                    (env, v)
                } else {
                    (env, Kr::Null)
                }
            }
            x => (env, x.clone())
        }
    }
}