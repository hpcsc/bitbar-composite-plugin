use crate::config;
use crate::error::CliError;

pub struct Executor {
    config: config::Config
}

pub fn new(config: config::Config) -> Executor {
    Executor { config }
}

impl Executor {
    pub fn execute(&self) -> Result<String, CliError> {
        Ok(format!("{:?}", self.config))
    }
}