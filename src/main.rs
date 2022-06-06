mod util;

use crate::util::welcome;
use crate::util::user_input;

fn main() {
    let mut user_choose: String = String::new();

    welcome::greet(&mut user_choose);
    let num = user_input::str_to_i32(&mut user_choose);
    
    print!("your number is : {}", num);
}
