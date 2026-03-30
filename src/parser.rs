use crate::{Chunk, Instruction, LangError, LangToken, Lexer, Span, TokenType};
use std::mem;
pub struct Parser {
    lexer: Lexer,
    previous: LangToken,
    current: LangToken,
    chunk: Chunk,
}
impl Parser {
    /// Init function already lexes the first two tokens
    /// into [`current`](Self::current) and [`previous`](Self::previous)
    /// Since lexing tokens can fail, function returns [`Result<>`]
    pub fn init(src: Box<str>) -> Result<Self, LangError> {
        if src.len() < 1 { return Err(LangError::compile(Span::init(0, 0), "Tried to compile empty file".to_string())) }
        let mut lexer = Lexer::init(src);
        let previous = lexer.emit_token()?;
        let current = lexer.emit_token()?;
        Ok(Self { lexer, previous, current, chunk:Chunk::init() })
    }

    /// Entry point for compilation phase of the interpreter
    /// Function consumes parser due to lack of reference
    pub fn compile(mut self) -> Result<Chunk, LangError> {
        let tokens = self.lexer.token_stream()?;
        println!("{:#?}", tokens);

        self.advance();
        self.consume(TokenType::EOF, "Expect end of expression.");

        Ok( self.chunk )
    }

    // TODO: Add proper handling (including underline diagnostic) later
    fn err_at_curr(&self, emsg: &str) -> LangError {
        LangError::compile(self.current.tspan, emsg.to_string())
    }
    fn err_at_prev(&self, emsg: &str) -> LangError {
        LangError::compile(self.previous.tspan, emsg.to_string())
    }
    
    /// Replaces [`previous`](Self::previous) with current
    /// Replaces [`current`](Self::current) with next token
    fn advance(&mut self) -> Result<(), LangError> {
        self.previous = mem::replace(&mut self.current, self.lexer.emit_token()?);
        Ok(())
    }

    fn consume(&mut self, expected: TokenType, message:&str) -> Result<(), LangError> {
        if matches!(&self.current.ttype, expected) {
            self.advance();
            Ok(())
        } else {
            Err(LangError::compile(self.current.tspan, message.to_string()))
        }

    }

    fn expression(&mut self) {
    }

    fn emit_instr(&mut self, instr:Instruction) {
        self.chunk.add_instruction(instr, self.previous.tspan);
    }
    fn emit_instrs(&mut self, instr1:Instruction, instr2:Instruction) {
        self.chunk.add_instruction(instr1, self.previous.tspan);
        self.chunk.add_instruction(instr2, self.previous.tspan);
    }
}
