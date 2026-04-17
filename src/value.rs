use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Num(f64),
    Bool(bool),
    NIL,
}
impl Value {
    pub fn val_as_num(&self) -> Result<f64, String> {
        match self {
            Value::Num(f) => Ok(f.clone()),
            _ => Err(format!("Tried to parse {:?} into Number(f64)", self)),
        }
    }
    pub fn val_as_bool(&self) -> Result<bool, String> {
        match self {
            Value::Bool(b) => Ok(b.clone()),
            _ => Err(format!("Tried to parse {:?} into Number(f64)", self)),
        }
    }
    pub fn negate_val(&self) -> Result<f64, String> {
        match self {
            Value::Num(f) => Ok(-f.clone()),
            _ => Err(format!("Tried to negate value of type {:?}", self)),
        }
    }
    pub fn invert_val(&self) -> bool {
        match self {
            Value::Bool(f) if !f => true,
            Value::NIL => true,
            _ => false,
        }
    }
}

impl Add for Value {
    type Output = Result<Value, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a + b)),
            _ => Err(format!("Tried to add invalid types: {:?}, {:?}", self, rhs)),
        }
    }
}
impl Sub for Value {
    type Output = Result<Value, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a - b)),
            _ => Err(format!(
                "Tried to subtract invalid types: {:?}, {:?}",
                self, rhs
            )),
        }
    }
}
impl Mul for Value {
    type Output = Result<Value, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a * b)),
            _ => Err(format!(
                "Tried to multiply invalid types: {:?}, {:?}",
                self, rhs
            )),
        }
    }
}
impl Div for Value {
    type Output = Result<Value, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Num(a), Value::Num(b)) => return Ok(Value::Num(a / b)),
            _ => Err(format!(
                "Tried to divide invalid types: {:?}, {:?}",
                self, rhs
            )),
        }
    }
}
