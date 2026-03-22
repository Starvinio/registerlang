
/// A single token produced by [`Scanner`]
pub struct LangToken {
    /// the type/category of this token
    pub ttype: TokenType,

    /// byte offset of token start in source string
    pub tptr: u32,
}
pub enum TokenType {
    //  === Arithmetic Operators ===
    
    /// +
    Plus, 

    /// -
    Minus,

    /// *
    Star,

    /// /
    Slash, 
          

    // === Boolean Operators === 

    /// =
    Equals,

    /// ==
    EqEquals, 

    /// <
    Lthen, 

    /// >
    Gthen, 

    // === Grouping === 

    /// (
    LParen,

    // )
    RParen,

    // === Literals === 

    /// Num literal (stored as 'f32')
    Num(f32), 

    /// Boolean literal (native bool)
    Bool(bool), 

    /// Nil / null value 
    /// Equals to 'false' on bool checks
    NIL,  

    /// End of file marker
    EOF,
}

