use crate::config;
use crate::error::CliError;
use crate::config::PluginConfig;
use std::process::Command;

#[derive(PartialEq, Debug)]
pub struct ExecutionOutput {
    pub plugin: String,
    pub output: String
}

pub struct Executor {
    config: config::Config
}

pub fn new(config: config::Config) -> Executor {
    Executor { config }
}

impl Executor {
    pub fn execute(&self) -> Vec<Result<ExecutionOutput, CliError>> {
        self.config.plugins.iter()
            .map(Executor::execute_single_plugin)
            .collect::<Vec<Result<ExecutionOutput, CliError>>>()
    }

    fn execute_single_plugin(plugin_config: &PluginConfig) -> Result<ExecutionOutput, CliError> {
        let output = Command::new(&plugin_config.command)
            .args(&plugin_config.args)
            .output()?;

        Ok(ExecutionOutput {
            plugin: plugin_config.display_name.clone(),
            output: String::from_utf8(output.stdout)?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, PluginConfig};

    #[test]
    fn execute_returns_output_of_all_plugins_when_successful() {
        let c = Config {
            plugins: vec![PluginConfig {
                display_name: "Plugin 1".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-1'".to_string()]
            }, PluginConfig {
                display_name: "Plugin 2".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-2'".to_string()]
            }]
        };

        let e = new(c);

        let result = e.execute();

        assert_eq!(2, result.len());

        let expected_first_result = &ExecutionOutput{
            plugin: "Plugin 1".to_string(),
            output: "plugin-1".to_string()
        };
        assert_eq!(expected_first_result, result[0].as_ref().unwrap());

        let expected_second_result = &ExecutionOutput{
            plugin: "Plugin 2".to_string(),
            output: "plugin-2".to_string()
        };
        assert_eq!(expected_second_result, result[1].as_ref().unwrap());
    }

    #[test]
    fn execute_returns_error_when_a_command_fails() {
        let c = Config {
            plugins: vec![PluginConfig {
                display_name: "Plugin 1".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-1'".to_string()]
            }, PluginConfig {
                display_name: "Plugin 2".to_string(),
                command: "some-random-command".to_string(),
                args: vec!()
            }]
        };

        let e = new(c);

        let result = e.execute();

        assert_eq!(2, result.len());

        let expected_first_result = &ExecutionOutput{
            plugin: "Plugin 1".to_string(),
            output: "plugin-1".to_string()
        };
        assert_eq!(expected_first_result, result[0].as_ref().unwrap());
        assert!(result[1].is_err());
    }
}