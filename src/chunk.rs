use crate::{Instruction, Span, Value};

pub struct Chunk {
    pub instructions: Vec<Instruction>, // OpCodes + Register Indices
    pub constants: Vec<Value>,          // Values loaded from source
    pub ispan: Vec<Span>,               // Span in proportion to instruction
}
impl Chunk {
    pub fn init() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            ispan: Vec::new(),
        }
    }

    // Adds a value to the constants Vector and returns the index
    // Index can later be used for loading the value into a register
    pub fn add_constant(&mut self, constant: Value) -> u16 {
        self.constants.push(constant);
        return (self.constants.len() - 1) as u16;
    }
    pub fn add_instruction(&mut self, instruction: Instruction, pos: Span) {
        self.instructions.push(instruction);
        self.ispan.push(pos);
    }
    pub fn get_span(&self, instr_indx: usize) -> Span {
        match self.ispan.get(instr_indx) {
            Some(span) => return *span,
            None => return Span::zero(),
        }
    }
}
