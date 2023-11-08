use std::env::args;

fn main() {
    emulator::run(&args().nth(1).expect("No file path provided"));
}