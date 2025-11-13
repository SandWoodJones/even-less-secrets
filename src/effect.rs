use std::time::Duration;
use std::{io, thread};

use crossterm::terminal;

use crate::char_attr::CharAttr;
use crate::charset::get_random_char;
use crate::termio::{
    clear_screen, cursor_pos, flush_output, move_cursor, reset_colors, set_foreground_color,
    wait_for_input,
};

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

            let attr = CharAttr::new(ch, args.reveal_duration);
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
        if self.args.decrypt_delay < 0 {
            wait_for_input()?;
        } else {
            thread::sleep(Duration::from_millis(self.args.decrypt_delay as u64));
        }

        self.jumble()?;
        self.reveal()
    }

    fn print_mask(&self) -> io::Result<()> {
        for ch in self.char_list.iter() {
            if Self::handle_whitespace(ch, self.args.blank_masks) {
                continue;
            }

            print!("{}", ch.mask);
            if ch.width == Some(2) {
                print!("{}", get_random_char())
            }

            flush_output()?;

            thread::sleep(Duration::from_millis(self.args.type_speed));
        }

        Ok(())
    }

    fn jumble(&self) -> io::Result<()> {
        for _ in 0..(self.args.jumble_duration / self.args.jumble_speed) {
            move_cursor(self.orig_cursor_pos)?;

            for ch in self.char_list.iter() {
                if Self::handle_whitespace(ch, self.args.blank_masks) {
                    continue;
                }

                print!("{}", get_random_char());
                if ch.width == Some(2) {
                    print!("{}", get_random_char())
                }
            }

            flush_output()?;

            thread::sleep(Duration::from_millis(self.args.jumble_speed));
        }

        Ok(())
    }

    fn reveal(&mut self) -> io::Result<()> {
        let speed_duration = Duration::from_millis(self.args.reveal_speed);
        let mut reveal_complete = false;

        while !reveal_complete {
            move_cursor(self.orig_cursor_pos)?;

            reveal_complete = true;
            for ch in self.char_list.iter_mut() {
                if Self::handle_whitespace(ch, self.args.blank_masks) {
                    continue;
                }

                if !ch.time.is_zero() {
                    if ch.time.as_millis() < (self.args.reveal_duration / 10) as u128 {
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

                    ch.time = ch.time.saturating_sub(speed_duration);
                    reveal_complete = false;
                } else {
                    set_foreground_color(&self.args.foreground_color)?;
                    print!("{}", ch.source);
                    reset_colors()?;
                }
            }

            flush_output()?;
            thread::sleep(speed_duration);
        }

        Ok(())
    }

    /// Returns `true` if the character was handled and should be skipped
    fn handle_whitespace(ch: &CharAttr, blank_masks: bool) -> bool {
        if ch.source.is_whitespace() {
            if ch.source == '\n' {
                print!("\r\n");
                return true;
            } else if !blank_masks {
                print!("{}", ch.source);
                return true;
            }
        }

        false
    }
}
