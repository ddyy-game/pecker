use std::io::{stdout, Result, Stdout, Write};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{size, Clear, ClearType},
};

pub struct MainScreen {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

impl MainScreen {
    pub fn new() -> Self {
        let (width, height) = size().expect("cannot determine terminal size");
        MainScreen {
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

    pub fn set_color(&mut self, color: Color) -> Result<()> {
        queue!(self.stdout, SetForegroundColor(color))
    }

    pub fn put_str(&mut self, s: &str) -> Result<()> {
        queue!(self.stdout, Print(s))
    }

    pub fn set_char(&mut self, c: char) -> Result<()> {
        queue!(self.stdout, Print(c), cursor::MoveLeft(1))
    }

    pub fn put_str_centered(&mut self, s: &str, row: i16) -> Result<()> {
        let len: u16 = s.len().try_into().expect("string length too long");
        let c = (self.width - len) / 2;
        let r = self.wrap_row(row);
        queue!(self.stdout, cursor::MoveTo(c, r), Print(s))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }

    fn wrap_row(&self, row: i16) -> u16 {
        if row > 0 && row as u16 >= self.height {
            self.height
        } else if row >= 0 {
            row as u16
        } else if self.height >= (-row as u16) {
            self.height - (-row as u16)
        } else {
            0u16
        }
    }
}

impl Default for MainScreen {
    fn default() -> Self {
        Self::new()
    }
}
