use std::fmt;

use crate::{LangError, LangToken, TokenType};

#[derive(Clone, Copy)]
pub struct Instruction(u32);

/*
     Instruction:
     [opcode] [   x  ] [   y  ] [   z  ]
     12345678 12345678 12345678 12345678
         1        2       3         4
*/

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opcode = OpCode::try_from(self.opcode()).unwrap();

        match opcode {
            OpCode::Load => {
                write!(
                    f,
                    "{:<8} {:>3} {:>3};",
                    format!("{:?}", opcode),
                    self.x(),
                    self.yz()
                )
            }
            _ => {
                write!(
                    f,
                    "{:<8} {:>3} {:>3} {:>3};",
                    format!("{:?}", opcode),
                    self.x(),
                    self.y(),
                    self.z()
                )
            }
        }
    }
}

impl Instruction {
    pub fn opcode(self) -> u8 {
        (self.0 >> 24) as u8
    }
    pub fn x(self) -> u8 {
        (self.0 >> 16) as u8
    }
    pub fn y(self) -> u8 {
        (self.0 >> 8) as u8
    }
    pub fn z(self) -> u8 {
        self.0 as u8
    }

    pub fn yz(self) -> u16 {
        self.0 as u16
    }

    pub fn make_xyz(opcode: u8, x: u8, y: u8, z: u8) -> Self {
        Instruction((opcode as u32) << 24 | (x as u32) << 16 | (y as u32) << 8 | z as u32)
    }

    pub fn make_xy(opcode: u8, x: u8, y: u16) -> Self {
        Instruction((opcode as u32) << 24 | (x as u32) << 16 | y as u32)
    }
    pub fn make_xx(opcode: u8, x: u8) -> Self {
        Instruction((opcode as u32) << 24 | (x as u32) << 16 | (x as u32) << 8)
    }
    pub fn make_x(opcode: u8, x: u8) -> Self {
        Instruction((opcode as u32) << 24 | (x as u32) << 16)
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Return = 0,
    Load = 1,
    LoadBool = 2,
    LoadNil = 3,
    Add = 4,
    Sub = 5,
    Mul = 6,
    Div = 7,
    Pow = 8,
    Neg = 9,
    Not = 10,
    Equal = 11,
    Lthen = 12,
    Lequal = 13,
}
impl TryFrom<u8> for OpCode {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0 => Ok(OpCode::Return),
            1 => Ok(OpCode::Load),
            2 => Ok(OpCode::LoadBool),
            3 => Ok(OpCode::LoadNil),
            4 => Ok(OpCode::Add),
            5 => Ok(OpCode::Sub),
            6 => Ok(OpCode::Mul),
            7 => Ok(OpCode::Div),
            8 => Ok(OpCode::Pow),
            9 => Ok(OpCode::Neg),
            10 => Ok(OpCode::Not),
            11 => Ok(OpCode::Equal),
            12 => Ok(OpCode::Lthen),
            13 => Ok(OpCode::Lequal),
            unknown => Err(unknown),
        }
    }
}
impl OpCode {
    /// Converts unary operators to opcodes
    pub fn un_op2opcode(op: &LangToken) -> Result<Self, LangError> {
        let opcode = match op.ttype {
            TokenType::Minus => OpCode::Neg,
            TokenType::Bang => OpCode::Not,
            _ => {
                return Err(LangError::compile(
                    op.tspan,
                    format!(
                        "Failed to convert unary operator '{}' to bytecode",
                        op.ttype
                    ),
                ));
            }
        };
        Ok(opcode)
    }
    /// Converts binary operators to opcodes
    pub fn bin_op2opcode(op: &LangToken) -> Result<Self, LangError> {
        let opcode = match op.ttype {
            TokenType::Plus => OpCode::Add,
            TokenType::Minus => OpCode::Sub,
            TokenType::Star => OpCode::Mul,
            TokenType::Slash => OpCode::Div,
            TokenType::Caret => OpCode::Pow,
            TokenType::EqEq => OpCode::Equal,
            TokenType::Lthen | TokenType::Gthen => OpCode::Lthen,
            TokenType::LthenEq | TokenType::GthenEq => OpCode::Lequal,
            _ => {
                return Err(LangError::compile(
                    op.tspan,
                    format!(
                        "Failed to convert binary operator '{}' to bytecode",
                        op.ttype
                    ),
                ));
            }
        };
        Ok(opcode)
    }
}
