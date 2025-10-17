use std::thread;
use std::time::Duration;

use unicode_width::UnicodeWidthChar;

use crate::CharAttr;
use crate::charset::get_random_char;
use crate::termio::{cursor_pos, flush_output, move_cursor};

const MAX_ROWS: u16 = 24;
const MAX_COLS: u16 = 80;

const EFFECT_SPEED: Duration = Duration::from_millis(40);
const JUMBLE_SECONDS: Duration = Duration::from_secs(2);
const JUMBLE_LOOP_SPEED: Duration = Duration::from_millis(35);
const REVEAL_LOOP_SPEED: Duration = Duration::from_millis(50);

fn print_mask(list: &Vec<CharAttr>) {
    for ch in list {
        if ch.source.is_whitespace() {
            print!("{}", ch.source);
            continue;
        }

        print!("{}", ch.mask);
        if ch.width == Some(2) {
            print!("{}", get_random_char())
        }

        flush_output().unwrap();

        thread::sleep(EFFECT_SPEED);
    }
}

fn jumble(list: &Vec<CharAttr>, cursor_orig_pos: (u16, u16)) {
    for _ in 0..(JUMBLE_SECONDS.as_millis() / JUMBLE_LOOP_SPEED.as_millis()) {
        move_cursor(cursor_orig_pos).unwrap();

        for ch in list {
            if ch.source.is_whitespace() {
                print!("{}", ch.source);
                continue;
            }

            print!("{}", get_random_char());
            if ch.width == Some(2) {
                print!("{}", get_random_char())
            }
        }

        flush_output().unwrap();

        thread::sleep(JUMBLE_LOOP_SPEED);
    }
}

fn reveal(list: &mut Vec<CharAttr>, cursor_orig_pos: (u16, u16)) {
    let mut reveal_complete = false;
    while !reveal_complete {
        move_cursor(cursor_orig_pos).unwrap();

        reveal_complete = true;
        for ch in list.iter_mut() {
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
                reveal_complete = false;
            } else {
                print!("{}", ch.source);
            }
        }

        flush_output().unwrap();
        thread::sleep(REVEAL_LOOP_SPEED);
    }
}

pub fn els_effect(input: &str) {
    let mut char_list: Vec<CharAttr> = vec![];

    let mut orig_cursor_pos = cursor_pos().unwrap();
    let mut cur_row = orig_cursor_pos.0;
    let mut cur_col = orig_cursor_pos.1;

    // process input
    for ch in input.chars() {
        // don't go beyond max rows
        if cur_row - orig_cursor_pos.0 >= MAX_ROWS - 1 {
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
            if cur_row == MAX_ROWS + 1 && orig_cursor_pos.0 > 0 {
                orig_cursor_pos.0 -= 1;
                cur_row -= 1;
            }
        }
    }

    print_mask(&char_list);
    jumble(&char_list, orig_cursor_pos);
    reveal(&mut char_list, orig_cursor_pos);
}
