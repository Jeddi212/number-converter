use std::io::{stdin, stdout, Write};

pub fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

pub fn str_to_u8(str: &mut String) -> u8 {
    str.trim().parse::<u8>().expect("Please input a valid number")
}

pub fn str_to_u32(str: &mut String) -> u32 {
    str.trim().parse::<u32>().expect("Please input a valid number")
}

pub fn str_to_i32(str: &mut String) -> i32 {
    str.trim().parse::<i32>().expect("Please input a valid number")
}