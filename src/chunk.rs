use crate::{Value, Instruction};


pub struct Chunk {
    pub instructions: Vec<Instruction>, // OpCodes + Register Indices
    pub constants: Vec<Value>, // Values loaded from source
    pub lines: Vec<(u32, u8)>, // line, occurences
}
impl Chunk {
    pub fn init() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    // Adds a value to the constants Vector and returns the index
    // Index can later be used for loading the value into a register
    pub fn add_constant(&mut self, constant: Value) -> u16 {
        self.constants.push(constant);
        return ( self.constants.len() - 1 ) as u16
    }
    pub fn add_instruction(&mut self, instruction: Instruction, line: u32) {
        self.instructions.push(instruction);
        self.add_line(line);        
    }
    fn add_line(&mut self, line: u32) {
        match self.lines.last_mut() {
            Some((last_line, occurences)) if *last_line == line => {
                *occurences += 1;
            }
            _ => self.lines.push((line, 1))
        }
    }
    pub fn get_line(&self, instr_indx: usize) -> u32 {
        let mut i = 0;
        let mut count = 0;
        while i < self.lines.len() {
            let current = self.lines[i];
            count += current.1 as usize;
            if count >= instr_indx + 1 {
                return current.0 as u32;
            }
            i += 1
        }

        return 0;
    }
}
