use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crossterm::{ExecutableCommand, cursor};
use unicode_width::UnicodeWidthChar;

#[derive(Debug)]
struct CharAttr {
    source: char,
    mask: char,
    width: Option<u16>,
    time: Duration,
}

fn get_random_char() -> char {
    let char_table = ['!', '\\', '#', '$', '%', '&', '\'', '(', ')', '*'];
    return char_table[fastrand::usize(..char_table.len())];
}

fn els_effect(input: &str) {
    const MAX_ROWS: u16 = 24;
    const MAX_COLS: u16 = 80;
    const EFFECT_SPEED: Duration = Duration::from_millis(40);
    const JUMBLE_SECONDS: Duration = Duration::from_secs(2);
    const JUMBLE_LOOP_SPEED: Duration = Duration::from_millis(35);
    const REVEAL_LOOP_SPEED: Duration = Duration::from_millis(50);

    let mut char_list: Vec<CharAttr> = vec![];

    let (mut orig_row, orig_col) = match cursor::position() {
        Ok(pos) => pos,
        Err(e) => {
            eprintln!("cursor position failed: {e:?}");
            return;
        }
    };
    let mut cur_row = orig_row;
    let mut cur_col = orig_col;

    // process input
    for ch in input.chars() {
        // don't go beyond max rows
        if cur_row - orig_row >= MAX_ROWS - 1 {
            break;
        }

        let width = ch.width_cjk().map(|w| w as u16);

        char_list.push(CharAttr {
            source: ch,
            mask: get_random_char(),
            width,
            time: Duration::from_millis(fastrand::u64(0..5000)),
        });

        if let Some(w) = width {
            cur_col += w;
        }
        if ch == '\n' || cur_col > MAX_COLS {
            cur_col = 0;
            cur_row += 1;
            if cur_row == MAX_ROWS + 1 && orig_row > 0 {
                orig_row -= 1;
                cur_row -= 1;
            }
        }
    }

    // print mask characters
    for ch in &char_list {
        if ch.source.is_whitespace() {
            print!("{}", ch.source);
            continue;
        }

        print!("{}", ch.mask);
        if ch.width == Some(2) {
            print!("{}", get_random_char())
        }

        if let Err(e) = io::stdout().flush() {
            eprintln!("flush failed: {e:?}");
            return;
        }

        thread::sleep(EFFECT_SPEED)
    }

    for _ in 0..(JUMBLE_SECONDS.as_millis() / JUMBLE_LOOP_SPEED.as_millis()) {
        if let Err(e) = io::stdout().execute(cursor::MoveTo(orig_row, orig_col)) {
            eprintln!("cursor move failed: {e:?}");
            return;
        };

        for ch in &char_list {
            if ch.source.is_whitespace() {
                print!("{}", ch.source);
                continue;
            }

            print!("{}", get_random_char());
            if ch.width == Some(2) {
                print!("{}", get_random_char())
            }
        }

        if let Err(e) = io::stdout().flush() {
            eprintln!("flush failed: {e:?}");
            return;
        }

        thread::sleep(JUMBLE_LOOP_SPEED);
    }

    let mut revealed = false;
    while !revealed {
        if let Err(e) = io::stdout().execute(cursor::MoveTo(orig_row, orig_col)) {
            eprintln!("cursor move failed: {e:?}");
            return;
        };

        revealed = true;
        for ch in &mut char_list {
            if ch.source.is_whitespace() {
                print!("{}", ch.source);
                continue;
            }

            if !ch.time.is_zero() {
                if ch.time.as_millis() < 500 {
                    if fastrand::u8(0..3) == 0 {
                        ch.mask = get_random_char();
                    }
                } else {
                    if fastrand::u8(0..10) == 0 {
                        ch.mask = get_random_char();
                    }
                }

                print!("{}", ch.mask);
                ch.time = ch.time.saturating_sub(REVEAL_LOOP_SPEED);
                revealed = false;
            } else {
                print!("{}", ch.source);
            }
        }

        if let Err(e) = io::stdout().flush() {
            eprintln!("flush failed: {e:?}");
            return;
        }
        thread::sleep(REVEAL_LOOP_SPEED);
    }
}

fn main() {
    els_effect("Hello World!");
}
