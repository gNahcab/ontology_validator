use std::fs::File;
use std::io::{BufReader};
use serde_json::Value;
use std::path::Path;
use crate::error::ReadError;

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<Value, ReadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json_file: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");
    Ok(json_file)
}
