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