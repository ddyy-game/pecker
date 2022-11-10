use std::io::{stdout, Stdout, Write};

use crossterm::{cursor, queue, style::Print, Result};

pub struct MainScreen {
    stdout: Stdout,
}

impl MainScreen {
    pub fn new() -> Self {
        MainScreen { stdout: stdout() }
    }

    pub fn put_str(&mut self, s: &str, row: u16) -> Result<()> {
        queue!(
            self.stdout,
            cursor::MoveTo(
                (s.len() / 2).try_into().expect("string length too long"),
                row,
            ),
            Print(s)
        )?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}
