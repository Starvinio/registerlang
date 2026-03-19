use crate::{LangError};

pub struct LangToken {
    pub ttype: TokenType,
    pub tptr: u32,
}
pub enum TokenType {
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
    ptr: usize
}

impl Scanner {
    pub fn init(src: Box<str>) -> Self {
        Self {
            src,
            ptr:0
        }
    }
    pub fn emit_tokens(&mut self) -> Result<Vec<LangToken>, LangError> {
        let mut buf = Vec::with_capacity(16);
        while self.ptr < self.src.len() {
            buf.push(self.next_token()?) 
        }
        Ok(buf)
    }
    
    // returns true if current is EOF char
    fn is_at_end(&self) -> bool {
        return self.src.as_bytes()[self.ptr] == b'\0'
    }
    
    // advances if current char matches expected
    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.src.as_bytes()[self.ptr] != expected as u8 {
            return false
        }
        self.ptr+=1;
        true
    }
    fn next_token(&mut self) -> Result<LangToken, LangError> {

        Ok(self.token(TokenType::NIL))
    }
    fn peek(&self) -> u8 {
        return self.src.as_bytes()[self.ptr]
    }
    fn peek_next(&self) -> u8 {
        return self.src.as_bytes()[self.ptr+1]
    }
    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\t' | b'\r' => self.ptr+=1,
                b'\n' => todo!("Add sum farkin newline detector"),
                _ => return
            }
        }
    }
    fn token(&self, ttype: TokenType) -> LangToken {
        LangToken {
            ttype,
            tptr: self.ptr as u32,
        }
    }
}
