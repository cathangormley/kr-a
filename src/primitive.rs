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
            Prim::Til => { (kr_til_wrapped, "til") },
            Prim::Enlist => { (kr_enlist_wrapped, "enlist") },
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

fn kr_til_wrapped(e:Env, args: Vec<Kr>) -> (Env, Kr) {
    if args.len() > 1 { return (e, Kr::Null) };
    (e, kr_til(&args[0]))
}

fn kr_til(x: &Kr) -> Kr {
    match x {
        Kr::J(n) => Kr::Jv((0..*n).collect()),
        x => x.clone()
    }
}


fn kr_enlist_wrapped(e:Env, args: Vec<Kr>) -> (Env, Kr) {
    if args.is_empty() {
        return (e, Kr::NN(args));
    }

    let kr = match &args[0] {
        Kr::I(_) => {
            if args.iter().all(|k| matches!(k, Kr::I(_))) {
                let iv = args.into_iter().map(|k| match k {
                    Kr::I(i) => i,
                    _ => unreachable!(),
                }).collect::<Vec<i32>>();
                Kr::Iv(iv)
            } else {
                Kr::NN(args)
            }
        }
        Kr::J(_) => {
            if args.iter().all(|k| matches!(k, Kr::J(_))) {
                let jv = args.into_iter().map(|k| match k {
                    Kr::J(j) => j,
                    _ => unreachable!(),
                }).collect::<Vec<i64>>();
                Kr::Jv(jv)
            } else {
                Kr::NN(args)
            }
        }
        Kr::E(_) => {
            if args.iter().all(|k| matches!(k, Kr::E(_))) {
                let ev = args.into_iter().map(|k| match k {
                    Kr::E(e) => e,
                    _ => unreachable!(),
                }).collect::<Vec<f32>>();
                Kr::Ev(ev)
            } else {
                Kr::NN(args)
            }
        }
        Kr::F(_) => {
            if args.iter().all(|k| matches!(k, Kr::F(_))) {
                let fv = args.into_iter().map(|k| match k {
                    Kr::F(f) => f,
                    _ => unreachable!(),
                }).collect::<Vec<f64>>();
                Kr::Fv(fv)
            } else {
                Kr::NN(args)
            }
        }
        Kr::C(_) => {
            if args.iter().all(|k| matches!(k, Kr::C(_))) {
                let cv = args.into_iter().map(|k| match k {
                    Kr::C(c) => c,
                    _ => unreachable!(),
                }).collect::<Vec<u8>>();
                Kr::Cv(cv)
            } else {
                Kr::NN(args)
            }
        }
        _ => Kr::NN(args),
    };
    (e, kr)
}