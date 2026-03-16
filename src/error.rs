use std::fmt;

#[derive(Debug)]
pub enum LangError {
    CompileError { line: u32, col: u32, msg: String },
    RuntimeError { line: u32, msg: String},
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        match self {
            LangError::CompileError { line, col, msg } => {
                write!(f, "{}[{}:{}] Compile Error: {}{}", RED, line, col, msg, RESET)
            }
            LangError::RuntimeError { line, msg } => {
                write!(f, "{}[{}] Runtime Error: {}{}", RED, line, msg, RESET)
            }
        }
    }
}
