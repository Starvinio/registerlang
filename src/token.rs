use std::fmt;

use crate::{LangError, Span};
/// A single token produced by [`Scanner`]
#[derive(Debug)]
pub struct LangToken {
    /// the type/category of this token
    pub ttype: TokenType,

    /// byte offset of token start in source string
    pub tspan: Span,
}
impl LangToken {
    pub fn new(ttype: TokenType, tspan: Span) -> Self {
        Self { ttype, tspan }
    }
    pub fn invalid_token_x(&self, filler: &str) -> LangError {
        return LangError::compile(self.tspan, format!("Invalid {}: {}", filler, self.ttype));
    }
}

/// Stores only the type of the token as a single byte
/// Token Data (number, booleans, strings) are parsed from src ptr
#[derive(Debug, PartialEq)]
pub enum TokenType {
    //  === Arithmetic Operators ===
    /// `+`
    Plus,

    /// `-`
    Minus,

    /// `*`
    Star,

    /// `/`
    Slash,

    // === Boolean Operators ===
    /// `!`
    Bang,

    /// `!=`
    BangEq,

    /// `=`
    Eq,

    /// `==`
    EqEq,

    /// `<`
    Lthen,

    /// `<=`
    LthenEq,

    /// `>`
    Gthen,

    /// `>=`
    GthenEq,

    // === Grouping ===
    /// `(`
    LParen,

    /// `)`
    RParen,

    // === Literals ===
    /// Num literal
    /// Value is parsed by compiler
    Num,

    // === Default Identifier ===
    // Could be variable or function name
    Identifier,

    /// Boolean 'true'
    True,

    /// Boolean 'false'
    False,

    /// Nil / null value
    /// Equals to 'false' on boolean checks
    NIL,

    // === Markers ===
    /// End of file marker
    EOF,

    /// Newline Marker
    /// Produced by both '\n' and '\r'
    NL,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // Arithmetic
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",

            // Boolean
            TokenType::Bang => "!",
            TokenType::BangEq => "!=",
            TokenType::Eq => "=",
            TokenType::EqEq => "==",
            TokenType::Lthen => "<",
            TokenType::LthenEq => "<=",
            TokenType::Gthen => ">",
            TokenType::GthenEq => ">=",

            // Grouping
            TokenType::LParen => "(",
            TokenType::RParen => ")",

            // Literals / keywords
            TokenType::Num => "<num>",
            TokenType::Identifier => "<identifier>",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::NIL => "nil",

            // Markers
            TokenType::EOF => "<eof>",
            TokenType::NL => "<nl>",
        };

        write!(f, "{}", s)
    }
}
