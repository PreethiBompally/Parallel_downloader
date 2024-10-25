use std::fs::OpenOptions;
use std::io::Write;
// use std::sync::{Arc, Mutex};

pub fn save_file(ext: &str, buffer: &[u8], part: usize, length: usize) -> std::io::Result<()> {
    let file_name = format!("part_{}.{}", part, ext);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;
    file.write_all(&buffer[..length])?;
    Ok(())
}