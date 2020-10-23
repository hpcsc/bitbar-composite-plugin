use serde::Deserialize;
use crate::error;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub plugins: Vec<PluginConfig>
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PluginConfig {
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub path: String,
}

pub fn from_file(path: &str) -> Result<Config, error::CliError> {
    let f = std::fs::File::open(path)?;
    Ok(serde_yaml::from_reader(f)?)
}