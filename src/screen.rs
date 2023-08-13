use std::{
    fmt::Display,
    io::{stdout, Result, Stdout, Write},
};

use crossterm::{
    cursor, execute, queue,
    style::{ContentStyle, PrintStyledContent, StyledContent, Stylize},
    terminal::{size, Clear, ClearType},
};

pub struct MainScreen {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

pub trait Styled {
    type Styled: AsRef<ContentStyle>;
    fn hit(self) -> Self::Styled;
    fn miss(self) -> Self::Styled;
    fn blank(self) -> Self::Styled;
    fn default(self) -> Self::Styled;
}

macro_rules! impl_styled {
    ($($t:ty),*) => { $(
        impl Styled for $t {
            type Styled = StyledContent<Self>;
            #[inline]
            fn hit(self) -> Self::Styled {
                self.green()
            }
            #[inline]
            fn miss(self) -> Self::Styled {
                self.red().underlined()
            }
            #[inline]
            fn blank(self) -> Self::Styled {
                self.dark_grey()
            }
            #[inline]
            fn default(self) -> Self::Styled {
                self.reset()
            }
        }
    )* }
}
impl_styled!(char, String, &str);

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

    pub fn put_str<D: Display>(&mut self, s: StyledContent<D>) -> Result<()> {
        queue!(self.stdout, PrintStyledContent(s))
    }

    pub fn set_char(&mut self, c: StyledContent<char>) -> Result<()> {
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
