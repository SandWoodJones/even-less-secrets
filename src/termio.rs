use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{ExecutableCommand, event};
use crossterm::{cursor, terminal};

pub fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    io::stdout().execute(cursor::Hide)?;
    spawn_event_listener();
    Ok(())
}

pub fn disable_raw_mode() -> io::Result<()> {
    io::stdout().execute(cursor::Show)?;
    terminal::disable_raw_mode()
}

fn spawn_event_listener() {
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(Event::Key(ev)) = event::read() {
                    check_interrupt(&ev);
                }
            }
        }
    });
}

pub fn wait_for_input() -> io::Result<()> {
    loop {
        match event::read()? {
            Event::Key(ev) => {
                check_interrupt(&ev);
                break;
            }
            _ => continue,
        }
    }

    Ok(())
}

fn check_interrupt(event: &KeyEvent) {
    match event.code {
        KeyCode::Char('c') | KeyCode::Char('d') | KeyCode::Char('z')
            if event.modifiers == KeyModifiers::CONTROL =>
        {
            disable_raw_mode().ok();
            std::process::exit(130);
        }
        _ => {}
    }
}

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
