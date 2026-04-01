use crate::{Chunk, Instruction, LangError, LangToken, Lexer, OpCode, Span, TokenType, Value};
use std::mem;
/// Parser struct is the main handler that transforms input into bytecode
/// It handles the Lexer and uses it's Token output dynamically for parsing
/// Chunk is later used for input of the VM
/// The top of the register stack is tracked to avoid unused registers
/// [`stack_top`](Self::stack_top) should always point at an empty register
pub struct Parser {
    lexer: Lexer,
    previous: LangToken,
    current: LangToken,
    chunk: Chunk,
    stack_top: u8,
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
        Ok(Self { lexer, previous, current, chunk:Chunk::init(), stack_top:0 })
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

    // TODO: Add proper handling (including underline diagnostic) 
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

    /// Shifts [`previous`](Self::previous) and [`current`](Self::current) twice
    /// This is useful for expression parsing, as we ensure that current is an operator
    fn advance_twice(&mut self) -> Result<(), LangError> {
        self.previous = mem::replace(&mut self.current, self.lexer.emit_token()?);
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

    fn emit_instr(&mut self, instr:Instruction) {
        self.chunk.add_instruction(instr, self.previous.tspan);
    }
    fn emit_instrs(&mut self, instr1:Instruction, instr2:Instruction) {
        self.chunk.add_instruction(instr1, self.previous.tspan);
        self.chunk.add_instruction(instr2, self.previous.tspan);
    }


    /// Returns a valid register index for usage as u8
    /// Also increments structure variable stack top
    fn alloc_register(&mut self) -> u8 {
        // TODO: Ensure this function is able to return a valid u8
        let reg = self.stack_top;
        self.stack_top += 1;
        reg
    }

    /// Tries to parse number from source string via copied span produced by lexer
    /// If it succeeds, it inserts the number into the Chunk consts,
    /// adds a load instruction to the Chunk's bytecode and
    /// returns the register index of the added number as u8
    fn number(&mut self, span: Span) -> Result<u8, LangError> {
        let number = match self.lexer.src[span.start()..span.end()].parse::<f32>() {
            Ok(f) => f,
            Err(_) => return Err(LangError::compile(span, "Invalid numeric value".to_string()))
        };
        let const_idx = self.chunk.add_constant(Value::Num(number));
        
        let reg = self.alloc_register();
        self.chunk.add_instruction(Instruction::make_xy(OpCode::Load as u8, reg, const_idx), span);
        Ok(reg)
    
    }

    /// Helper function to start recursive execution of [`expression_bp`](Self::expression_bp)
    /// Starts with minimum binding power of 0
    fn expression(&mut self) {
        self.expression_bp(0);
    }
    /// Uses **Pratt Parsing** and a mix of recursive/iterative structure
    /// To generate instructions based on an expression
    fn expression_bp(&mut self, min_bp: u8) -> Result<u16, LangError> {
        let lhs_idx = match self.previous.ttype {
            TokenType::Num => self.number(self.previous.tspan)?,
            _ => return Err(LangError::compile(
                    self.previous.tspan,
                    "Unexpected start of expression".to_string()
                    ))
        };
        loop {
            let op = match self.current.ttype {
                TokenType::EOF => break,
                _ => &self.current
            };
            let (l_bp, r_bp) = self.infix_bp(op)?;

            if l_bp < min_bp { break }

            self.advance();

            let rhs_idx = self.expression_bp(r_bp);
        }

        Ok(lhs_idx)
    }
    fn infix_bp(&self, op: &LangToken) -> Result<(u8, u8), LangError> {
        let bp = match &op.ttype {
            TokenType::Plus | TokenType::Minus => (1, 2),
            TokenType::Star | TokenType::Slash => (3, 4),
            t => return Err(LangError::compile(op.tspan, format!("Unexpected Operator: '{}'", t)))
        };
        Ok(bp)
    }
}

