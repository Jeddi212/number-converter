mod util;

use crate::util::user_input;

fn main() {
    let mut input: String = String::new();
    user_input::read(&mut input);
    println!("Hello, world! {}", &input);
}
