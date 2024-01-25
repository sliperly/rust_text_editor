use std::{
    fmt,
    io::{self, Stdout, Write},
};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{RawTerminal, IntroRawMode};
};

const EXIT_CHARACTER: char = 'q';
const PADDING_BUTTOM: u16 = 2;
const INFO_MESSAGE: &str = "CTRL-Q = exit";
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(53, 63, 63);

struct ScreenSize {
    width: u16,
    height: u16,
}

#[derive(Default)]
struct Position {
    x: u16,
    y: u16,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}|{})", self.x, self.y)
    }
}
pub struct Editor {
    exit: bool,
    stdout: RawTerminal<Stdout>,
    coursor_position: Position,
}

impl Editor {
    pub fn new() -> Result<Self, io::Error> {
        let (width: u16, height: u16) = termion::terminal_size()?;

        Ok(Editor {
            exit: false,
            stdout: io::stdout().intro_raw_mode()?,
            screen_size: ScreenSize { width, height: height.saturating_sub(PADDING_BUTTOM) },
            cursor_position: Position::default(),
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        while !self.exit {
            self.render()?;
            self.process_key()?;
        }

        Ok(())
    }

    fn render(&mut self) -> Result<(), io::Error> {
        print!("{}", termion::cursor::Goto::default());

        self.render_rows();
        self.render_status_bar();

        print!("{}", termion::cursor::Goto(
            self.cursor_position.x.saturating_add(1),
            self.cursor_position.y.saturating_add(1),
        ));

        self.stdout.flush()
    }

    fn render_rows(&self) {
        for row_num: u16 in 0..self.screen_size.height {
            print!("{}", termion::clear::CurrentLine);
            if row_num == self.screen_size.height / 2 {
                let message: &str = "Hello from rust-text-editor";
                let padding: String = " ".repeat(
                    (self.screen_size.width / 2 + 1) as usize - message.len() / 2
                );

                println!("~{}{}\r", paddiing, message);
            } else {
                println!("~\r");
            }
        }
    }

    fn render_status_bar(&self) {
        for row_num: u16 in 0..self.screen_size.height {
            print!("{}", termion::clear::CurrentLine);

        let status_message: String = format!("cursor {}", self.cursor_position);
        let end_spaces: String = " ".repeat(
            self.screen_size.width.saturating_sub(status_message.len() as u16) as usize
        );
        let status: String = format!("{}{}", status_message, end_spaces);

        print!("{}{}", color::Bg(STATUS_BG_COLOR), color::Fg(STATUS_FG_COLOR));
        println!("{}\r", status);
        print("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));

        print!("{}", termion::clear::CurrentLine);
        print!("{}\r", String::from(INFO_MESSAGE));
    }

    fn process_key(&mut self) -> Result<(), io::Error> {
        match self.next_key()? {
            Key::Ctrl(EXIT_CHARACTER) => { self.exit = true; },
            Key::Char(c) => {println!("your input: {}\r", c); },
            Key::Up => {
                self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
            },
            Key::Down => {
                if self.cursor_position.y < self.screen_size.height - 1 {
                    self.cursor_position.y = self.cursor_position.y.saturating_add(1);
                }
            },
            Key::Left => {
                self.cursor_position.x = self.cursor_position.x.saturating_sub(1);
            },
            Key::Right => {
                if self.cursor_position.x < self.screen_size.width - 1 {
                    self.cursor_position.x = self.cursor_position.x.saturating_add(1);
                }
            },
            _ => ()
        }

        Ok(())
    }

    fn next_key(&self) -> Result<Key, io::Error> {
        match io::stdin().keys().next() {
            Some(key) => key,
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "invalid input"
            ))
        }

    }
}