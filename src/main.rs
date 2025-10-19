use std::io::{self, Read};

mod char_attr;
mod charset;
mod effect;
mod termio;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    effect::els_effect(input.trim_end())?;
    println!();
    Ok(())
}
