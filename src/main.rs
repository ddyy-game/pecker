use std::io::stdout;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use pecker::screen::MainScreen;

fn main() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    let mut screen = MainScreen::new();
    screen.put_str("hello, world", 2).unwrap();
    screen.flush().unwrap();
    loop {}
}
