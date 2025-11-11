use std::time::Duration;
use std::{io, thread};

use crossterm::terminal;

use crate::char_attr::CharAttr;
use crate::charset::get_random_char;
use crate::termio::{clear_screen, cursor_pos, flush_output, move_cursor, wait_for_input};

const AUTODECRYPT_INTERVAL: Duration = Duration::from_secs(1);
const EFFECT_SPEED: Duration = Duration::from_millis(40);
const JUMBLE_SECONDS: Duration = Duration::from_secs(2);
const JUMBLE_LOOP_SPEED: Duration = Duration::from_millis(35);
const REVEAL_LOOP_SPEED: Duration = Duration::from_millis(50);

pub struct ElsEffect {
    args: crate::Args,
    char_list: Vec<CharAttr>,
    orig_cursor_pos: (u16, u16),
}

impl ElsEffect {
    pub fn new(input: &str, args: crate::Args) -> io::Result<Self> {
        let mut char_list: Vec<CharAttr> = vec![];
        let terminal_size = terminal::size()?;
        let mut orig_cursor_pos = if args.clear_screen {
            (0, 0)
        } else {
            cursor_pos()?
        };

        let mut cur_col = orig_cursor_pos.0;
        let mut cur_row = orig_cursor_pos.1;

        // process input
        for ch in input.chars() {
            // don't go beyond max rows
            if cur_row - orig_cursor_pos.1 >= terminal_size.1 - 1 {
                break;
            }

            let attr = CharAttr::new(ch);
            if let Some(w) = attr.width {
                cur_col += w;
            }

            char_list.push(attr);

            if ch == '\n' || cur_col > terminal_size.0 {
                cur_col = 0;
                cur_row += 1;
                if cur_row == terminal_size.1 && orig_cursor_pos.1 != 0 {
                    orig_cursor_pos.1 -= 1;
                    cur_row -= 1;
                }
            }
        }

        Ok(Self {
            args,
            char_list,
            orig_cursor_pos,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        if self.args.clear_screen {
            clear_screen()?;
            move_cursor((0, 0))?;
        }

        self.print_mask()?;
        if self.args.auto_decrypt {
            thread::sleep(AUTODECRYPT_INTERVAL)
        } else {
            wait_for_input()?;
        }

        self.jumble()?;
        self.reveal()
    }

    fn print_mask(&self) -> io::Result<()> {
        for ch in self.char_list.iter() {
            if ch.source.is_whitespace() && !self.args.blank_masks {
                if ch.source == '\n' {
                    print!("\r\n");
                } else {
                    print!("{}", ch.source);
                }
                continue;
            }

            print!("{}", ch.mask);
            if ch.width == Some(2) {
                print!("{}", get_random_char())
            }

            flush_output()?;

            thread::sleep(EFFECT_SPEED);
        }

        Ok(())
    }

    fn jumble(&self) -> io::Result<()> {
        for _ in 0..(JUMBLE_SECONDS.as_millis() / JUMBLE_LOOP_SPEED.as_millis()) {
            move_cursor(self.orig_cursor_pos)?;

            for ch in self.char_list.iter() {
                if ch.source.is_whitespace() && !self.args.blank_masks {
                    if ch.source == '\n' {
                        print!("\r\n");
                    } else {
                        print!("{}", ch.source);
                    }
                    continue;
                }

                print!("{}", get_random_char());
                if ch.width == Some(2) {
                    print!("{}", get_random_char())
                }
            }

            flush_output()?;

            thread::sleep(JUMBLE_LOOP_SPEED);
        }

        Ok(())
    }

    fn reveal(&mut self) -> io::Result<()> {
        let mut reveal_complete = false;
        while !reveal_complete {
            move_cursor(self.orig_cursor_pos)?;

            reveal_complete = true;
            for ch in self.char_list.iter_mut() {
                if ch.source.is_whitespace() && !self.args.blank_masks {
                    if ch.source == '\n' {
                        print!("\r\n");
                    } else {
                        print!("{}", ch.source);
                    }
                    continue;
                }

                if !ch.time.is_zero() {
                    if ch.time.as_millis() < 500 {
                        if fastrand::u8(0..3) == 0 {
                            ch.mask = get_random_char();
                        }
                    } else if fastrand::u8(0..10) == 0 {
                        ch.mask = get_random_char();
                    }

                    print!("{}", ch.mask);
                    if ch.width == Some(2) {
                        print!("{}", get_random_char())
                    }

                    ch.time = ch.time.saturating_sub(REVEAL_LOOP_SPEED);
                    reveal_complete = false;
                } else {
                    print!("{}", ch.source);
                }
            }

            flush_output()?;
            thread::sleep(REVEAL_LOOP_SPEED);
        }

        Ok(())
    }
}
