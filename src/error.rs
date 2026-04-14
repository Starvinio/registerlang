use std::fmt;

use crate::Span;

#[derive(Debug)]
pub struct LangError {
    espan: Span,
    emsg: String,
    etype: ErrorType,
}
#[derive(Debug)]
pub enum ErrorType {
    CompileError,
    RuntimeError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::CompileError => {
                write!(f, "Compile Error")
            }
            ErrorType::RuntimeError => {
                write!(f, "Runtime Error")
            }
        }
    }
}

impl LangError {
    pub fn compile(espan: Span, emsg: String) -> LangError {
        LangError {
            espan,
            emsg,
            etype: ErrorType::CompileError,
        }
    }
    pub fn runtime(espan: Span, emsg: String) -> LangError {
        LangError {
            espan,
            emsg,
            etype: ErrorType::RuntimeError,
        }
    }
    pub fn exit_code(&self) -> i32 {
        match self.etype {
            ErrorType::CompileError => 65,
            ErrorType::RuntimeError => 70,
        }
    }
    pub fn print_error(&self, src: &str) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        let (line, col) = self.span_to_loc(src);
        println!(
            "{}[{}:{}] {}: {}{}",
            RED, line, col, self.etype, self.emsg, RESET
        )
    }
    fn span_to_loc(&self, src: &str) -> (u32, u32) {
        let mut current_line = 1;
        let mut line_ptr = 0;
        let mut src_ptr = 0;
        while src_ptr < src.len() {
            if src_ptr as u32 == self.espan.start_u32() {
                let col = (src_ptr - line_ptr) as u32;
                return (current_line as u32, col);
            }
            if src.as_bytes()[src_ptr] == b'\n' {
                current_line += 1;
                line_ptr = src_ptr;
            }
            src_ptr += 1;
        }
        // Never reached, sentinel Value
        return (0, 0);
    }
}
