use std::collections::HashMap;
use crate::kr::Kr;

pub fn init() -> Env {

    let mut env: Env = Env::new();
    env.var.insert("one".to_string().into_bytes(), Kr::J(1));
    env.var.insert("two".to_string().into_bytes(), Kr::J(2));

    env
}

#[derive(Clone)]
pub struct Env {
    // For now env is a hashmap of names to Kr variables
    // Later it can become a kr_tree
    pub var: HashMap<Vec<u8>, Kr>,
}

impl Env {
    pub fn new() -> Self {
        Env { var: HashMap::new() }
    }

    pub fn value<'a>(&'a self, x: &'a Kr ) -> &Kr {
        match x {
            Kr::S(t) => {
                match self.var.get(t) {
                    Some(kr) => kr,
                    None => &Kr::Null
                }
            },
            _ => x,
        }
    }
}