use crate::{Scanner, LangError, Chunk};
pub struct Compiler {

}
impl Compiler {
    pub fn init() -> Self {
        Self {}
    }
    pub fn compile(&mut self, source: String) -> Result<Chunk, LangError> {
        let mut scanner = Scanner::init(source.into_boxed_str());
        let tokens = scanner.emit_tokens()?;

        // DEBUG ONLY
        Ok(Chunk::init())
    }
}
