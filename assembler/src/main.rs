use std::{fs, env::args, str::SplitWhitespace};

fn main() {
    let path = args().nth(1).expect("No file path provided");

    let mut code = fs::read_to_string(&path)
        .expect("Bad path")
        .split_whitespace();

    let mut binary = Vec::new();


    for token in code {
        println!("{}", token);
        let mut string = String::new();
        let mut chars = token.chars();
        string.push(chars.next().expect("Unrecognized token"));

        match string.as_str() {
            "//" => {
                continue;
            },
            "jump" => {
                binary.push(0x00);
                binary.push(get_value(&mut code));
            },
            _ => {
                panic!("Unrecognized token");
            }
        }
    }

    fs::write(path.replace(".txt", ".bin"), binary);
}

fn get_value(code: &mut SplitWhitespace) -> u8 {
    let s = code.next()
        .expect("Unrecognized token");
    if s.starts_with("0x") {
        u8::from_str_radix(
            s
            .replace("0x", "")
            .as_str(), 
            16
        ).expect("Not hex")
    }
    else if s.ends_with("b") {
        u8::from_str_radix(
            s
            .replace("b", "")
            .as_str(), 
            2
        ).expect("Not binary")
    }
    else {
        u8::from_str_radix(
            s, 
            10
        ).expect("Not decimal")
    }
}