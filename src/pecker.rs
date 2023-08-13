use std::io::Result;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode,
};

use crate::{
    screen::{MainScreen, Style},
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
        );
        Pecker { screen, text_lines }
    }

    pub fn reset(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.text_lines.redraw(&mut self.screen)?;
        self.screen
            .debug(&format!("next: {}", self.text_lines.current() as char))?;
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
                        self.screen.set_style(Style::Blank)?;
                        self.screen.set_char(self.text_lines.current() as char)?;
                        match self.text_lines.backward() {
                            Action::Redraw => {
                                self.text_lines.redraw(&mut self.screen)?;
                            }
                            _ => {
                                self.text_lines.move_to_cursor(&mut self.screen)?;
                            }
                        };

                        if self.text_lines.n_miss > 0 {
                            self.screen.debug("next: backspace")?;
                        } else {
                            self.screen
                                .debug(&format!("next: {}", self.text_lines.current() as char))?;
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

                        match action {
                            Action::Miss => {
                                self.screen.set_style(Style::Miss)?;
                                self.screen.set_char(current_char as char)?;
                            }
                            _ => {
                                self.screen.set_style(Style::Hit)?;
                                self.screen.set_char(current_char as char)?;
                            }
                        };

                        match action {
                            Action::Redraw => self.text_lines.redraw(&mut self.screen)?,
                            _ => {
                                self.text_lines.move_to_cursor(&mut self.screen)?;
                            }
                        }

                        if self.text_lines.n_miss > 0 {
                            self.screen.debug("next: backspace")?;
                        } else {
                            self.screen
                                .debug(&format!("next: {}", self.text_lines.current() as char))?;
                        }

                        if matches!(action, Action::End) {
                            self.screen.clear()?;
                            break;
                        }
                    }
                }
                Event::Resize(width, height) => {
                    self.screen.set_size(width, height);
                    self.text_lines.rewrap(width);
                    self.text_lines.redraw(&mut self.screen)?;
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
