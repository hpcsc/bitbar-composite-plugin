use crate::config;
use crate::config::PluginConfig;
use crate::error::CliError;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use tokio::process::Command;

#[derive(PartialEq, Debug)]
pub struct ExecutionOutput {
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub plugin: String,
    pub result: Result<ExecutionOutput, CliError>,
}

pub fn new_execution_result_with_output(
    plugin: &str,
    stdout: &str,
    stderr: &str,
) -> ExecutionResult {
    ExecutionResult {
        plugin: String::from(plugin),
        result: Ok(ExecutionOutput {
            stdout: String::from(stdout),
            stderr: String::from(stderr),
        }),
    }
}

pub struct Executor<'a> {
    config: &'a config::Config,
}

pub fn new(config: &config::Config) -> Executor {
    Executor { config }
}

impl Executor<'_> {
    pub async fn execute(&self) -> Vec<ExecutionResult> {
        self.config
            .plugins
            .iter()
            .map(Executor::execute_single_plugin)
            .collect::<FuturesUnordered<_>>()
            .collect()
            .await
    }

    async fn execute_single_plugin(plugin_config: &PluginConfig) -> ExecutionResult {
        let output_result = Command::new(&plugin_config.command)
            .args(&plugin_config.args)
            .output()
            .await;

        if output_result.is_err() {
            return ExecutionResult {
                plugin: plugin_config.display_name.clone(),
                result: Err(output_result.unwrap_err().into()),
            };
        }

        let output = output_result.unwrap();

        let stdout_result = String::from_utf8(output.stdout);
        if stdout_result.is_err() {
            return ExecutionResult {
                plugin: plugin_config.display_name.clone(),
                result: Err(stdout_result.unwrap_err().into()),
            };
        }

        let stderr_result = String::from_utf8(output.stderr);
        if stderr_result.is_err() {
            return ExecutionResult {
                plugin: plugin_config.display_name.clone(),
                result: Err(stderr_result.unwrap_err().into()),
            };
        }

        ExecutionResult {
            plugin: plugin_config.display_name.clone(),
            result: Ok(ExecutionOutput {
                stdout: stdout_result.unwrap(),
                stderr: stderr_result.unwrap(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, PluginConfig};

    #[tokio::test]
    async fn execute_returns_output_of_all_plugins_when_successful() {
        let c = Config {
            plugins: vec![
                PluginConfig {
                    display_name: "Plugin 1".to_string(),
                    command: "bash".to_string(),
                    args: vec!["-c".to_string(), "echo -n 'plugin-1'".to_string()],
                    show_in_sub_menu: false,
                },
                PluginConfig {
                    display_name: "Plugin 2".to_string(),
                    command: "bash".to_string(),
                    args: vec!["-c".to_string(), "echo -n 'plugin-2'".to_string()],
                    show_in_sub_menu: false,
                },
            ],
        };

        let e = new(&c);

        let result = e.execute().await;

        assert_eq!(2, result.len());

        let expected_first_result = ExecutionOutput {
            stdout: String::from("plugin-1"),
            stderr: String::new(),
        };
        let plugin_1_result = find_output_by_plugin(&result, "Plugin 1".to_string());
        assert!(plugin_1_result.is_some());
        assert_eq!(
            &expected_first_result,
            plugin_1_result.unwrap().as_ref().unwrap()
        );

        let expected_second_result = ExecutionOutput {
            stdout: String::from("plugin-2"),
            stderr: String::new(),
        };
        let plugin_2_result = find_output_by_plugin(&result, "Plugin 2".to_string());
        assert!(plugin_2_result.is_some());
        assert_eq!(
            &expected_second_result,
            plugin_2_result.unwrap().as_ref().unwrap()
        );
    }

    #[tokio::test]
    async fn execute_returns_error_when_a_command_fails() {
        let c = Config {
            plugins: vec![
                PluginConfig {
                    display_name: "Plugin 1".to_string(),
                    command: "bash".to_string(),
                    args: vec!["-c".to_string(), "echo -n 'plugin-1'".to_string()],
                    show_in_sub_menu: false,
                },
                PluginConfig {
                    display_name: "Plugin 2".to_string(),
                    command: "some-random-command".to_string(),
                    args: vec![],
                    show_in_sub_menu: false,
                },
            ],
        };

        let e = new(&c);

        let result = e.execute().await;

        assert_eq!(2, result.len());

        let expected_plugin_1_result = ExecutionOutput {
            stdout: String::from("plugin-1"),
            stderr: String::new(),
        };
        let plugin_1_result = find_output_by_plugin(&result, "Plugin 1".to_string());
        assert!(plugin_1_result.is_some());
        assert_eq!(
            &expected_plugin_1_result,
            plugin_1_result.unwrap().as_ref().unwrap()
        );

        let plugin_2_result = find_output_by_plugin(&result, "Plugin 2".to_string());
        assert!(plugin_2_result.is_some());
        assert!(plugin_2_result.unwrap().is_err());
    }

    fn find_output_by_plugin(
        result: &Vec<ExecutionResult>,
        plugin_name: String,
    ) -> Option<&Result<ExecutionOutput, CliError>> {
        for r in result {
            if r.plugin == plugin_name {
                return Some(&r.result);
            }
        }

        None
    }
}
