/* 
Simplified instructionset based on CHIP-8
* instructions are 16 bits long
* registers are V0-VF
* all registers are 8 bits (0x00-0xFF) (0-255)
* VF is reserved for flags for some math ops (carry, borrow, etc)
* memory is addressed by 8 bits (0x00-0xFF) (0-255). Every address holds 1 byte (8 bits)
* address 0xFF are reserved for output. If a program writes to this address, it will be printed to the screen
* AA is a 8bit constant
* PC is the program counter (8 bits)


| Opcode | Type | Pseudo Code | Assembly | Description |
|-|-|-|-|-|
| 00NN | Flow | goto(NN) | jump NN | Set PC to NN |
| 10XY | Cond | if VX == VY | ifeq VX VY | If VX is equal to VY, skip the next instruction |
| 20XY | Cond | if VX != VY | ifneq VX VY | If register X is not equal to register Y, skip the next instruction |
| 30XY | Cond | if VX < VY | ifle VX VY | If register X is less than register Y, skip the next instruction |
| 40XY | Reg | VX = VY | setrr VX VY | Set VX to VY |
| 410X | Reg | VX = PC | setrpc VX | Set VX to PC |
| 5XNN | Reg | VX = NN | setrm | Set VX to NN |
| 6XAA | Reg | VX = AA | setrc VX AA | Set VX to AA |
| 700X | PC | PC = VX | setpcr VX | Set PC to VX |
| 8XNN | Mem | NN = VX | setmr NN VX | Set memory address NN to VX |
| 90XY | Math | VX = VX + VY | add VX VY | Add VX to VY and store the result in VX. If the result is greater than 255 (0xFF), set the carry flag VF to 1, otherwise set VF to 0 |
| 91XY | Math | VX = VX - VY | sub VX VY | Subtract VY from VX and store the result in VX. If VY is greater than VX, set the borrow flag VF to 1, otherwise set VF to 0 |
| A0XY | BitOp | VX = VX & VY | and VX VY | Set VX to VX AND VY |
| A1XY | BitOp | VX = VX \ VY | or VX VY | Set VX to VX OR VY |
| A2XY | BitOp | VX = VX ^ VY | xor VX VY | Set VX to VX XOR VY |
| A3XY | BitOp | VX = ~VX | not VX | Set VX to NOT VX |
 */


#![feature(slice_as_chunks)]
#![feature(iter_array_chunks)]
use std::fmt::Display;
use std::{fs, env::args};

fn main() {
    let path = args().nth(1).expect("No file path provided");

    let mut program_size = 0;

    let rom = u8Array::<512>::from(
        fs::read_to_string(&path)
        .expect("Bad path")
        .split_whitespace()
        .collect::<String>()
        .chars()
        .array_chunks::<2>()
        .map(|char| { 
            program_size += 1; 
            (char[0].to_digit(16).expect("not hex") * 16 + char[1].to_digit(16).expect("not hex")) as u8 
        })
        .collect::<Vec<u8>>()
    );

    program_size = program_size / 2;

    println!("Program size: {}", program_size);
    println!("ROM: {}", rom);

    let mut registers = u8Array::from([0u8; 16]);

    let mut memory = u8Array::from([0u8; 256]);

    let mut pc: u8 = 0;

    loop {
        if pc >= program_size {
            break;
        }

        let iu8 = [rom[pc * 2], rom[pc * 2 + 1]];
        let iu4 = [(iu8[0] & 0xF0) >> 4, iu8[0] & 0x0F, (iu8[1] & 0xF0) >> 4, iu8[1] & 0x0F];

        
/*         println!("Instruction: {:02X} {:02X}", iu8[0], iu8[1]);
        println!("PC: {}", pc);
        println!("Registers: {:?}", registers); */

        match iu4[0] {
            0x0 => { // 00NN jump NN
                let address = iu8[1];
                pc = address;
                continue;
            },
            0x1 => { // 10XY ifeq VX VY
                let vx = iu4[2];
                let vy = iu4[3];
                if registers[vx] == registers[vy] {
                    pc += 1;
                }
            },
            0x2 => { // 20XY ifneq VX VY
                let vx = iu4[2];
                let vy = iu4[3];
                if registers[vx] != registers[vy] {
                    pc += 1;
                }
            },
            0x3 => { // 30XY ifle VX VY
                let vx = iu4[2];
                let vy = iu4[3];
                if registers[vx] <= registers[vy] {
                    pc += 1;
                }
            },
            0x4 => {
                match iu4[1] {
                    0x0 => { // 40XY setrr VX VY
                        let vx = iu4[2];
                        let vy = iu4[3];
                        registers[vx] = registers[vy];
                    },
                    0x1 => { // 410X setrpc VX
                        let vx = iu4[3];
                        pc = registers[vx];
                    },
                    _ => {
                        print_instruction(&iu8, &iu4);
                        panic!("Unknown instruction")
                    },
                }
            },
            0x5 => { // 5XNN setrm VX NN
                let vx = iu4[2];
                let nn = iu8[1];
                registers[vx] = memory[nn];
            },
            0x6 => { // 6XAA setrc VX AA
                let vx = iu4[1];
                let aa = iu8[1];
                registers[vx] = aa;
            },
            0x7 => { // 700X setpcr VX
                let vx = iu4[3];
                pc = registers[vx];
            },
            0x8 => { // 8XNN setmr NN VX
                let nn = iu8[1];
                let vx = iu4[1];
                if nn == 0xFF {
                    println!("{}", registers[vx]);
                }
                memory[nn] = registers[vx];
            },
            0x9 => { 
                match iu4[1] {
                    0x0 => { // 90XY add VX VY
                        let vx = iu4[2];
                        let vy = iu4[3];
                        let result = registers[vx] as u16 + registers[vy] as u16;
                        registers[vx] = result as u8;
                        registers[0xF] = if result > 0xFF { 1 } else { 0 };
                    },
                    0x1 => { // 91XY sub VX VY
                        let vx = iu4[2];
                        let vy = iu4[3];
                        let result = registers[vx] as i16 - registers[vy] as i16;
                        registers[vx] = result as u8;
                        registers[0xF] = if result < 0 { 1 } else { 0 };
                    },
                    _ => {
                        print_instruction(&iu8, &iu4);
                        panic!("Unknown instruction")
                    },
                }
            },
            0xA => {
                match iu4[1] {
                    0x0 => { // A0XY and VX VY
                        let vx = iu4[2];
                        let vy = iu4[3];
                        registers[vx] = registers[vx] & registers[vy];
                    },
                    0x1 => { // A1XY or VX VY
                        let vx = iu4[2];
                        let vy = iu4[3];
                        registers[vx] = registers[vx] | registers[vy];
                    },
                    0x2 => { // A2XY xor VX VY
                        let vx = iu4[2];
                        let vy = iu4[3];
                        registers[vx] = registers[vx] ^ registers[vy];
                    },
                    0x3 => { // A3XY not VX
                        let vx = iu4[2];
                        registers[vx] = !registers[vx];
                    },
                    _ => {
                        print_instruction(&iu8, &iu4);
                        panic!("Unknown instruction")
                    },
                }
            },
            _ => {
                print_instruction(&iu8, &iu4);
                panic!("Unknown instruction")
            },
        }

        pc += 1;
    }
}

fn print_instruction(iu8: &[u8; 2], iu4: &[u8; 4]) {
    println!("iu8: {:02X} {:02X}", iu8[0], iu8[1]);
    println!("iu4: {:?}", iu4);
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
struct u8Array<const N: usize> {
    data: [u8; N]
}

use std::ops::{Index, IndexMut};

impl<const N: usize> Display for u8Array<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        self.data.iter().for_each(|byte| {
            string.push_str(&format!("{:02X} ", byte));
        });
        write!(f, "{}", string)
    }
    
}

impl<const N:usize> From<Vec<u8>> for u8Array<N> {
    fn from(vec: Vec<u8>) -> Self {
        let mut array = [0u8; N];
        vec.iter().enumerate().for_each(|(i, byte)| array[i] = *byte);
        Self{ data: array }
    }
}

impl<const N:usize> From<[u8; N]> for u8Array<N> {
    fn from(array: [u8; N]) -> Self {
        Self{ data: array }
    }
}

impl<const N: usize> Index<u8> for u8Array<N> {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl<const N: usize> IndexMut<u8> for u8Array<N> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}