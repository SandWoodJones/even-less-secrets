use std::time::Duration;

use unicode_width::UnicodeWidthChar;

use crate::charset::get_random_char;

const MAX_DURATION: Duration = Duration::from_millis(2000); // 5000

#[derive(Debug)]
pub struct CharAttr {
    pub source: char,
    pub mask: char,
    pub width: Option<u16>,
    pub time: Duration,
}

impl CharAttr {
    pub fn new(source: char) -> Self {
        let width = source.width_cjk().map(|w| w as u16);

        CharAttr {
            source,
            mask: get_random_char(),
            width,
            time: Duration::from_millis(fastrand::u64(0..MAX_DURATION.as_millis() as u64)),
        }
    }
}
