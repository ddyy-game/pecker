use std::{collections::HashMap, io::Result};

use crate::{screen::MainScreen, text::Expect};

#[derive(Debug)]
pub enum Finger {
    Left(u16, u16),
    ShiftLeft(u16, u16),
    Right(u16, u16),
    ShiftRight(u16, u16),
    Thumb,
}

#[derive(Default)]
pub struct Layout {
    keyboard_pos: HashMap<char, Finger>,
}

impl Layout {
    #[must_use]
    pub fn new() -> Self {
        let mut layout = Self::default();
        let layout_str = vec![
            ["54321`", "%$#@!~", "67890-=\x08", "^&*()_+"],
            ["trewq\t", "TREWQ", "yuiop[]\\", "YUIOP{}|"],
            ["gfdsa", "GFDSA", "hjkl;'\n", "HJKL:\""],
            ["bvcxz", "BVCXZ", "nm,./", "NM<>?"],
        ];
        layout.keyboard_pos.insert(' ', Finger::Thumb);
        for (i, row) in layout_str.iter().enumerate() {
            for (j, c) in row[0].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, Finger::Left(i as u16, j as u16));
            }
        }
        for (i, row) in layout_str.iter().enumerate() {
            for (j, c) in row[1].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, Finger::ShiftLeft(i as u16, j as u16));
            }
        }
        for (i, row) in layout_str.iter().enumerate() {
            for (j, c) in row[2].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, Finger::Right(i as u16, j as u16));
            }
        }
        for (i, row) in layout_str.iter().enumerate() {
            for (j, c) in row[3].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, Finger::ShiftRight(i as u16, j as u16));
            }
        }
        layout
    }

    pub fn redraw(&self, screen: &mut MainScreen, c: Expect) -> Result<()> {
        screen.save()?;
        match c {
            Expect::Char(c) => {
                screen.debug(&format!("{:?}", self.keyboard_pos.get(&c)))?;
            }
            Expect::Softbreak => {
                screen.debug(&format!("{:?}", self.keyboard_pos.get(&' ')))?;
            }
            Expect::Backspace => {
                screen.debug(&format!("{:?}", self.keyboard_pos.get(&'\x08')))?;
            }
        }
        screen.load()?;
        screen.flush()?;
        Ok(())
    }
}
