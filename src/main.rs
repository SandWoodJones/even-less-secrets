use std::io::{self, Read, Write};

use crate::termio::{disable_raw_mode, enable_raw_mode};

mod char_attr;
mod charset;
mod effect;
mod termio;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    enable_raw_mode()?;
    let result = effect::els_effect(input.trim_end());
    disable_raw_mode()?;

    result?;
    writeln!(io::stdout(), "")
}
