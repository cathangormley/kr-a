use std::collections::HashMap;
use crate::kr::Kr;
use crate::Name;

pub fn init() -> Env {

    let mut env: Env = Env::new();
    env.var.insert(Name { text:"one".to_string().into_bytes() }, Kr::J(1));
    env.var.insert(Name { text:"two".to_string().into_bytes() }, Kr::J(2));

    env
}


pub struct Env {
    // For now env is a hashmap of names to Kr variables
    // Later it can become a kr_tree
    pub var: HashMap<Name, Kr>,
}

impl Env {
    pub fn new() -> Self {
        Env { var: HashMap::new() }
    }

    pub fn value<'a>(&'a self, x: &'a Kr ) -> &Kr {
        match x {
            Kr::Cv(t) => self.var.get(&Name { text:t.to_vec() }).unwrap_or(&Kr::J(0)),
            _ => x,
        }
    }
}