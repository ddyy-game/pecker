use std::io::Result;

use crate::layout::Layout;

use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode,
};

use crate::{
    screen::{MainScreen, Styled},
    text::{State, TextLines},
};

pub struct Pecker {
    screen: MainScreen,
    text_lines: TextLines,
    layout: Layout,
}

impl Pecker {
    #[must_use]
    pub fn new() -> Self {
        let screen = MainScreen::new();
        let text_lines = TextLines::new();
        let layout = Layout::new();
        Self {
            screen,
            text_lines,
            layout,
        }
    }

    pub fn reset(&mut self, text: &str, align_center: bool) -> Result<()> {
        enable_raw_mode()?;
        let expect = self
            .text_lines
            .reset(Some(text), self.screen.width, align_center);
        self.text_lines.redraw(&mut self.screen)?;
        self.layout.redraw(&mut self.screen, expect)?;
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
                        // step 1. update text lines
                        // move backward
                        let (expect, redraw) = self.text_lines.backward();

                        // step 2. update screen
                        // move the cursor on screen
                        if redraw {
                            self.text_lines.redraw(&mut self.screen)?;
                        } else {
                            self.text_lines.move_to_cursor(&mut self.screen)?;
                        }
                        // reset style for current char
                        let current_char = self.text_lines.current() as char;
                        self.screen.set(current_char.blank())?;

                        // step 3. inspect next char
                        self.layout.redraw(&mut self.screen, expect)?;

                        continue;
                    }

                    let c = match event.code {
                        KeyCode::Enter => Some('\n'),
                        KeyCode::Char(c) => Some(c),
                        _ => None,
                    };

                    if let Some(c) = c {
                        // step 1. update text lines
                        // record current char
                        let current_char = self.text_lines.current() as char;
                        // move forward
                        let (state, expect, redraw) = self.text_lines.forward(c);

                        // step 2. update screen
                        // set style for current char
                        match state {
                            State::Hit | State::End => {
                                self.screen.set(current_char.hit())?;
                            }
                            State::Miss => {
                                self.screen.set(current_char.miss())?;
                            }
                        };
                        // actually move the cursor on screen
                        if redraw {
                            self.text_lines.redraw(&mut self.screen)?;
                        } else {
                            self.text_lines.move_to_cursor(&mut self.screen)?;
                        }

                        // step 3. inspect next char
                        self.layout.redraw(&mut self.screen, expect)?;

                        if matches!(state, State::End) {
                            self.screen.clear()?;
                            break;
                        }
                    }
                }
                Event::Resize(width, height) => {
                    self.screen.set_size(width, height);
                    let expect = self
                        .text_lines
                        .reset(None, width, self.text_lines.align_center);
                    self.text_lines.redraw(&mut self.screen)?;
                    self.layout.redraw(&mut self.screen, expect)?;
                }
                Event::FocusGained | Event::FocusLost | Event::Mouse(_) => (),
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
