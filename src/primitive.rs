use crate::error::KrEvalError;
use crate::kr::Kr;
use crate::init::Env;
use crate::text::Text;

/*
List of built in primitive functions

*/
#[derive(Clone, Debug)]
pub enum Prim {
    First,
    Last,
    Til,
    Enlist,
    Value,
}

#[derive(Clone, Debug)]
pub struct Primitive {
    f: fn(Env, &[Kr]) -> (Env, Result<Kr, KrEvalError>),
    text: Text,
    rank: usize,
}

impl Primitive {
    pub fn new(prim: Prim) -> Self {
        let (f, t, rank): (fn(Env, &[Kr]) -> (Env, Result<Kr, KrEvalError>), &str, usize) = match prim {
            Prim::First => { (kr_first_wrapped, "first", 1) },
            Prim::Last => { (kr_last_wrapped, "last", 1) },
            Prim::Til => { (kr_til_wrapped, "til", 1) },
            Prim::Enlist => { (kr_enlist, "enlist", 0) },
            Prim::Value => { (kr_value, "value", 1) }
        };
        Primitive { f, text: Text::from_str(t), rank}
    }
    pub fn apply(&self, env: Env, args:&[Kr]) -> (Env, Result<Kr, KrEvalError>) {
        // Check rank, unless it is enlist
        if (args.len() != self.rank) && (self.f != kr_enlist) {
            return (env, Err(KrEvalError::Rank))
        };
        (self.f)(env, args)
    }
    pub fn display(&self) -> String {
        self.to_string()
    }
}

impl ToString for Primitive {
    fn to_string(&self) -> String { 
        self.text.to_string()
    }
}

macro_rules! first {
    ($list:expr, $default:expr) => { $list.first().copied().unwrap_or($default) };
}

fn kr_first_wrapped(e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    (e, kr_first(&args[0]))
}

fn kr_first(x: &Kr) -> Result<Kr, KrEvalError> {
    match x {
        Kr::Iv(list) => Ok(Kr::I(first!(list, 0i32))),
        Kr::Jv(list) => Ok(Kr::J(first!(list, 0i64))),
        Kr::Ev(list) => Ok(Kr::E(first!(list, 0f32))),
        Kr::Fv(list) => Ok(Kr::F(first!(list, 0f64))),
        Kr::Cv(list) => Ok(Kr::C(first!(list, b' '))),
        Kr::NN(list) => Ok((*list.first().unwrap_or(&Kr::Null)).clone()),
        _ => Err(KrEvalError::Type),
    }
}

macro_rules! last {
    ($list:expr, $default:expr) => { $list.last().copied().unwrap_or($default) };
}

fn kr_last_wrapped(e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    (e, kr_last(&args[0]))
}

fn kr_last(x: &Kr) -> Result<Kr, KrEvalError>  {
    match x {
        Kr::Iv(list) => Ok(Kr::I(last!(list, 0i32))),
        Kr::Jv(list) => Ok(Kr::J(last!(list, 0i64))),
        Kr::Ev(list) => Ok(Kr::E(last!(list, 0f32))),
        Kr::Fv(list) => Ok(Kr::F(last!(list, 0f64))),
        Kr::Cv(list) => Ok(Kr::C(last!(list, b' '))),
        Kr::NN(list) => Ok((*list.last().unwrap_or(&Kr::Null)).clone()),
        _ => Err(KrEvalError::Type),
    }
}

fn kr_til_wrapped(e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    (e, kr_til(&args[0]))
}

fn kr_til(x: &Kr) -> Result<Kr, KrEvalError> {
    match x {
        Kr::J(n) => Ok(Kr::Jv((0..*n).collect())),
        _ => Err(KrEvalError::Type),
    }
}

fn kr_enlist(e: Env, args: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    return (e, Ok(Kr::NN(args.to_vec())));
}

fn kr_value(e: Env, v: &[Kr]) -> (Env, Result<Kr, KrEvalError>) {
    let r = e.val(&v[0]);
    (e, r)
}