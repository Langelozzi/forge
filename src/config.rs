use crate::constants::CONFIG_FILENAME;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct ForgeConfig {
    pub project: ProjectSection,
    pub build: BuildSection,
}

#[derive(Deserialize, Debug)]
pub struct ProjectSection {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct BuildSection {
    pub compiler: String,
    pub flags: Vec<String>,
    pub ignore_files: Vec<String>,
}

impl ForgeConfig {
    /// The "Constructor" that reads the file and returns the object
    pub fn load() -> std::io::Result<Self> {
        let content = fs::read_to_string(CONFIG_FILENAME)?;

        let config: ForgeConfig = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }
}
