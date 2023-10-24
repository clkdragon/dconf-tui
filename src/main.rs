fn print_version() -> Result<i32, AppError> {
    let version = env!("CARGO_PKG_VERSION");
    writeln!(&mut std::io::stdout(), "{PROGRAM_NAME}-{version}")?;
    Ok(0)
}

fn main() {
    println!("Hello, world!");
}
