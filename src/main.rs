use std::io::Result;

use pecker::pecker::Pecker;

fn main() -> Result<()> {
    let mut pecker = Pecker::new();
    pecker.reset()?;

    // start main event loop
    pecker.start()?;

    Ok(())
}
