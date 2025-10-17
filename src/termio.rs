use std::io::{self, Write};

use crossterm::{ExecutableCommand, cursor};

pub fn flush_output() -> io::Result<()> {
    if let Err(e) = io::stdout().flush() {
        eprintln!("flush failed: {e:?}");
        return Err(e);
    }

    Ok(())
}

pub fn move_cursor(pos: (u16, u16)) -> io::Result<()> {
    if let Err(e) = io::stdout().execute(cursor::MoveTo(pos.0, pos.1)) {
        eprintln!("cursor move failed: {e:?}");
        return Err(e);
    };

    Ok(())
}

pub fn cursor_pos() -> io::Result<(u16, u16)> {
    match cursor::position() {
        Ok(pos) => Ok(pos),
        Err(e) => {
            eprintln!("cursor position failed: {e:?}");
            return Err(e);
        }
    }
}
