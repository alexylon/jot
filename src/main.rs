mod jot;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let words = &args;

    jot::write_data(words);
}
