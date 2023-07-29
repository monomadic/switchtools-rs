use switchtools::{self, NSPFile};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: nsp-extract <FILE>");
        return Ok(());
    };

    let file = std::fs::read(&path)?;

    NSPFile::read(file.as_slice())?;

    Ok(())
}
