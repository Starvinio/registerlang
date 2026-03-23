/// A single token produced by [`Scanner`]
#[derive(Debug)]
pub struct LangToken {
    /// the type/category of this token
    pub ttype: TokenType,

    /// byte offset of token start in source string
    pub tptr: u32,
}

/// Stores only the type of the token as a single byte
/// Token Data (number, booleans, strings) are parsed from src ptr
#[derive(Debug)]
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

