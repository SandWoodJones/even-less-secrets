use std::time::Duration;

use unicode_width::UnicodeWidthChar;

use crate::charset::get_random_char;

#[derive(Debug)]
pub struct CharAttr {
    pub source: char,
    pub mask: char,
    pub width: Option<u16>,
    pub time: Duration,
}

impl CharAttr {
    pub fn new(source: char, max_duration: u64) -> Self {
        let width = source.width_cjk().map(|w| w as u16);

        CharAttr {
            source,
            mask: get_random_char(),
            width,
            time: Duration::from_millis(fastrand::u64(0..max_duration)),
        }
    }
}
