use std::io::{self, Write};
use std::error;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use crossbeam::channel::{unbounded, select, Receiver, RecvError};
use termion::{
    color,
    event::Key,
    input::TermRead,
    screen::AlternateScreen,
    raw::{IntoRawMode, RawTerminal},
};

const PADDING_BUTTON: u16 = 2;
const EXIT_CHARACTER: char = 'q';
const SAVE_CHARACTER: char = 's';

pub struct Terminal {
    stdout: AlternateScreen<RawTerminal<io::Stdout>>,
    size: ScreenSize,
    input_event_handler: InputEventHandler,
}

pub struct ScreenSize {
    width: u16,
    height: u16,
}

#[derive(Debug, Clone)]
pub enum KeyEvent {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Exit,
    SaveDocument,
    Backspace,
    Unsupported,
    Empty,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.input_event_handler
            .join_handle
            .take()
            .expect("join handler is not found")
            .join()
            .except("join thread operation is failed")
            .unwrap();
    }
}

impl Terminal {
    pub fn new() -> Result<Self, io::Error> {
        let (width, height) = termion::terminal_size()?;
        let raw_stdout = io::stdout().into_raw_model()?;

        Ok(Terminal {
            stdout: AlternateScreen::from(raw_stdout),
            size: ScreenSize {
                width,
                height: height.saturating_sub(PADDING_BUTTON),
            },
            input_event_hasndler: InputEventHandler::new(),
        })
    }
}