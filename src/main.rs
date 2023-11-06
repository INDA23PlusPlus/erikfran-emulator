use std::{fs, process::{self, Stdio}, env::args, iter};

#[repr(u8)]
enum Instructions {
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JE,
    AND,
    OR,
    XOR,
    NOT,
}

fn main() {
    let mut registers = [0u8; 4];

    let mut memory = [0u8; 256];

    let path = args().nth(1).expect("No file path provided");
    let code = fs::read(&path).unwrap();

    println!("Source code:\n\n{:?}\n", code);

    code.iter().for_each(|byte| {
        print!("{:02X} ", byte);
    });
    for byte in code.iter() {
        match byte {
            _ => {}
        }
    }
}
