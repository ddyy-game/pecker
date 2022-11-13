use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor, execute, queue,
    style::Print,
    terminal::{size, Clear, ClearType},
    Result,
};

pub struct MainScreen {
    stdout: Stdout,
    width: u16,
    height: u16,
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

    pub fn reset(&mut self) -> Result<()> {
        self.clear()?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
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
        if row >= 0 {
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
