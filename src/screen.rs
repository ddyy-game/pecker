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
            height,
            width,
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn clear(&mut self) -> Result<()> {
        execute!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn put_str_centered(&mut self, s: &str, row: i8) -> Result<()> {
        let len: u16 = s.len().try_into().expect("string length too long");
        let c = (self.width - len) / 2;
        let r = if row >= 0 {
            row as u16
        } else {
            self.height - (-row as u16)
        };
        queue!(self.stdout, cursor::MoveTo(c, r), Print(s))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}
