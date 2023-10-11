use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)?;
    Ok(file_str)
}

pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
