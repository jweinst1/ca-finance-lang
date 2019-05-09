use std::io::{self, Write};
/// The core help message for the `Sapphire` cli
static HELP_MESSAGE: &'static str = "~~~ Sapphire Help ~~~\n";

fn main() -> io::Result<()> {
    io::stdout().write(HELP_MESSAGE.as_bytes())?;

    Ok(())
}
