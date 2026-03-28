use crate::{Chunk, LangError, LangToken, Lexer, TokenType, Span};
pub struct Parser {
    lexer: Lexer,
    current: LangToken,
    previous: LangToken,
}
impl Parser {
    pub fn init(src: Box<str>) -> Self {
        Self { lexer:Lexer::init(src), current: LangToken::new(TokenType::EOF, Span::init(0, 0)), previous: LangToken::new(TokenType::EOF, Span::init(0, 0))}
    }
    pub fn compile(&mut self) -> Result<Chunk, LangError> {
        let tokens = self.lexer.token_stream()?;
        println!("{:#?}", tokens);

        // self.advance();
        // self.consume(TokenType::EOF, "Expect end of expression.");

        // DEBUG ONLY
        Ok(Chunk::init())
    }
    // fn consume(&mut self, expected: TokenType, message:&str) -> Result<(), LangError> {
    //     if matches!(&self.current.ttype, expected) {
    //         self.advance();
    //         return Ok(());
    //     }
    //     return Err(LangError::compile(self.current.tspan, message.to_string()))
    //
    // }
    // fn advance(&mut self) -> Result<(), LangError> {
    //     self.previous = self.current;
    //     self.current = self.lexer.emit_token()?;
    //     Ok(())
    // }
}
