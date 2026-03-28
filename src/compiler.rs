use crate::{Chunk, LangError, LangToken, Lexer, TokenType};
pub struct Parser {
    lexer: Lexer,
    current: Option<LangToken>,
    previous: Option<LangToken>,
}
impl Parser {
    pub fn init(src: Box<str>) -> Self {
        Self { lexer:Lexer::init(src), current: None, previous: None}
    }
    pub fn compile(&mut self, source: String) -> Result<Chunk, LangError> {

        let mut lexer = Lexer::init(source.into_boxed_str());
        let tokens = lexer.token_stream()?;
        println!("{:#?}", tokens);

        self.advance();
        self.consume(TokenType::EOF, "Expect end of expression.");

        // DEBUG ONLY
        Ok(Chunk::init())
    }
    fn consume(&mut self, expected: TokenType, message:&str) -> Result<(), LangError> {
        if matches!(&self.current.ttype, expected) {
            self.advance();
            return Ok(());
        }
        return Err(LangError::compile(self.tokens[self.current].tptr, message.to_string()))

    }
    fn advance(&mut self) -> Result<(), LangError> {
        self.previous = self.current;
        self.current = Some(self.lexer.emit_token()?);
        Ok(())
    }
}
