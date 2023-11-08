use std::env::args;

fn main() {
    assembler::run(&args().nth(1).expect("No file path provided"));
}