use std::fs::File;
use std::io::{Read, Result};

use clap::Parser;
use pecker::pecker::Pecker;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    file: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // read content from file
    let mut f = File::open(&cli.file)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;

    // initialize pecker
    let mut pecker = Pecker::new();
    pecker.reset(text.trim_end(), cli.file.ends_with(".txt"))?;

    // start main event loop
    pecker.start()?;

    Ok(())
}
