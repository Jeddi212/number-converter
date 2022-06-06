mod controller;
mod service;
mod util;

use controller::switcher::proceed;
use util::welcome;

fn main() {
    let mut user_choose: String = String::new();

    welcome::greet(&mut user_choose);
    proceed(&user_choose);
}
