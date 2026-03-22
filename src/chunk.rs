use crate::{Value, Instruction};


pub struct Chunk {
    pub instructions: Vec<Instruction>, // OpCodes + Register Indices
    pub constants: Vec<Value>, // Values loaded from source
    pub src_pos: Vec<u32>, // line, occurences
}
impl Chunk {
    pub fn init() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            src_pos: Vec::new(),
        }
    }

    // Adds a value to the constants Vector and returns the index
    // Index can later be used for loading the value into a register
    pub fn add_constant(&mut self, constant: Value) -> u16 {
        self.constants.push(constant);
        return ( self.constants.len() - 1 ) as u16
    }
    pub fn add_instruction(&mut self, instruction: Instruction, pos: u32) {
        self.instructions.push(instruction);
        self.src_pos.push(pos);
    }
    pub fn get_line(&self, instr_indx: usize) -> u32 {
        match self.src_pos.get(instr_indx) {
            Some(line) => return *line,
            None => return 0
        }
    }
}
