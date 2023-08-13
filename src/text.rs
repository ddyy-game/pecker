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

pub enum State {
    Hit,
    Miss,
    End,
}

#[derive(Debug)]
pub enum Expect {
    Char(char),
    Softbreak,
    Backspace,
}

impl TextLines {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self, text: Option<&str>, width: u16) -> Expect {
        if let Some(text) = text {
            self.raw_text = text.bytes().collect();
            self.raw_text.push(b' ');
            self.n_hit = 0;
            self.n_miss = 0;
            self.cursor_pos = (0, 0);
        }

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

        Expect::Char(self.current() as char)
    }

    #[inline]
    #[must_use]
    pub fn current(&self) -> u8 {
        self.raw_text[self.n_hit + self.n_miss]
    }

    #[inline]
    #[must_use]
    pub fn at_line_end(&self) -> bool {
        let (column, row) = self.cursor_pos;
        column as usize == self.lines[row as usize].len() - 1
    }

    #[inline]
    #[must_use]
    pub fn is_softbreak(&self) -> bool {
        self.at_line_end() && self.current() == b' '
    }

    pub fn forward(&mut self, c: char) -> (State, Expect, bool) {
        // do not move any further if already at the end
        if self.n_hit + self.n_miss != self.raw_text.len() - 1 {
            // check if matches
            if self.n_miss == 0
                && (c as u8 == self.raw_text[self.n_hit] || self.is_softbreak() && c == '\n')
            {
                self.n_hit += 1;
            } else {
                self.n_miss += 1;
            }

            // move cursor
            if self.at_line_end() {
                self.cursor_pos.1 += 1;
                self.cursor_pos.0 = 0;
            } else {
                self.cursor_pos.0 += 1;
            }
        }

        // output
        let state = if self.n_miss > 0 {
            State::Miss
        } else if self.n_hit == self.raw_text.len() - 1 {
            State::End
        } else {
            State::Hit
        };
        let expect = if self.n_miss != 0 {
            Expect::Backspace
        } else if self.is_softbreak() {
            Expect::Softbreak
        } else {
            Expect::Char(self.current() as char)
        };
        let redraw = self.cursor_pos.0 == 0;

        (state, expect, redraw)
    }

    pub fn backward(&mut self) -> (Expect, bool) {
        if self.n_miss != 0 {
            self.n_miss -= 1;
            let (x, y) = self.cursor_pos;
            if x == 0 {
                self.cursor_pos.1 -= 1;
                self.cursor_pos.0 = (self.lines[y as usize - 1].len() - 1) as u16;
            } else {
                self.cursor_pos.0 -= 1;
            };
        }

        if self.n_miss != 0 {
            (Expect::Backspace, false)
        } else if self.is_softbreak() {
            (Expect::Softbreak, true)
        } else {
            (Expect::Char(self.current() as char), false)
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
        let offset_x = (screen.width - self.lines[row as usize].len() as u16 - 1) / 2;
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
            let mut line = self.lines[i].clone();
            let len = line.len();
            line[len - 1] = b' ';
            if n_correct > 0 {
                screen.set_style(Style::Hit)?;
                if line.len() <= n_correct {
                    screen.put_str(std::str::from_utf8(&line).expect("strings must be utf8"))?;
                    n_correct -= line.len();
                    continue;
                }
                screen.put_str(
                    std::str::from_utf8(&line[..n_correct]).expect("string slices must be utf-8"),
                )?;
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
                }
                screen.put_str(
                    std::str::from_utf8(&line[n_correct..n_correct + n_mistaken])
                        .expect("string slices must be utf-8"),
                )?;
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
    let mut len = 0;

    for word in text.split(|c| c == &b' ' || c == &b'\n') {
        let c = if len == 0 { 0 } else { text[len - 1] };
        if c == b'\n' || current_line.len() + word.len() > width as usize {
            lines.push(current_line);
            current_line = Vec::new();
        }
        len += word.len() + 1;
        current_line.extend_from_slice(word);
        if len != text.len() + 1 {
            current_line.push(text[len - 1]);
        };
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}
