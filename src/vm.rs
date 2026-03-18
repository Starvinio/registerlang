use std::collections::HashMap;
use crate::{Chunk, LangError, OpCode, Value};
const MAX_REGISTERS: usize = 256; 

pub struct VM {
    registers: Vec<Value>,
    _globals: HashMap<String, Value>,
    ip: usize, // Instruction pointer
}

impl VM {
    pub fn init() -> Self {
        return Self { 
            registers: Vec::with_capacity(8),
            _globals: HashMap::new(),
            ip: 0 
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), LangError> {
        while self.ip < chunk.instructions.len() {
            self.exec_current_instr(chunk)?;
            self.ip += 1
        }
        Ok(())
    }

    fn exec_current_instr(&mut self, chunk: &Chunk) -> Result<(), LangError> {
        let instr = chunk.instructions[self.ip];
        let res = match OpCode::try_from(instr.opcode()) {
            Ok(OpCode::Return) => self.ret(chunk, instr.x())?,
            Ok(OpCode::Load) => self.load(chunk, instr.x(), instr.yz())?,
            Ok(OpCode::Add)=> self.add(chunk, instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Sub)=> self.sub(chunk, instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Mul)=> self.mul(chunk, instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Div)=> self.div(chunk, instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Equal)=> self.equal(chunk, instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Lthen)=> self.lthen(chunk, instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Gthen)=> self.gthen(chunk, instr.x(), instr.y(), instr.z())?,

            Err(b) => return Err(self.err_from_string(chunk, format!("Invalid OpCode: {}", b)))
        };
        return Ok(res)

    }

    fn print_instructions(&self, chunk: &Chunk) {
        println!("========= INSTRUCTIONS ==========");
        let mut last_line = None;

        for (ip, instr) in chunk.instructions.iter().enumerate() {
            let line = chunk.get_line(ip);

            if Some(line) != last_line {
                println!("{:>4}", line);
                last_line = Some(line);
            }
            println!("      {:04}  {}", ip, instr);
        }

        println!("=================================");
    }
    fn ret(&mut self, _chunk:&Chunk, dest:u8) -> Result<(), LangError> {
        println!("return: {:?}", self.registers[dest as usize]);
        Ok(())
    }
    fn load(&mut self, chunk: &Chunk, dest: u8, idx: u16) -> Result<(), LangError> {
        if self.registers.len() < MAX_REGISTERS {
            self.registers[dest as usize] = chunk.constants[idx as usize].clone();
            Ok(())
        } else {
            Err(self.err_from_str(chunk, "Register Overflow"))
        }
    }
    fn add(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] + self.registers[b as usize];
        println!("register[{dest}] = {:?} + {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(chunk, s))
        }
        Ok(())
    }
    fn sub(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] - self.registers[b as usize];
        println!("register[{dest}] = {:?} - {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(chunk, s))
        }
        Ok(())
    }
    fn mul(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] * self.registers[b as usize];
        println!("register[{dest}] = {:?} * {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(chunk, s))
        }
        Ok(())
    }
    fn div(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] / self.registers[b as usize];
        println!("register[{dest}] = {:?} / {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(chunk, s))
        }
        Ok(())
    }
    fn equal(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8)-> Result<(), LangError> {
        let a_val = self.registers[a as usize];
        let b_val = self.registers[b as usize];
        let res_bool = match (a_val, b_val) {
            (Value::Num(i), Value::Num(j)) => i == j,
            (Value::Bool(i), Value::Bool(j)) => i == j,
            (Value::None, Value::Bool(i)) | (Value::Bool(i), Value::None) => { i == false },
            _ => return Err(self.err_from_str(chunk, "Invalid '==' comparison"))

        };
        self.registers[dest as usize] = Value::Bool(res_bool);
        Ok(())
    }
    fn lthen(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        self.registers[dest as usize] = Value::Bool(
            match (self.registers[a as usize], self.registers[b as usize]) {
                (Value::Num(i), Value::Num(j)) => i < j,
                _ => return Err(self.err_from_str(chunk, "Invalid '<' comparison"))
            }
        );
        Ok(())
    }
    fn gthen(&mut self, chunk:&Chunk, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        self.registers[dest as usize] = Value::Bool(
            match (self.registers[a as usize], self.registers[b as usize]) {
                (Value::Num(i), Value::Num(j)) => i > j,
                _ => return Err(self.err_from_str(chunk, "Invalid '>' comparison"))
            }
        );
        Ok(())
    }

    fn err_from_str(&self, chunk: &Chunk, msg: &str) -> LangError {
        LangError::RuntimeError { line: chunk.get_line(self.ip), msg: msg.to_string() } 
    }
    fn err_from_string(&self, chunk: &Chunk, msg: String) -> LangError {
        LangError::RuntimeError { line: chunk.get_line(self.ip), msg } 
    }

    

    
}
