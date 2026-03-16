pub enum LangToken {
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

