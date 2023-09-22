use std::fs::File;
use std::io::{Read, Result};

use clap::Parser;
use rand::{seq::IteratorRandom, thread_rng};

use pecker::pecker::Pecker;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    file: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut text = String::new();
    let mut align_center = true;

    if let Some(f) = cli.file {
        if !f.ends_with(".txt") {
            align_center = false;
        }
        // read content from file
        let mut f = File::open(&f)?;
        f.read_to_string(&mut text)?;
    } else {
        let mut rng = thread_rng();
        let words = include_str!("google-10000-english-usa-no-swears.txt");
        for word in words
            .split_whitespace()
            .take(1000)
            .choose_multiple(&mut rng, 20)
        {
            text.push_str(word);
            text.push(' ');
        }
    }

    // initialize pecker
    let mut pecker = Pecker::new();
    pecker.reset(text.trim_end(), align_center)?;

    // start main event loop
    pecker.start()?;

    Ok(())
}
