use std::collections::HashMap;
use crate::kr::Kr;

use crate::primitive::{Primitive, Prim};
use crate::text::Text;

pub fn init() -> Env {
    let mut env: Env = Env::new();
    env.var.insert(Text::from_str("one"), Kr::J(1));
    env.var.insert(Text::from_str("two"), Kr::J(2));
    env.var.insert(Text::from_str("alph"), Kr::C(b'a'));
    env.var.insert(Text::from_str("first"), Kr::Prim(Primitive::new(Prim::First)));
    env.var.insert(Text::from_str("last"), Kr::Prim(Primitive::new(Prim::Last)));
    env.var.insert(Text::from_str("til"), Kr::Prim(Primitive::new(Prim::Til)));
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
    pub fn val(&self, x: &Kr) -> Kr {
        match x {
            Kr::S(s) => {
                match self.var.get(s) {
                    Some(kr) => kr.clone(),
                    None => Kr::Null,
                }
            },
            _ => x.clone(),
        }
    }
}