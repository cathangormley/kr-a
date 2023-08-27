use crate::operator::Operator;

#[derive(Clone, Debug)]
pub enum Kr {
    I(i32), Iv(Vec<u8>),         // Integer
    J(i64),         // Long
    E(f32),         // Real
    F(f64),         // Float
    Op(Operator),   // Operator
    C(u8),          // Character
    Cv(Vec<u8>)     // Character vector (string)
}