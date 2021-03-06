use crate::service::converter;
use crate::util::user_input::*;

pub fn proceed(user_choose: &String) {
    print!("\nNumber ::: ");
    // trimmed user input, at a time convert it to str
    let output = match user_choose.trim() {
        "1"  => converter::dec_to_bin(str_to_i32(read_new())),
        "2"  => converter::dec_to_oct(str_to_i32(read_new())),
        "3"  => converter::dec_to_hex(str_to_i32(read_new())),
        "4"  => converter::bin_to_dec(read_new()),
        "5"  => converter::bin_to_oct(read_new()),
        "6"  => converter::bin_to_hex(read_new()),
        "7"  => converter::oct_to_dec(read_new()),
        "8"  => converter::oct_to_bin(read_new()),
        "9"  => converter::oct_to_hex(read_new()),
        "10" => converter::hex_to_dec(read_new()),
        "11" => converter::hex_to_bin(read_new()),
        "12" => converter::hex_to_oct(read_new()),
        _ => panic!("Please input a number between 1 ~ 12"),
    };

    println!("Result ::: {}", output);
}