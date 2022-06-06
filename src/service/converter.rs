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
    let mut temp: String = String::new();

    temp
}

pub fn dec_to_hex(source: i32) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn bin_to_dec(source: String) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn bin_to_oct(source: String) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn bin_to_hex(source: String) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn oct_to_dec(source: i32) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn oct_to_bin(source: i32) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn oct_to_hex(source: i32) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn hex_to_dec(source: String) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn hex_to_bin(source: String) -> String {
    let mut temp: String = String::new();

    temp
}

pub fn hex_to_oct(source: String) -> String {
    let mut temp: String = String::new();

    temp
}