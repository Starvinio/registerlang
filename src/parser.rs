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
        if src.len() < 1 {
            return Err(LangError::compile(
                Span::zero(),
                "Tried to compile empty file".to_string(),
            ));
        }
        let mut lexer = Lexer::init(src);
        let previous = lexer.emit_token()?;
        let current = lexer.emit_token()?;
        Ok(Self {
            lexer,
            previous,
            current,
            chunk: Chunk::init(),
            stack_top: 0,
        })
    }

    /// Entry point for compilation phase of the interpreter
    /// Function consumes parser
    pub fn compile(mut self) -> Result<Chunk, LangError> {
        self.expression()?;
        self.consume(
            TokenType::EOF,
            self.current.tspan,
            "Expect end of expression.",
        )?;

        Ok(self.chunk)
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
        self.previous = self.lexer.emit_token()?;
        self.current = self.lexer.emit_token()?;
        Ok(())
    }

    fn consume(&mut self, expected: TokenType, span: Span, message: &str) -> Result<(), LangError> {
        if self.current.ttype == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(LangError::compile(span, message.to_string()))
        }
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
        let number: f64 = match self.lexer.src[span.start()..span.end()].parse::<f64>() {
            Ok(f) => f,
            Err(_) => {
                return Err(LangError::compile(
                    span,
                    "Invalid numeric value".to_string(),
                ));
            }
        };
        let const_idx = self.chunk.add_constant(Value::Num(number));

        let reg = self.alloc_register();
        self.chunk.add_instruction(
            Instruction::make_xy(OpCode::Load as u8, reg, const_idx),
            span,
        );
        Ok(reg)
    }

    fn boolean(&mut self, span: Span) -> u8 {
        let val = match self.previous.ttype {
            TokenType::True => true,
            _ => false,
        };
        let const_idx = self.chunk.add_constant(Value::Bool(val));

        let reg = self.alloc_register();
        self.chunk.add_instruction(
            Instruction::make_xy(OpCode::Load as u8, reg, const_idx),
            span,
        );
        reg
    }

    /// Helper function to start recursive execution of [`expression_bp`](Self::expression_bp)
    /// Starts with minimum binding power of 0
    fn expression(&mut self) -> Result<(), LangError> {
        self.expression_bp(0)?;
        Ok(())
    }

    /// Uses **Pratt Parsing** with a mix of recursive/iterative structure
    /// and generates instructions based on an expression.
    /// Returns register idx of lhs or the register idx of the outer result
    /// Heavily Inspired by:
    /// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
    fn expression_bp(&mut self, min_bp: u8) -> Result<u8, LangError> {
        let mut lhs_reg = match self.previous.ttype {
            TokenType::Num => self.number(self.previous.tspan)?,
            TokenType::True | TokenType::False | TokenType::NIL => {
                self.boolean(self.previous.tspan)
            }
            TokenType::LParen => {
                let l_span = self.previous.tspan;
                self.advance()?;
                let res = self.expression_bp(0)?;
                self.consume(TokenType::RParen, l_span, "Unmatched closing delimiter '('")?;
                res
            }
            _ => {
                let ((), r_bp) = match self.prefix_bp() {
                    Some(res) => res,
                    None => {
                        return Err(LangError::compile(
                            self.previous.tspan,
                            format!("Unexpected start of expression: {:?}", self.previous),
                        ));
                    }
                };
                let opcode = match self.previous.ttype {
                    TokenType::Minus => OpCode::Neg,
                    _ => OpCode::Not, // Fine because we only have 2 prefix operators
                };
                self.advance()?;
                let rhs = self.expression_bp(r_bp)?;
                self.unary_op(opcode, rhs, self.previous.tspan)
            }
        };
        loop {
            let (l_bp, r_bp, invert) = match self.infix_bp() {
                Some(bp) => bp,
                None => break,
            };

            if l_bp < min_bp {
                break;
            }

            let expr_start = self.previous.tspan.start(); // stored for better debugging

            let opcode = OpCode::op2opcode(&self.current)?;

            // Advance twice to that prev = num and curr = op
            self.advance_twice()?;

            let rhs_reg = self.expression_bp(r_bp)?;
            let expr_span = Span::start_end(expr_start, self.previous.tspan.end());

            lhs_reg = self.binary_op(opcode, lhs_reg, rhs_reg, expr_span, invert);
            self.free_register();
        }

        Ok(lhs_reg)
    }

    /// Assigns binding power to a given token assuming its an operator
    /// Returns invert flag as true if invertation is desirable
    /// if it's not an operator, it function returns None
    fn infix_bp(&mut self) -> Option<(u8, u8, bool)> {
        let mut invert_flag = false;
        let (lhs, rhs) = match &self.current.ttype {
            TokenType::Eq => (2, 1),

            TokenType::EqEq | TokenType::Lthen | TokenType::LthenEq | TokenType::BangEq => (3, 4),

            // a > b => b < a
            TokenType::Gthen | TokenType::GthenEq => {
                invert_flag = true;
                (3, 4)
            }

            TokenType::Plus | TokenType::Minus => (5, 6),

            TokenType::Star | TokenType::Slash => (7, 8),

            TokenType::Caret => (9, 10),
            _ => return None,
        };
        Some((lhs, rhs, invert_flag))
    }

    /// Returns a dummy for l_bp and a high precedence r_bp
    /// Also handles all cases for an invalid lhs token
    fn prefix_bp(&self) -> Option<((), u8)> {
        let bp = match self.previous.ttype {
            TokenType::Plus | TokenType::Minus | TokenType::Bang => ((), 8),
            _ => return None,
        };
        Some(bp)
    }

    fn binary_op(&mut self, op: OpCode, lhs_reg: u8, rhs_reg: u8, span: Span, invert: bool) -> u8 {
        if invert {
            self.chunk.add_instruction(
                Instruction::make_xyz(op as u8, lhs_reg, rhs_reg, lhs_reg),
                span,
            );
        } else {
            self.chunk.add_instruction(
                Instruction::make_xyz(op as u8, lhs_reg, lhs_reg, rhs_reg),
                span,
            );
        }
        lhs_reg
    }

    fn unary_op(&mut self, op: OpCode, reg: u8, span: Span) -> u8 {
        self.chunk
            .add_instruction(Instruction::make_xx(op as u8, reg), span);
        reg
    }
}
