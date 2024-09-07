use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub tab_size: usize,
    pub use_spaces: bool,
    pub auto_indent: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            tab_size: 4,
            use_spaces: true,
            auto_indent: true,
        }
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&contents)?;
        Ok(settings)
    }

    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}