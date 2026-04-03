use std::{fs, path::Path};

use crate::util::result::Result;

pub fn read_file(path: &Path) -> Result<serde_json::Value> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn write_file(path: &Path, value: &serde_json::Value) -> Result<()> {
    let contents = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{contents}\n"))?;
    Ok(())
}
