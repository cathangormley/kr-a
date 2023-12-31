use crate::error::KrEvalError;
use crate::operator::Operator;
use crate::text::Text;
use crate::init::Env;
use crate::primitive::Primitive;

#[derive(Clone, Debug)]
pub enum Kr {
    // B(bool),Bv(Vec<bool>),   // Boolean
    I(i32), Iv(Vec<i32>),       // Integer
    J(i64), Jv(Vec<i64>),       // Long 
    E(f32), Ev(Vec<f32>),       // Real
    F(f64), Fv(Vec<f64>),       // Float
    C(u8),  Cv(Vec<u8>),        // Character
    S(Text),
    Op(Operator),               // Operator
    Prim(Primitive),            // Primitive
    Null,                       // Null
    NN(Vec<Kr>),                // General list of variables
}

impl Kr {
    pub fn print(&self) -> String {
        match self {
            Kr::I(n) => n.to_string(),
            Kr::J(n) => n.to_string(),
            Kr::E(n) => n.to_string(),
            Kr::F(n) => n.to_string(),
            Kr::C(c) => "\"".to_string() + &c.to_string().to_owned() + "\"",
            Kr::S(sym) => { "`".to_string() + &sym.to_string()},
            Kr::Null => "(::)".to_string(),
            Kr::Iv(iv) => vec_to_string(iv, " ", "", ""),
            Kr::Jv(jv) => vec_to_string(jv, " ", "", ""),
            Kr::Ev(ev) => vec_to_string(ev, " ", "", ""),
            Kr::Fv(fv) => vec_to_string(fv, " ", "", ""),
            Kr::Cv(cv) => {"\"".to_owned() + &String::from_utf8(cv.to_vec()).unwrap() + "\""},
            Kr::Op(op) => op.to_string(),
            Kr::Prim(prim) => prim.display(),
            Kr::NN(kl) => { // vec_to_string(kl, "\n ")
                let mut output = String::new();
                output.push('[');
                for k in kl {
                    output.push_str(&k.print());
                    output.push_str(";");
                }
                output.pop();
                output.push(']');
                output
            }
        }
    }
}

impl Kr {
    pub fn apply(&self, env: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
        match self {
            Kr::Op(op) => op.apply(env, args),
            Kr::Prim(prim) => prim.apply(env, args),
            _ => (env, Err(KrEvalError::NotAVerb)),
        }
    }
}

fn vec_to_string<T>(v: &Vec<T>, delim: &str, start: &str, end: &str) -> String 
where T:ToString
{
    let x: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("{}{}{}", start, x.join(delim), end)
}