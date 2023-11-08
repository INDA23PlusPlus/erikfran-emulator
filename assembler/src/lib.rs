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
| 8XNN | Mem | NN = VX | setmr VX NN | Set memory address NN to VX |
| 90XY | Math | VX = VX + VY | add VX VY | Add VX to VY and store the result in VX. If the result is greater than 255 (0xFF), set the carry flag VF to 1, otherwise set VF to 0 |
| 91XY | Math | VX = VX - VY | sub VX VY | Subtract VY from VX and store the result in VX. If VY is greater than VX, set the borrow flag VF to 1, otherwise set VF to 0 |
| A0XY | BitOp | VX = VX & VY | and VX VY | Set VX to VX AND VY |
| A1XY | BitOp | VX = VX \ VY | or VX VY | Set VX to VX OR VY |
| A2XY | BitOp | VX = VX ^ VY | xor VX VY | Set VX to VX XOR VY |
| A3XY | BitOp | VX = ~VX | not VX | Set VX to NOT VX |


*/

use std::{fs, str::SplitWhitespace, collections::HashMap, iter::Peekable};

enum Label {
    Label(String),
    #[allow(non_camel_case_types)]
    u8(u8),
}

enum Instructions {
    Jump(Label),
    IfEq(Label, Label),
    IfNeq(Label, Label),
    IfLe(Label, Label),
    SetRr(Label, Label),
    SetRpc(Label),
    SetRm(Label, Label),
    SetRc(Label, Label),
    SetPcr(Label),
    SetMr(Label, Label),
    Add(Label, Label),
    Sub(Label, Label),
    And(Label, Label),
    Or(Label, Label),
    Xor(Label, Label),
    Not(Label),
}
pub fn run(path: &str) {
    let t = fs::read_to_string(path)
        .expect("Bad path");

    let mut input = t
        .split_whitespace()
        .peekable();

    let mut labels = HashMap::new();

    let mut code = Vec::new();

    let mut comment = false;

    let mut pc = 0;

    // lexer
    loop {
        let token = if let Some(s) = input.next() { s }
        else { break };
            
        if token.starts_with("/*") {
            comment = true;
            continue;
        }
        if token.ends_with("*/") {
            comment = false;
            continue;
        }
        if comment {
            continue;
        }

        if token.starts_with("@") {
            if input.peek() == Some(&"=") {
                input.next();
                let value = get_label(&mut input);
                labels.insert(token.replace("@", ""), value);
                continue;
            }
            else {
                labels.insert(token.replace("@", ""), Label::u8(pc));
            }

            continue;
        }

        match token {
            "jump" => {
                code.push(Instructions::Jump(
                    get_label(&mut input)
                ));
            },
            "ifeq" => {
                code.push(Instructions::IfEq(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "ifneq" => {
                code.push(Instructions::IfNeq(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "ifle" => {
                code.push(Instructions::IfLe(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "setrr" => {
                code.push(Instructions::SetRr(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "setrpc" => {
                code.push(Instructions::SetRpc(
                    get_label(&mut input)
                ));
            },
            "setrm" => {
                code.push(Instructions::SetRm(
                    get_label(&mut input),
                    get_label(&mut input)
                ));
            },
            "setrc" => {
                code.push(Instructions::SetRc(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "setpcr" => {
                code.push(Instructions::SetPcr(
                    get_label(&mut input)
                ));
            },
            "setmr" => {
                code.push(Instructions::SetMr(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "add" => {
                code.push(Instructions::Add(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "sub" => {
                code.push(Instructions::Sub(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "and" => {
                code.push(Instructions::And(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "or" => {
                code.push(Instructions::Or(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "xor" => {
                code.push(Instructions::Xor(
                    get_label(&mut input), 
                    get_label(&mut input)
                ));
            },
            "not" => {
                code.push(Instructions::Not(
                    get_label(&mut input)
                ));
            },
            _ => {
                continue;
            }
        }

        pc += 1;
    }

    let mut binary = Vec::new();

    // tokens to binary
    for token in code {
        match token {
            Instructions::Jump(label) => {
                binary.push(0x00);
                binary.push(label_to_u8(label, &labels));
            },
            Instructions::IfEq(label1, label2) => {
                binary.push(0x10);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::IfNeq(label1, label2) => {
                binary.push(0x20);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::IfLe(label1, label2) => {
                binary.push(0x30);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::SetRr(label1, label2) => {
                binary.push(0x40);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::SetRpc(label) => {
                binary.push(0x41);
                binary.push(label_to_u8(label, &labels));
            },
            Instructions::SetRm(label1, label2) => {
                binary.push(0x50 | label_to_u8(label1, &labels));
                binary.push(label_to_u8(label2, &labels));
            },
            Instructions::SetRc(label1, label2) => {
                binary.push(0x60 | label_to_u8(label1, &labels));
                binary.push(label_to_u8(label2, &labels));
            },
            Instructions::SetPcr(label) => {
                binary.push(0x70);
                binary.push(label_to_u8(label, &labels));
            },
            Instructions::SetMr(label1, label2) => {
                binary.push(0x80 | label_to_u8(label1, &labels));
                binary.push(label_to_u8(label2, &labels));
            },
            Instructions::Add(label1, label2) => {
                binary.push(0x90);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::Sub(label1, label2) => {
                binary.push(0x91);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::And(label1, label2) => {
                binary.push(0xA0);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::Or(label1, label2) => {
                binary.push(0xA1);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::Xor(label1, label2) => {
                binary.push(0xA2);
                binary.push(u4u4_to_u8(
                    label_to_u8(label1, &labels), 
                    label_to_u8(label2, &labels)
                ));
            },
            Instructions::Not(label) => {
                binary.push(0xA3);
                binary.push(label_to_u8(label, &labels));
            },
        }
    }

    fs::write(path.replace(".asm", ".bin"), binary).unwrap();
}

fn get_label(code: &mut Peekable<SplitWhitespace>) -> Label {
    let s = code.next()
        .expect("Expected value");
    if s.starts_with("0x") {
        Label::u8(u8::from_str_radix(
            s
            .replace("0x", "")
            .as_str(), 
            16
        ).expect("Not hex"))
    }
    else if s.starts_with("0b") {
        Label::u8(u8::from_str_radix(
            s
            .replace("0b", "")
            .as_str(), 
            2
        ).expect("Not binary"))
    }
    else if s.starts_with("@") {
        Label::Label(s.replace("@", ""))
    }
    else {
        Label::u8(u8::from_str_radix(
            s, 
            10
        ).expect("Not decimal"))
    }
}

fn label_to_u8(label: Label, labels: &HashMap<String, Label>) -> u8 {
    match label {
        Label::Label(s) => {
            match labels.get(&s) {
                Some(Label::u8(u)) => {
                    *u
                },
                _ => {
                    panic!("Label not found");
                }
            }
        },
        Label::u8(u) => {
            u
        }
    }
}

fn u4u4_to_u8(u41: u8, u42: u8) -> u8 {
    u41 << 4 | u42
}