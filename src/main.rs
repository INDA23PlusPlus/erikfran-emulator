#![feature(slice_as_chunks)]
use std::{fs, process::{self, Stdio}, env::args, iter};

fn main() {
    let path = args().nth(1).expect("No file path provided");

    let code = fs::read(&path).unwrap();
    let (instructions, []) = code.as_chunks::<2>() else {
        panic!("File size is not a multiple of 4")
    };

    println!("Source code:");
    instructions.iter().for_each(|instruction| {
        print_instruction(instruction)
    });

    let mut registers = [0u8; 16];

    let mut memory = [0u8; 256];

    let mut pc: usize = 0;

    loop {
        i = instructions[pc];
        match i[0] >> 4 {
            0x00 => { // 00NN jump NN
                let address = i[1];
                pc = address as usize;
            },
            0x01 => { // 10XY ifeq VX VY
                let vx = (i[1] << 4) >> 4;
                let vy = i[1] >> 4;
            },
            _ => {
                print_instruction(&instructions[pc]);
                panic!()
            }
        }

        pc += 1;
    }
}

fn print_instruction(instruction: &[u8; 4]) {
    instruction.iter().for_each(|byte| {
        print!("{:02X} ", byte);
    });
    println!("");
}

impl first_4bits for u8 {
    
}