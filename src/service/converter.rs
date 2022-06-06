use crate::util::user_input::{str_to_i32, chr_to_i32};

pub fn dec_to_bin(source: i32) -> String {
    let mut n = source;
    let mut temp: String = String::new();
    
    while n > 0 {
        if n % 2 != 0 {
            temp.push('1');
        } else {
            temp.push('0');
        }

        n /= 2;
    }

    temp.chars().rev().collect::<String>()
}

pub fn dec_to_oct(source: i32) -> String {
    let mut n = source;
    let mut m: i32;
    let mut vec_temp: Vec<i32> = Vec::new();
    while n > 0 {
        m = n % 8;
        n /= 8;
        vec_temp.push(m);
    }

    // iterate over the vector
    // cast it from i32 to String
    // reverse the order
    vec_temp.into_iter().map(|x| x.to_string()).rev().collect()
}

pub fn dec_to_hex(source: i32) -> String {
    let mut n = source;
    let mut temp: String = String::new();
    
    if source == 0 {
        temp.push('0');
    }

    while n > 0 {
        temp.push(get_hex(n % 16));
        n /= 16;
    }

    temp.chars().rev().collect::<String>()
}

pub fn bin_to_dec(source: String) -> String {
    let mut n: String = source.clone();
    let mut i: u32 = 0;
    let mut temp: i32 = 0;
    
    while n.len() > 0 {
        let current = n.pop().expect("String can't pop(ed)");
        if current == '1' {
            temp += i32::pow(2, i);
        }
        i += 1;
    }
    
    temp.to_string()
}

pub fn bin_to_oct(source: String) -> String {
    dec_to_oct(str_to_i32(bin_to_dec(source)))
}

pub fn bin_to_hex(source: String) -> String {
    dec_to_hex(str_to_i32(bin_to_dec(source)))
}

pub fn oct_to_dec(source: String) -> String {
    let mut n: String = source.clone();
    let mut i: u32 = 0;
    let mut temp: i32 = 0;
    
    while n.len() > 0 {
        let current = n.pop().expect("String can't pop(ed)");
        temp += chr_to_i32(current) * i32::pow(8, i);
        i += 1;
    }
    
    temp.to_string()
}

pub fn oct_to_bin(source: String) -> String {
    dec_to_bin(str_to_i32(oct_to_dec(source)))
}

pub fn oct_to_hex(source: String) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn hex_to_dec(source: String) -> String {
    let mut n: String = source.clone();
    let mut i: u32 = 0;
    let mut temp: i32 = 0;
    
    while n.len() > 0 {
        let current = n.pop().expect("String can't pop(ed)");
        temp += hex_get_dec(current) * i32::pow(16, i);
        i += 1;
    }
    
    temp.to_string()
}

pub fn hex_to_bin(source: String) -> String {
    dec_to_bin(str_to_i32(hex_to_dec(source)))
}

pub fn hex_to_oct(source: String) -> String {
    dec_to_oct(str_to_i32(hex_to_dec(source)))
}

fn hex_get_dec(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        'C' | 'c' => 12,
        'D' | 'd' => 13,
        'E' | 'e' => 14,
        'F' | 'f' => 15,
        _ => 0
    }
}

fn get_hex(number: i32) -> char {
    match number {
        0  => '0',
        1  => '1',
        2  => '2',
        3  => '3',
        4  => '4',
        5  => '5',
        6  => '6',
        7  => '7',
        8  => '8',
        9  => '9',
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => 'x'
    }
}