// https://jwodder.github.io/kbits/posts/clap-bool-negate/

use std::io::{self, Read, Write};

use clap::Parser;

use crate::{
    args::Args,
    effect::ElsEffect,
    termio::{disable_raw_mode, enable_raw_mode},
};

mod args;
mod char_attr;
mod charset;
mod color;
mod effect;
mod termio;

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    enable_raw_mode()?;
    let result = ElsEffect::new(input.trim_end(), args)?.run();
    disable_raw_mode()?;

    result?;
    writeln!(io::stdout())
}
