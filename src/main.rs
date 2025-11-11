// https://jwodder.github.io/kbits/posts/clap-bool-negate/

use std::io::{self, Read, Write};

use clap::{ArgAction, Parser};

use crate::termio::{disable_raw_mode, enable_raw_mode};

mod char_attr;
mod charset;
mod effect;
mod termio;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'a',
        long = "auto-decrypt",
        overrides_with = "auto_decrypt",
        help = "Start decrypting sequence immediatly after jumbling input"
    )]
    _no_auto_decrypt: bool,
    #[arg(short='A', long="no-auto-decrypt", action = ArgAction::SetFalse, help = "Wait for user input before starting the decrypting sequence")]
    auto_decrypt: bool,

    #[arg(
        short = 's',
        long,
        overrides_with = "_no_blank_masks",
        help = "Blank spaces will be encrypted and decrypted"
    )]
    blank_masks: bool,
    #[arg(short='S', long="no-blank-masks", action = ArgAction::SetFalse, help = "Blank spaces are ignored")]
    _no_blank_masks: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    enable_raw_mode()?;
    let result = effect::els_effect(input.trim_end(), args);
    disable_raw_mode()?;

    result?;
    writeln!(io::stdout(), "")
}
