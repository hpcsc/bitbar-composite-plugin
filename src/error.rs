use std::io;

#[derive(Debug)]
pub enum CliError {
    IO(io::Error),
    Deserialize(serde_yaml::Error)
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IO(err)
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(err: serde_yaml::Error) -> CliError {
        CliError::Deserialize(err)
    }
}