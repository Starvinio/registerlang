use crate::{LangError};
pub enum LangToken {
    // Arithmetic Operators
    Plus, // + 
    Minus, // -
    Star, // *
    Slash, // /
          
    // Boolean Operators
    EqEquals, // ==
    Lthen, // <
    Gthen, // >

    // Blocks
    LParen, // (
    RParen, // )

    // Literals
    Num(f32), // float
    Bool(bool), // true/false
    NIL,  
}


pub struct Scanner {
    src: Box<str>,
    line: u32,
    current: usize
}

impl Scanner {
    pub fn init(src: Box<str>) -> Self {
        Self {
            src,
            line: 1,
            current:0
        }
    }
    pub fn emit_tokens(&mut self) -> Result<Vec<LangToken>, LangError> {
        let mut buf = Vec::with_capacity(16);
        while self.current < self.src.len() {
            buf.push(self.next_token()?) 
        }
        Ok(buf)
    }
    fn next_token(&mut self) -> Result<LangToken, LangError> {

        Ok(LangToken::NIL)
    }
    fn peek(&mut self) -> u8 {
        return self.src.as_bytes()[self.current+1]
    }
}
