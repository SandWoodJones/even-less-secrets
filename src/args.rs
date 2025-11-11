use clap::ArgAction;

use crate::color::Color;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 'a',
        long = "auto-decrypt",
        overrides_with = "auto_decrypt",
        help = "Start decrypting sequence immediatly after jumbling input"
    )]
    _no_auto_decrypt: bool,
    #[arg(short='A', long="no-auto-decrypt", action = ArgAction::SetFalse, help = "Wait for user input before starting the decrypting sequence")]
    pub auto_decrypt: bool,

    #[arg(
        short = 's',
        long,
        overrides_with = "_no_blank_masks",
        help = "Blank spaces will be encrypted and decrypted"
    )]
    pub blank_masks: bool,
    #[arg(short='S', long, action = ArgAction::SetFalse, help = "Blank spaces are ignored")]
    _no_blank_masks: bool,

    #[arg(
        short = 'c',
        long,
        overrides_with = "_no_clear_screen",
        help = "Clear the screen prior to printing any output"
    )]
    pub clear_screen: bool,
    #[arg(short='C', long, action = ArgAction::SetFalse)]
    _no_clear_screen: bool,

    #[arg(
        short = 'f',
        long,
        value_name = "COLOR",
        default_value = "white",
        help = "The foreground color of the decrypted text. Accepts any of the 16 standard ANSI colors or, if supported by the terminal, a hexadecimal RGB color code"
    )]
    pub foreground_color: Color,
}
