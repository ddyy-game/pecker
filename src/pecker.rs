use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode,
    Result,
};

use crate::screen::MainScreen;

pub struct Pecker {
    screen: MainScreen,
}

impl Pecker {
    pub fn new() -> Self {
        Pecker {
            screen: MainScreen::new(),
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.screen.reset()?;
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
                    self.screen.clear()?;
                    self.screen.put_str_centered(&format!("{:?}", event), 4)?;
                    self.screen.flush()?;
                }
                Event::Resize(width, height) => {
                    self.screen.set_size(width, height);
                    self.screen.reset()?;
                    self.screen
                        .put_str_centered(&format!("resize: {}x{}", width, height), -4)?;
                    self.screen.flush()?;
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
