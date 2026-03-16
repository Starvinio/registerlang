use crate::{Chunk, Instruction, LangError, OpCode, Value};
const MAX_REGISTERS: usize = 256; 

pub struct VM {
    code: Chunk,
    registers: Vec<Value>,
    ip: usize, // Instruction pointer
}

impl VM {
    pub fn init(chunk:Chunk, register_size: u8) -> Self {
        return Self { 
            code: chunk, 
            registers: vec![Value::None; register_size as usize], 
            ip: 0 
        }
    }

    pub fn interpret(&mut self) -> Result<(), LangError> {
        while self.ip < self.code.instructions.len() {
            let instr = self.code.instructions[self.ip];
            self.exec_instruction(instr)?;
            self.ip += 1;
        }
        println!("{:?}", self.registers);
        Ok(())
    }

    fn exec_instruction(&mut self, instr: Instruction) -> Result<(), LangError> {
        let res = match OpCode::try_from(instr.opcode()) {
            Ok(OpCode::Return) => self.ret(instr.x())?,
            Ok(OpCode::Load) => self.load(instr.x(), instr.yz())?,
            Ok(OpCode::Add)=> self.add(instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Sub)=> self.sub(instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Mul)=> self.mul(instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Div)=> self.div(instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Equal)=> self.equal(instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Lthen)=> self.lthen(instr.x(), instr.y(), instr.z())?,
            Ok(OpCode::Gthen)=> self.gthen(instr.x(), instr.y(), instr.z())?,

            Err(b) => return Err(self.err_from_string(format!("Invalid OpCode: {}", b)))
        };
        return Ok(res)

    }

    pub fn print_instructions(&self) {
        println!("========= INSTRUCTIONS ==========");
        let mut last_line = None;

        for (ip, instr) in self.code.instructions.iter().enumerate() {
            let line = self.code.get_line(ip);

            if Some(line) != last_line {
                println!("{:>4}", line);
                last_line = Some(line);
            }
            println!("      {:04}  {}", ip, instr);
        }

        println!("=================================");
    }
    fn ret(&mut self, dest:u8) -> Result<(), LangError> {
        println!("return: {:?}", self.registers[dest as usize]);
        Ok(())
    }
    fn load(&mut self, dest: u8, idx: u16) -> Result<(), LangError> {
        if self.registers.len() < MAX_REGISTERS {
            self.registers[dest as usize] = self.code.constants[idx as usize].clone();
            Ok(())
        } else {
            Err(self.err_from_str("Register Overflow"))
        }
    }
    fn add(&mut self, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] + self.registers[b as usize];
        println!("register[{dest}] = {:?} + {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(s))
        }
        Ok(())
    }
    fn sub(&mut self, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] - self.registers[b as usize];
        println!("register[{dest}] = {:?} - {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(s))
        }
        Ok(())
    }
    fn mul(&mut self, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] * self.registers[b as usize];
        println!("register[{dest}] = {:?} * {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(s))
        }
        Ok(())
    }
    fn div(&mut self, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        let result = self.registers[a as usize] / self.registers[b as usize];
        println!("register[{dest}] = {:?} / {:?} = {:?}", self.registers[a as usize], self.registers[b as usize], result);

        match result {
            Ok(val) => self.registers[dest as usize] = val,
            Err(s) => return Err(self.err_from_string(s))
        }
        Ok(())
    }
    fn equal(&mut self, dest:u8, a:u8, b:u8)-> Result<(), LangError> {
        let a_val = self.registers[a as usize];
        let b_val = self.registers[b as usize];
        let res_bool = match (a_val, b_val) {
            (Value::Num(i), Value::Num(j)) => i == j,
            (Value::Bool(i), Value::Bool(j)) => i == j,
            (Value::None, Value::Bool(i)) | (Value::Bool(i), Value::None) => { i == false },
            _ => return Err(self.err_from_str("Invalid '==' comparison"))

        };
        self.registers[dest as usize] = Value::Bool(res_bool);
        Ok(())
    }
    fn lthen(&mut self, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        self.registers[dest as usize] = Value::Bool(
            match (self.registers[a as usize], self.registers[b as usize]) {
                (Value::Num(i), Value::Num(j)) => i < j,
                _ => return Err(self.err_from_str("Invalid '<' comparison"))
            }
        );
        Ok(())
    }
    fn gthen(&mut self, dest:u8, a:u8, b:u8) -> Result<(), LangError> {
        self.registers[dest as usize] = Value::Bool(
            match (self.registers[a as usize], self.registers[b as usize]) {
                (Value::Num(i), Value::Num(j)) => i > j,
                _ => return Err(self.err_from_str("Invalid '>' comparison"))
            }
        );
        Ok(())
    }

    fn err_from_str(&self, msg: &str) -> LangError {
        LangError::RuntimeError { line: self.code.get_line(self.ip), msg: msg.to_string() } 
    }
    fn err_from_string(&self, msg: String) -> LangError {
        LangError::RuntimeError { line: self.code.get_line(self.ip), msg } 
    }

    

    
}
