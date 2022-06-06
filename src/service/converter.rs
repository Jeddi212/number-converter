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