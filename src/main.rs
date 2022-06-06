mod controller;
mod service;
mod util;

use controller::switcher::proceed;

use crate::util::welcome;
use crate::util::user_input;

fn main() {
    let mut user_choose: String = String::new();

    welcome::greet(&mut user_choose);
    proceed(&user_choose);
}
