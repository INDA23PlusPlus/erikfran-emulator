use std::{fs, process::{self, Stdio}, env::args, iter};

fn main() {
    let mut registers = [0u8; 16];

    let mut memory = [0u8; 4096];

    let path = args().nth(1).expect("No file path provided");
    let code = fs::read(&path).unwrap();

    println!("Source code:\n\n{:?}\n", code);

    code.iter().for_each(|byte| {
        println!("{:02X} ", byte);
    });
    for instruction in code.chunks(3) {
        match instruction[0] {
            
            _ => {panic!("Unknown instruction: {:02X}", instruction[0])}
        }
    }
}
