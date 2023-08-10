use std::io::Result;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    style::Color,
    terminal::enable_raw_mode,
};

use crate::{
    screen::MainScreen,
    text::{Action, TextLines},
};

pub struct Pecker {
    screen: MainScreen,
    text_lines: TextLines,
}

impl Pecker {
    pub fn new() -> Self {
        let screen = MainScreen::new();
        let text_lines = TextLines::new(
            "Hello, world! This is example text from pecker.",
            screen.width,
            screen.height,
        );
        Pecker { screen, text_lines }
    }

    pub fn reset(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.text_lines.redraw(&mut self.screen)?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            match read()? {
                Event::Key(event) => {
                    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
                    {
                        self.screen.clear()?;
                        break;
                    }

                    if event.code == KeyCode::Backspace {
                        self.screen.set_color(Color::DarkGrey)?;
                        self.screen.set_char(self.text_lines.current() as char)?;
                        if self.text_lines.backward() {
                            self.text_lines.redraw(&mut self.screen)?;
                        } else {
                            let (x, y) = self.text_lines.cursor_pos;
                            self.screen.move_to(x, y)?;
                            self.screen.flush()?;
                        }
                        continue;
                    }

                    let c = match event.code {
                        KeyCode::Enter => Some('\n'),
                        KeyCode::Char(c) => Some(c),
                        _ => None,
                    };

                    if let Some(c) = c {
                        let current_char = self.text_lines.current();
                        let action = self.text_lines.forward(c as u8);
                        if action == Action::Mismatch {
                            self.screen.set_color(Color::Red)?;
                            self.screen.set_char(current_char as char)?;
                        } else {
                            self.screen.set_color(Color::Green)?;
                            self.screen.set_char(current_char as char)?;
                        }
                        let (x, y) = self.text_lines.cursor_pos;
                        self.screen.move_to(x, y)?;
                        self.screen.flush()?;

                        if action == Action::Redraw {
                            self.text_lines.redraw(&mut self.screen)?;
                        } else if action == Action::End {
                            break;
                        }
                    }
                }
                Event::Resize(width, height) => {
                    self.screen.set_size(width, height);
                    if self.text_lines.set_size(width, height) {
                        self.text_lines.redraw(&mut self.screen)?;
                    }
                }
                Event::FocusGained => (),
                Event::FocusLost => (),
                Event::Mouse(_) => (),
            }
        }

        Ok(())
    }
}

impl Default for Pecker {
    fn default() -> Self {
        Self::new()
    }
}
