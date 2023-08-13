use std::{
    io::{stdout, Result, Stdout, Write},
};

use crossterm::{
    cursor, execute, queue,
    style::{PrintStyledContent, StyledContent, Stylize},
    terminal::{size, Clear, ClearType},
};

pub struct MainScreen {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

pub trait Styled: Sized {
    fn format(self) -> String;
    fn hit(self) -> StyledContent<String> {
        self.format().green()
    }
    fn miss(self) -> StyledContent<String> {
        self.format().red().underlined()
    }
    fn blank(self) -> StyledContent<String> {
        self.format().dark_grey()
    }
    fn default(self) -> StyledContent<String> {
        self.format().reset()
    }
}

impl Styled for &str {
    fn format(self) -> String {
        self.replace('\n', "⏎")
    }
}

impl Styled for String {
    fn format(self) -> String {
        self.replace('\n', "⏎")
    }
}

impl Styled for char {
    fn format(self) -> String {
        if self == '\n' {
            "⏎".to_string()
        } else {
            self.to_string()
        }
    }
}

impl MainScreen {
    #[must_use]
    pub fn new() -> Self {
        let (width, height) = size().expect("cannot determine terminal size");
        Self {
            stdout: stdout(),
            width,
            height,
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn move_to(&mut self, column: u16, row: u16) -> Result<()> {
        queue!(self.stdout, cursor::MoveTo(column, row))
    }

    pub fn put_str(&mut self, s: StyledContent<String>) -> Result<()> {
        queue!(self.stdout, PrintStyledContent(s))
    }

    pub fn set_char(&mut self, c: StyledContent<String>) -> Result<()> {
        queue!(self.stdout, PrintStyledContent(c), cursor::MoveLeft(1))
    }

    pub fn debug(&mut self, s: &str) -> Result<()> {
        queue!(
            self.stdout,
            cursor::SavePosition,
            cursor::MoveTo(0, self.height),
            PrintStyledContent(" ".repeat(self.width as usize).default()),
            cursor::MoveTo(0, self.height),
            PrintStyledContent(s.default()),
            cursor::RestorePosition,
        )?;
        self.flush()
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}

impl Default for MainScreen {
    fn default() -> Self {
        Self::new()
    }
}
