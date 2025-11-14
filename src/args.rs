// https://jwodder.github.io/kbits/posts/clap-bool-negate/
use clap::ArgAction;

use crate::color::Color;

const DEFAULT_AUTODECRYPT_DELAY: &str = "1000";
const DEFAULT_TYPE_EFFECT_SPEED: u64 = 4;
const DEFAULT_JUMBLE_DURATION: u64 = 2000;
const DEFAULT_JUMBLE_SPEED: u64 = 35;
const DEFAULT_REVEAL_DURATION: u64 = 5000;
const DEFAULT_REVEAL_SPEED: u64 = 50;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 'a',
        long,
        value_name = "MILLIS",
        num_args = 0..=1,
        default_missing_value = DEFAULT_AUTODECRYPT_DELAY,
        help = format!("Start decrypting sequence after a delay [default: {}].\nIf not specified, wait for user input", DEFAULT_AUTODECRYPT_DELAY)
    )]
    pub auto_decrypt: Option<u64>,

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
        help = "The foreground color of the decrypted text.\nAccepts any of the 16 standard ANSI colors or, if supported by the terminal, a hexadecimal RGB color code"
    )]
    pub foreground_color: Color,

    #[arg(
        long,
        value_name = "MILLIS",
        default_value_t = DEFAULT_TYPE_EFFECT_SPEED,
        help = "The delay between typing each character's mask",
        help_heading = "Settings"
    )]
    pub type_speed: u64,

    #[arg(
        long,
        value_name = "MILLIS",
        default_value_t = DEFAULT_JUMBLE_DURATION,
        help = "The duration of the jumble effect",
        help_heading = "Settings"
    )]
    pub jumble_duration: u64,

    #[arg(
        long,
        value_name = "MILLIS",
        default_value_t = DEFAULT_JUMBLE_SPEED,
        help = "The delay between passes of the jumble effect",
        help_heading = "Settings"
    )]
    pub jumble_speed: u64,

    #[arg(
        long,
        value_name = "MILLIS",
        default_value_t = DEFAULT_REVEAL_DURATION,
        help = "The maximum duration for a character to get revealed",
        help_heading = "Settings"
    )]
    pub reveal_duration: u64,

    #[arg(
        long,
        value_name = "MILLIS",
        default_value_t = DEFAULT_REVEAL_SPEED,
        help = "The delay between passes of the reveal effect",
        help_heading = "Settings"
    )]
    pub reveal_speed: u64,
}
