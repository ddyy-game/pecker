use std::io::{stdout, Result, Stdout, Write};

use crossterm::{
    cursor, execute, queue,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal::{size, Clear, ClearType},
};

pub struct MainScreen {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub enum Style {
    Hit,
    Miss,
    Blank,
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

    pub fn set_style(&mut self, style: Style) -> Result<()> {
        match style {
            Style::Hit => queue!(
                self.stdout,
                SetAttribute(Attribute::Reset),
                SetForegroundColor(Color::Green),
            ),
            Style::Miss => queue!(
                self.stdout,
                SetAttribute(Attribute::Underlined),
                SetForegroundColor(Color::Red),
            ),
            Style::Blank => queue!(
                self.stdout,
                SetAttribute(Attribute::Reset),
                SetForegroundColor(Color::DarkGrey),
            ),
        }
    }

    pub fn put_str(&mut self, s: &str) -> Result<()> {
        queue!(self.stdout, Print(s))
    }

    pub fn set_char(&mut self, c: char) -> Result<()> {
        queue!(
            self.stdout,
            Print(if c == '\n' { ' ' } else { c }),
            cursor::MoveLeft(1)
        )
    }

    pub fn debug(&mut self, s: &str) -> Result<()> {
        queue!(
            self.stdout,
            cursor::SavePosition,
            cursor::MoveTo(0, self.height),
            Print(" ".repeat(self.width as usize)),
            cursor::MoveTo(0, self.height),
            Print(s),
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
