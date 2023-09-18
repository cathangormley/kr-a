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
}

#[derive(Clone, Debug)]
pub struct Primitive {
    prim: Prim,
    f: fn(Env, Vec<Kr>) -> (Env, Kr),
    text: Text,
}

impl Primitive {
    pub fn new(prim: Prim) -> Self {

    let (f, t): (fn(Env, Vec<Kr>) -> (Env, Kr), &str) = match prim {
        Prim::First => { (kr_first_wrapped, "first") },
        Prim::Last => { (kr_last_wrapped, "last") },
    };
    Primitive { prim, f, text: Text::from_str(t) }
    }

    pub fn apply(&self, e: Env, args:Vec<Kr>) -> (Env, Kr) {
        (self.f)(e, args)
    }
}


macro_rules! first {
    ($list:expr, $default:expr) => { $list.first().copied().unwrap_or($default) };
}

fn kr_first_wrapped(e: Env, args: Vec<Kr>) -> (Env, Kr) {
    if args.len() > 1 { return (e, Kr::Null) };
    (e, kr_first(&args[0]))
}

fn kr_first(x: &Kr) -> Kr {
    match x {
        Kr::Iv(list) => Kr::I(first!(list, 0i32)),
        Kr::Jv(list) => Kr::J(first!(list, 0i64)),
        Kr::Ev(list) => Kr::E(first!(list, 0f32)),
        Kr::Fv(list) => Kr::F(first!(list, 0f64)),
        Kr::Cv(list) => Kr::C(first!(list, b' ')),
        x => x.clone(),
    }
}


macro_rules! last {
    ($list:expr, $default:expr) => { $list.last().copied().unwrap_or($default) };
}

fn kr_last_wrapped(e: Env, args: Vec<Kr>) -> (Env, Kr) {
    if args.len() > 1 { return (e, Kr::Null) };
    (e, kr_last(&args[0]))
}

fn kr_last(x: &Kr) -> Kr {
    match x {
        Kr::Iv(list) => Kr::I(last!(list, 0i32)),
        Kr::Jv(list) => Kr::J(last!(list, 0i64)),
        Kr::Ev(list) => Kr::E(last!(list, 0f32)),
        Kr::Fv(list) => Kr::F(last!(list, 0f64)),
        Kr::Cv(list) => Kr::C(last!(list, b' ')),
        x => x.clone(),
    }
}