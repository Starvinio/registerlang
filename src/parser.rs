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
        println!("Parser struct\nprevious: {:?}\ncurrent: {:?}\n", previous, current);
        Ok(Self { lexer, previous, current, chunk:Chunk::init(), stack_top: 0 })
    }

    /// Entry point for compilation phase of the interpreter
    /// Function consumes parser
    pub fn compile(mut self) -> Result<Chunk, LangError> {
        self.expression()?;
        self.consume(TokenType::EOF, "Expect end of expression.")?;

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
    /// Should **ONLY** be called if stack top is less than 255!
    fn alloc_register(&mut self) -> u8 {
        // TODO: Ensure this function is able to return a valid u8
        let reg = self.stack_top;
        self.stack_top += 1;
        reg
    }

    /// Simply decreases the stack top
    /// If a value was previously there, it will later be overwritten
    /// Should **ONLY** be called if stack top is greater than 0!
    fn free_register(&mut self) {
        self.stack_top -= 1;
    }

    /// Tries to parse number from source string via copied span produced by lexer
    /// If it succeeds, it inserts the number into the Chunk consts,
    /// adds a load instruction to the Chunk's bytecode and
    /// returns the register index of the added number as u8
    fn number(&mut self, span: Span) -> Result<u8, LangError> {
        let number: f32 = match self.lexer.src[span.start()..span.end()].parse::<f32>() {
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
    fn expression(&mut self) -> Result<(), LangError> {
        self.expression_bp(0)?;
        Ok(())
    }
    /// Uses **Pratt Parsing** and a mix of recursive/iterative structure
    /// To generate instructions based on an expression
    /// Returns register idx of lhs or the register idx of the outer result
    fn expression_bp(&mut self, min_bp: u8) -> Result<u8, LangError> {
        let lhs_reg = match self.previous.ttype {
            TokenType::Num => self.number(self.previous.tspan)?,
            _ => return Err(LangError::compile(
                    self.previous.tspan,
                    format!("Unexpected start of expression: {:?}", self.previous)
                    ))
        };
        loop {
            let op = match self.current.ttype {
                TokenType::EOF => break,
                TokenType::Plus | TokenType::Minus |
                    TokenType::Star | TokenType::Slash => &self.current.ttype,
                _ => return Err(self.err_at_curr("Invalid Operator"))
            };
            let (l_bp, r_bp) = self.infix_bp(&op)?;

            if l_bp < min_bp { break }

            // store pos of lhs for better debugging
            let lhs_start = self.previous.tspan.start();
            // store opcode of current operator
            let opcode_expr = self.op2opcode(&self.current)?;
            // Advance twice to that prev = num and curr = op
            self.advance_twice()?;

            let rhs_reg = self.expression_bp(r_bp)?;
            let expr_span = Span::init(lhs_start, self.previous.tspan.end() - lhs_start);

            return Ok(self.binary_op(opcode_expr, lhs_reg, rhs_reg, expr_span));
        }

        Ok(lhs_reg)
    }
    fn infix_bp(&self, op: &TokenType) -> Result<(u8, u8), LangError> {
        let bp = match &op {
            TokenType::Plus | TokenType::Minus => (1, 2),
            TokenType::Star | TokenType::Slash => (3, 4),
            _ => return Err(self.err_at_curr("Invalid Operator"))
        };
        Ok(bp)
    }

    fn op2opcode(&self, op: &LangToken) -> Result<OpCode, LangError> {
        let opcode = match op.ttype {
            TokenType::Plus => OpCode::Add,
            TokenType::Minus => OpCode::Sub,
            TokenType::Star => OpCode::Mul,
            TokenType::Slash => OpCode::Div,
            _ => return Err(LangError::compile(op.tspan, format!("Failed to convert operator {} to bytecode", op.ttype)))
        };
        Ok(opcode)
    }
        
    fn binary_op(&mut self, op: OpCode, lhs_reg: u8, rhs_reg: u8, span: Span) -> u8 {
        self.chunk.add_instruction( Instruction::make_xyz(op as u8, lhs_reg, lhs_reg, rhs_reg), span );
        lhs_reg 
    }
}

