use std::{cmp::Ordering, collections::HashMap, io::Result};

use crossterm::style::Stylize;

use crate::{screen::MainScreen, text::Expect};

#[derive(Default)]
pub struct Layout {
    keyboard_pos: HashMap<char, (i16, i16, bool)>,
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
        layout.keyboard_pos.insert(' ', (0, 0, false));
        for (j, row) in layout_str.iter().enumerate() {
            for (i, c) in row[0].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, (-(i as i16 + 1), j as i16, false));
            }
        }
        for (j, row) in layout_str.iter().enumerate() {
            for (i, c) in row[1].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, (-(i as i16 + 1), j as i16, true));
            }
        }
        for (j, row) in layout_str.iter().enumerate() {
            for (i, c) in row[2].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, (i as i16 + 1, j as i16, false));
            }
        }
        for (j, row) in layout_str.iter().enumerate() {
            for (i, c) in row[3].char_indices() {
                layout
                    .keyboard_pos
                    .insert(c, (i as i16 + 1, j as i16, true));
            }
        }
        layout
    }

    pub fn redraw(&self, screen: &mut MainScreen, c: Expect) -> Result<()> {
        screen.debug(&format!("{c:?}"))?;
        screen.save()?;
        self.clear(screen)?;

        let (col, row, shift) = *self
            .keyboard_pos
            .get(&match c {
                Expect::Char(c) => c,
                Expect::Softbreak => ' ',
                Expect::Backspace => '\x08',
            })
            .unwrap_or(&(0i16, -1i16, false));

        // left hand
        screen.move_to(screen.width / 2 - 4, screen.height - 8)?;
        for i in 0..4 {
            screen.move_by(-5, 0)?;
            let (highlight, len, dir) = if col >= 0 {
                (shift && i == 3, 1, -1)
            } else {
                (-col.min(-2).max(-5) == i + 2, 4 - row as u16, col + i + 2)
            };
            self.draw_finger(screen, len, dir, highlight)?;
        }

        // right hand
        screen.move_to(screen.width / 2 + 1, screen.height - 8)?;
        for i in 0..4 {
            screen.move_by(5, 0)?;
            let (highlight, len, dir) = if col <= 0 {
                (shift && i == 3, 1, 1)
            } else {
                (col.max(2).min(5) == i + 2, 4 - row as u16, col - i - 2)
            };
            self.draw_finger(screen, len, dir, highlight)?;
        }

        // thumb
        if col == 0 && row == 0 {
            screen.move_to(screen.width / 2 - 2, screen.height - 6)?;
            screen.put("====".bold().green())?;
        }

        screen.load()?;
        screen.flush()?;
        Ok(())
    }

    fn clear(&self, screen: &mut MainScreen) -> Result<()> {
        for i in 0..5 {
            screen.move_to(0, screen.height - 8 - i)?;
            screen.put(" ".repeat(screen.width as usize).reset())?;
        }
        screen.move_to(screen.width / 2 - 2, screen.height - 6)?;
        screen.put("    ".reset())?;
        Ok(())
    }

    fn draw_finger(
        &self,
        screen: &mut MainScreen,
        len: u16,
        direction: i16,
        highlight: bool,
    ) -> Result<()> {
        let tip = if highlight {
            " ⌒ ".bold().green()
        } else {
            " ⌒ ".reset()
        };
        let (side_str, direction, offset) = match (highlight, direction.cmp(&0)) {
            (false, _) | (true, Ordering::Equal) => ("| |", 0, 0),
            (true, Ordering::Less) => ("\\ \\", -1, (direction + 1) * 2),
            (true, Ordering::Greater) => ("/ /", 1, (direction - 1) * 2),
        };
        let side = if highlight {
            side_str.bold().green()
        } else {
            side_str.reset()
        };
        let len = if highlight { len } else { 2 };
        screen.move_by(offset, 0)?;
        for _ in 0..len {
            screen.put(side)?;
            screen.move_by(-3 + direction, -1)?;
        }
        screen.put(tip)?;
        screen.move_by(-3 - len as i16 * direction - offset, len as i16)?;
        Ok(())
    }
}
