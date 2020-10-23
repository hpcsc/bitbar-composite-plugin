use crate::config;
use crate::error::CliError;
use crate::config::PluginConfig;
use tokio::process::Command;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;

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
    pub async fn execute(&self) -> Vec<Result<ExecutionOutput, CliError>> {
        self.config.plugins.iter()
            .map(Executor::execute_single_plugin)
            .collect::<FuturesUnordered<_>>()
            .collect()
            .await
    }

    async fn execute_single_plugin(plugin_config: &PluginConfig) -> Result<ExecutionOutput, CliError> {
        let output = Command::new(&plugin_config.command)
            .args(&plugin_config.args)
            .output()
            .await?;

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

    #[tokio::test]
    async fn execute_returns_output_of_all_plugins_when_successful() {
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

        let result = e.execute().await;

        assert_eq!(2, result.len());

        let expected_first_result = ExecutionOutput{
            plugin: "Plugin 1".to_string(),
            output: "plugin-1".to_string()
        };
        let plugin_1_result = find_output_by_plugin(&result, "Plugin 1".to_string());
        assert!(plugin_1_result.is_some());
        assert_eq!(&expected_first_result, plugin_1_result.unwrap());

        let expected_second_result = ExecutionOutput{
            plugin: "Plugin 2".to_string(),
            output: "plugin-2".to_string()
        };
        let plugin_2_result = find_output_by_plugin(&result, "Plugin 2".to_string());
        assert!(plugin_2_result.is_some());
        assert_eq!(&expected_second_result, plugin_2_result.unwrap());
    }

    #[tokio::test]
    async fn execute_returns_error_when_a_command_fails() {
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

        let result = e.execute().await;

        assert_eq!(2, result.len());

        let plugin_1_expected_result = ExecutionOutput{
            plugin: "Plugin 1".to_string(),
            output: "plugin-1".to_string()
        };
        let plugin_1_result = find_output_by_plugin(&result, "Plugin 1".to_string());
        assert!(plugin_1_result.is_some());
        assert_eq!(&plugin_1_expected_result, plugin_1_result.unwrap());
        assert!(find_error_output(&result).is_some());
    }

    fn find_output_by_plugin(result: &Vec<Result<ExecutionOutput, CliError>>, plugin_name: String) -> Option<&ExecutionOutput> {
        for r in result {
            if (*r).is_ok() &&
                r.as_ref().unwrap().plugin == plugin_name {
                return Some(r.as_ref().unwrap())
            }
        }

        None
    }

    fn find_error_output(result: &Vec<Result<ExecutionOutput, CliError>>) -> Option<&CliError> {
        for r in result {
            if (*r).is_err() {
                return r.as_ref().err()
            }
        }

        None
    }
}