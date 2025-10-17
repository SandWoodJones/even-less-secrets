use std::time::Duration;

mod effect;
mod termio;
mod charset;

#[derive(Debug)]
struct CharAttr {
    source: char,
    mask: char,
    width: Option<u16>,
    time: Duration,
}

fn main() {
    effect::els_effect("Hello World!");
}
