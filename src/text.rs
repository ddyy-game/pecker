use std::io::Result;

use crate::screen::MainScreen;

pub struct TextLines {
    raw_text: Vec<u8>,
    lines: Vec<Vec<u8>>,
    text_pos: usize,
    pub cursor_pos: (u16, u16),
    screen_height: u16,
    screen_width: u16,
    mismatches: usize,
}

#[derive(PartialEq, Eq)]
pub enum Action {
    Match,
    Mismatch,
    Redraw,
    End,
}

impl TextLines {
    pub fn new(text: &str, width: u16, height: u16) -> TextLines {
        let mut raw_text: Vec<u8> = text.bytes().collect();
        let lines = wrap_string(&raw_text, width);
        raw_text.push(b' ');
        TextLines {
            raw_text,
            lines,
            text_pos: 0,
            cursor_pos: (0, 0),
            screen_width: width,
            screen_height: height,
            mismatches: 0,
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) -> bool {
        if self.screen_width == width && self.screen_height == height {
            return false;
        }
        self.screen_width = width;
        self.screen_height = height;
        self.lines = wrap_string(&self.raw_text, width);
        let mut n = self.text_pos + self.mismatches;
        let mut pos = (0u16, 0u16);
        while n >= self.lines[pos.1 as usize].len() {
            n -= self.lines[pos.1 as usize].len();
            pos.1 += 1;
        }
        pos.0 = n as u16;
        self.cursor_pos = pos;
        return true;
    }

    pub fn current(&self) -> u8 {
        self.raw_text[self.text_pos + self.mismatches]
    }

    pub fn forward(&mut self, c: u8) -> Action {
        let pos = &mut self.cursor_pos;
        if pos.1 as usize == self.lines.len() - 1
            && pos.0 as usize == self.lines[pos.1 as usize].len() - 1
        {
            return if self.mismatches > 0 {
                Action::Mismatch
            } else {
                Action::End
            };
        }

        if c == self.raw_text[self.text_pos] && self.mismatches == 0 {
            self.text_pos += 1;
        } else {
            self.mismatches += 1
        }

        if pos.0 as usize == self.lines[pos.1 as usize].len() - 1 {
            pos.1 += 1;
            pos.0 = 0;
        } else {
            pos.0 += 1
        }

        if pos.0 as usize == 0 {
            Action::Redraw
        } else if self.mismatches > 0 {
            Action::Mismatch
        } else if self.text_pos == self.raw_text.len() - 1 {
            Action::End
        } else {
            Action::Match
        }
    }

    pub fn backward(&mut self) -> bool {
        if self.mismatches == 0 {
            return false;
        }
        self.mismatches -= 1;
        let pos = &mut self.cursor_pos;
        if pos.0 as usize == 0 {
            pos.1 -= 1;
            pos.0 = (self.lines[pos.1 as usize].len() - 1) as u16;
            return true;
        }
        pos.0 -= 1;
        return false;
    }

    pub fn redraw(&mut self, screen: &mut MainScreen) -> Result<()> {
        screen.clear()?;
        let mut n_correct = self.text_pos;
        let mut n_mistaken = self.mismatches;
        for (i, line) in self.lines.iter().enumerate() {
            screen.move_to(0, i as u16)?;
            if n_correct > 0 {
                screen.set_color(crossterm::style::Color::Green)?;
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
                screen.set_color(crossterm::style::Color::Red)?;
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
                screen.set_color(crossterm::style::Color::DarkGrey)?;
                screen.put_str(
                    std::str::from_utf8(&line[n_correct + n_mistaken..])
                        .expect("strings must be utf8"),
                )?;
                n_correct = 0;
                n_mistaken = 0;
            }
        }
        screen.move_to(self.cursor_pos.0, self.cursor_pos.1)?;
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
    if current_line.len() > 0 {
        lines.push(current_line);
    }
    return lines;
}
