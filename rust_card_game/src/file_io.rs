use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

pub fn save_data<T: Serialize>(path: &Path, data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string_pretty(data)?;
    fs::write(path, toml_string)?;
    Ok(())
}

pub fn load_data<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let toml_string = fs::read_to_string(path)?;
    let data: T = toml::from_str(&toml_string)?;
    Ok(data)
}


