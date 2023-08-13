use std::io::Result;

use crate::screen::{MainScreen, Style};

#[derive(Default)]
pub struct TextLines {
    raw_text: Vec<u8>,
    lines: Vec<Vec<u8>>,
    pub n_hit: usize,
    pub n_miss: usize,
    pub cursor_pos: (u16, u16),
}

pub enum Action {
    Hit,
    Miss,
    Redraw,
    End,
}

impl TextLines {
    pub fn new(text: &str, width: u16) -> TextLines {
        let mut raw_text: Vec<u8> = text.bytes().collect();
        raw_text.push(b' ');
        let mut t = TextLines {
            raw_text,
            ..Default::default()
        };
        t.rewrap(width);
        t
    }

    pub fn rewrap(&mut self, width: u16) {
        let text_width =
            (width - 8).min(((self.raw_text.len() as f32).sqrt().ceil() + 15.0) as u16);
        self.lines = wrap_string(&self.raw_text, text_width);
        let mut n = self.n_hit + self.n_miss;
        let mut pos = (0u16, 0u16);
        while n >= self.lines[pos.1 as usize].len() {
            n -= self.lines[pos.1 as usize].len();
            pos.1 += 1;
        }
        pos.0 = n as u16;
        self.cursor_pos = pos;
    }

    #[inline]
    pub fn current(&self) -> u8 {
        self.raw_text[self.n_hit + self.n_miss]
    }

    #[inline]
    pub fn at_line_end(&self, column: u16, row: u16) -> bool {
        column as usize == self.lines[row as usize].len() - 1
    }

    pub fn forward(&mut self, c: u8) -> Action {
        let (x, y) = self.cursor_pos;

        // if already at the end
        if self.n_hit + self.n_miss == self.raw_text.len() - 1 {
            return if self.n_miss > 0 {
                Action::Miss
            } else {
                Action::End
            };
        }

        // check if matches
        if self.n_miss == 0
            && (c == self.raw_text[self.n_hit] || self.at_line_end(x, y) && c == b'\n')
        {
            self.n_hit += 1;
        } else {
            self.n_miss += 1
        }

        // move cursor
        if self.at_line_end(x, y) {
            self.cursor_pos.1 += 1;
            self.cursor_pos.0 = 0;
        } else {
            self.cursor_pos.0 += 1
        }

        // respond action to be taken after moving forward
        if self.cursor_pos.0 == 0 {
            Action::Redraw
        } else if self.n_miss > 0 {
            Action::Miss
        } else if self.n_hit == self.raw_text.len() - 1 {
            Action::End
        } else {
            Action::Hit
        }
    }

    pub fn backward(&mut self) -> Action {
        if self.n_miss == 0 {
            return Action::Hit;
        }
        self.n_miss -= 1;
        let pos = &mut self.cursor_pos;
        if pos.0 as usize == 0 {
            pos.1 -= 1;
            pos.0 = (self.lines[pos.1 as usize].len() - 1) as u16;
            return Action::Redraw;
        }
        pos.0 -= 1;
        if self.n_miss == 0 {
            Action::Hit
        } else {
            Action::Miss
        }
    }

    pub fn move_to_cursor(&mut self, screen: &mut MainScreen) -> Result<(u16, u16)> {
        let (x, y) = self.move_to(screen, self.cursor_pos.0, self.cursor_pos.1)?;
        screen.flush()?;
        Ok((x, y))
    }

    pub fn move_to(
        &mut self,
        screen: &mut MainScreen,
        column: u16,
        row: u16,
    ) -> Result<(u16, u16)> {
        let offset_x = (screen.width - self.lines[row as usize].len() as u16) / 2;
        let offset_y = (screen.height - self.lines.len() as u16) / 2;
        screen.move_to(column + offset_x, row + offset_y)?;
        Ok((column + offset_x, row + offset_y))
    }

    pub fn redraw(&mut self, screen: &mut MainScreen) -> Result<()> {
        screen.clear()?;
        let mut n_correct = self.n_hit;
        let mut n_mistaken = self.n_miss;

        for i in 0..self.lines.len() {
            self.move_to(screen, 0, i as u16)?;
            let line = &self.lines[i];
            if n_correct > 0 {
                screen.set_style(Style::Hit)?;
                if line.len() <= n_correct {
                    screen.put_str(std::str::from_utf8(line).expect("strings must be utf8"))?;
                    n_correct -= line.len();
                    continue;
                } else {
                    screen.put_str(
                        std::str::from_utf8(&line[..n_correct])
                            .expect("string slices must be utf-8"),
                    )?;
                }
            }
            if n_correct < line.len() && n_mistaken > 0 {
                screen.set_style(Style::Miss)?;
                if line.len() - n_correct <= n_mistaken {
                    screen.put_str(
                        std::str::from_utf8(&line[n_correct..]).expect("strings must be utf8"),
                    )?;
                    n_mistaken -= line.len() - n_correct;
                    n_correct = 0;
                    continue;
                } else {
                    screen.put_str(
                        std::str::from_utf8(&line[n_correct..n_correct + n_mistaken])
                            .expect("string slices must be utf-8"),
                    )?;
                }
            }
            if n_mistaken + n_correct < line.len() {
                screen.set_style(Style::Blank)?;
                screen.put_str(
                    std::str::from_utf8(&line[n_correct + n_mistaken..])
                        .expect("strings must be utf8"),
                )?;
                n_correct = 0;
                n_mistaken = 0;
            }
        }

        self.move_to_cursor(screen)?;
        screen.flush()?;
        Ok(())
    }
}

fn wrap_string(text: &[u8], width: u16) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();

    for word in text.split(|c| c == &b' ') {
        if current_line.len() + word.len() > width as usize {
            lines.push(current_line);
            current_line = Vec::new();
        }
        current_line.extend_from_slice(word);
        current_line.push(b' ');
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}
