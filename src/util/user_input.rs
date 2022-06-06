use std::io::{stdin, stdout, Write};

pub fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

pub fn read_new() -> String {
    let mut input = String::new();

    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(&mut input).expect("Failed to read input");
    
    input
}

pub fn str_to_i32(str: String) -> i32 {
    str.trim().parse::<i32>().expect("Please input a valid number")
}