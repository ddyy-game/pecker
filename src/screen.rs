use std::io::{stdout, Stdout};

pub struct Screen {
    stdout: Stdout,
}

impl Screen {
    pub fn new() -> Self {
        Screen { stdout: stdout() }
    }
    pub fn hello(&self) {
        println!("hello, world");
    }
}
