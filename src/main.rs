use std::io::{self, Read};
use std::time::Duration;

mod charset;
mod effect;
mod termio;

#[derive(Debug)]
struct CharAttr {
    source: char,
    mask: char,
    width: Option<u16>,
    time: Duration,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    effect::els_effect(&input);
}
