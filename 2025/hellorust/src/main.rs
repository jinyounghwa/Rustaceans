use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::create("hello.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}