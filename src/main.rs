use registerlang::{Chunk, *};

fn main() {
    let mut chunk = Chunk::init();

    let a = chunk.add_constant(Value::Num(1.0));
    let b = chunk.add_constant(Value::Num(2.0));
    let c = chunk.add_constant(Value::Num(3.0));
    let x = chunk.add_constant(Value::Bool(true));

    chunk.add_instruction(Instruction::make_xy(OpCode::Load as u8, 0, a), 1);
    chunk.add_instruction(Instruction::make_xy(OpCode::Load as u8, 1, b), 1);
    chunk.add_instruction(Instruction::make_xy(OpCode::Load as u8, 2, c), 1);
    chunk.add_instruction(Instruction::make_xy(OpCode::Load as u8, 5, x), 1);

    chunk.add_instruction(Instruction::make_xyz(OpCode::Add as u8, 0, 0, 1), 2);
    chunk.add_instruction(Instruction::make_xyz(OpCode::Mul as u8, 1, 0, 2), 3);
    chunk.add_instruction(Instruction::make_xyz(OpCode::Div as u8, 3, 1, 0), 4);
    chunk.add_instruction(Instruction::make_xyz(OpCode::Equal as u8, 4, 0, 2), 6);
    chunk.add_instruction(Instruction::make_xyz(OpCode::Equal as u8, 5, 5, 1), 6);
    chunk.add_instruction(Instruction::make_xyz(OpCode::Return as u8, 3, 0, 0), 99);

    let mut vm = VM::init(chunk, 6);
    vm.print_instructions();
    let res = vm.interpret();
    match res {
        Err(e) => eprintln!("{e}"),
        _ => println!("Exited successfully")
    }
    
    println!("Size of Instruction is {}", size_of::<Instruction>());
    println!("Size of Opcode is {}", size_of::<OpCode>());
    println!("Size of Value is {}", size_of::<Value>());
    println!("Size of Chunk is {}", size_of::<Chunk>());
    assert!(size_of::<Instruction>() == 4);
    return; 
}
