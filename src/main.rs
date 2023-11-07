#![feature(slice_as_chunks)]
use std::{fs, process::{self, Stdio}, env::args, iter};

fn main() {
    let mut registers = [0u8; 16];


    let mut memory = [0u8; 256];

    let pc: usize = 0;

    let path = args().nth(1).expect("No file path provided");

    let code = fs::read(&path).unwrap();
    let (instructions, []) = code.as_chunks::<4>() else {
        panic!("File size is not a multiple of 4")
    };

    println!("Source code:");
    instructions.iter().for_each(|instruction| {
        print_instruction(instruction)
    });
    loop {


        match instructions[pc][0] {
            
            _ => {
                print_instruction(&instructions[pc]);
                panic!()
            }
        }
    }
}

fn print_instruction(instruction: &[u8; 4]) {
    instruction.iter().for_each(|byte| {
        print!("{:02X} ", byte);
    });
    println!("");
}