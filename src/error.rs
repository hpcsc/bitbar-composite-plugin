use std::{io, string};

#[derive(Debug)]
pub enum CliError {
    IO(io::Error),
    Deserialize(serde_yaml::Error),
    StringFromUtf8(string::FromUtf8Error),
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

impl From<string::FromUtf8Error> for CliError {
    fn from(err: string::FromUtf8Error) -> CliError {
        CliError::StringFromUtf8(err)
    }
}
