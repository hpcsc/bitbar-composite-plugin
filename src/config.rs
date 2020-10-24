use crate::error;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub plugins: Vec<PluginConfig>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct PluginConfig {
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default, alias = "showInSubMenu")]
    pub show_in_sub_menu: bool,
}

pub fn from_file(path: &str) -> Result<Config, error::CliError> {
    let f = std::fs::File::open(path)?;
    Ok(serde_yaml::from_reader(f)?)
}
