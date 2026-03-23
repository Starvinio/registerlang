use crate::{Lexer, LangError, Chunk};
pub struct Compiler {

}
impl Compiler {
    pub fn init() -> Self {
        Self {}
    }
    pub fn compile(&mut self, source: String) -> Result<Chunk, LangError> {
        let mut lexer = Lexer::init(source.into_boxed_str());
        let tokens = lexer.token_stream()?;
        println!("{:#?}", tokens);

        // DEBUG ONLY
        Ok(Chunk::init())
    }
}
