use std::io::prelude::*;

pub const PROGRAM_NAME: &str = "dconf-tui";

fn print_version() -> Result<i32, std::io::Error> {
    let version = env!("CARGO_PKG_VERSION");
    writeln!(&mut std::io::stdout(), "{PROGRAM_NAME}-{version}")?;
    Ok(0)
}

fn main() {
    println!("Hello, world!");
}
