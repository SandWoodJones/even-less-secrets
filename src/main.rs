use std::io::{self, Read};

mod char_attr;
mod charset;
mod effect;
mod termio;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    effect::els_effect(input.trim_end());
    println!()
}
