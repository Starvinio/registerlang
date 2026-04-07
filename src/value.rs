use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Num(f32),
    Bool(bool),
    None
}

impl Add for Value {
    type Output = Result<Value, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a + b)),
            _ => Err(format!("Tried to add invalid types: {:?}, {:?}", self, rhs))
        }
    }
}
impl Sub for Value {
    type Output = Result<Value, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a - b)),
            _ => Err(format!("Tried to subtract invalid types: {:?}, {:?}", self, rhs))
        }
    }
}
impl Mul for Value {
    type Output = Result<Value, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a * b)),
            _ => Err(format!("Tried to multiply invalid types: {:?}, {:?}", self, rhs))
        }
    }
}
impl Div for Value {
    type Output = Result<Value, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a / b)),
            _ => Err(format!("Tried to divide invalid types: {:?}, {:?}", self, rhs))
        }
    }
}







