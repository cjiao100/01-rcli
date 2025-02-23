use anyhow::Result;
use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input).unwrap()))
    }
}

pub fn read_data(input: &str) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buffer = String::new();
    let _ = reader.read_to_string(&mut buffer);
    Ok(buffer)
}
