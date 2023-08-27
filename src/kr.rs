use crate::operator::Operator;

#[derive(Clone, Debug)]
pub enum Kr {
    I(i32),
    J(i64),
    E(f32),
    F(f64),
    Op(Operator)
}