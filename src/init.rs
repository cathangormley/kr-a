use std::collections::HashMap;
use crate::error::KrEvalError;
use crate::kr::Kr;

use crate::operator::{Operator, Op};
use crate::primitive::{Primitive, Prim};
use crate::text::Text;

macro_rules! insert_operator {
    ($env:expr, $($text:expr => $op:expr),*) => {
        $($env.var.insert(Text::from_str($text), Kr::Op(Operator::new($op)));)*
    };
}

macro_rules! insert_primitive {
    ($env:expr, $($text:expr => $prim:expr),*) => {
        $($env.var.insert(Text::from_str($text), Kr::Prim(Primitive::new($prim)));)*
    };
}

pub fn init() -> Env {
    let mut env: Env = Env::new();
    env.var.insert(Text::from_str("one"), Kr::J(1));
    env.var.insert(Text::from_str("two"), Kr::J(2));
    env.var.insert(Text::from_str("alph"), Kr::C(b'a'));
    insert_primitive!(
        env,
        "first" => Prim::First,
        "last" => Prim::Last,
        "til" => Prim::Til,
        "value" => Prim::Value
    );
    insert_operator!(
        env,
        "+" => Op::Addition,
        "-" => Op::Subtraction,
        "*" => Op::Multiplication,
        "%" => Op::Division,
        ":" => Op::Assign,
        "," => Op::Join
    );
    env
}

#[derive(Clone)]
pub struct Env {
    // For now env is a hashmap of names to Kr variables
    // Later it can become a kr_tree
    pub var: HashMap<Text, Kr>,
    pub opts: Vec<String>,
}

impl Env {
    pub fn new() -> Self {
        let opts: Vec<String> = std::env::args().collect();
        Env { var: HashMap::new(), opts }
    }
    pub fn val(&self, v: &Kr) -> Result<Kr, KrEvalError> {
        match v {
            Kr::S(s) => {
                self.var.get(s).ok_or(KrEvalError::NotDefined).cloned()
            },
            _ => Err(KrEvalError::Type),
        }
    }
}