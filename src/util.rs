use std::{fs::File, io::Write};

use anyhow::Result;

pub fn write_file(data: Option<String>, path: &str) -> Result<()> {

    let mut f = if std::path::Path::new(path.clone()).exists() {
        open_file(path.clone())?
    } else { create_file(path.clone())? };

    match data {
        Some(o) => { f.write(o.as_bytes())?; },
        None => {},
    }

    Ok(())
}

pub fn read_file(path: &str) -> Result<String> { Ok(std::fs::read_to_string(path)?) }

pub fn create_file(path: &str) -> Result<File> { Ok(std::fs::File::create(path)?) }

fn open_file(path: &str) -> Result<File> { Ok(std::fs::OpenOptions::new().append(true).open(path)?) }