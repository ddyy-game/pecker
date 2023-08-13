use std::io::{stdout, Result, Stdout, Write};

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

    pub fn set_char(&mut self, c: StyledContent<char>) -> Result<()> {
        queue!(self.stdout, PrintStyledContent(c), cursor::MoveLeft(1))
    }

    pub fn debug(&mut self, s: &str) -> Result<()> {
        queue!(
            self.stdout,
            cursor::SavePosition,
            cursor::MoveTo(1, self.height - 1),
            PrintStyledContent(" ".repeat(self.width as usize - 2).underlined()),
            cursor::MoveTo(1, self.height - 1),
            PrintStyledContent(s.bold().underlined()),
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

pub trait Styled: Sized {
    type Formatted: Stylize<Styled = Self::Formatted>;
    fn format(self) -> Self::Formatted;

    fn hit(self) -> Self::Formatted {
        self.format().green()
    }
    fn miss(self) -> Self::Formatted {
        self.format().red().underlined()
    }
    fn blank(self) -> Self::Formatted {
        self.format().dark_grey()
    }
    fn default(self) -> Self::Formatted {
        self.format().reset()
    }
}

impl Styled for &str {
    type Formatted = StyledContent<String>;
    fn format(self) -> Self::Formatted {
        self.replace('\n', "⏎").stylize()
    }
}

impl Styled for String {
    type Formatted = StyledContent<String>;
    fn format(self) -> Self::Formatted {
        self.replace('\n', "⏎").stylize()
    }
}

impl Styled for char {
    type Formatted = StyledContent<char>;
    fn format(self) -> Self::Formatted {
        if self == '\n' {
            '⏎'.stylize()
        } else {
            self.stylize()
        }
    }
}
