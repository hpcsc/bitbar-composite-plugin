use crate::config;
use crate::exec::ExecutionResult;

pub struct Formatter<'a> {
    config: &'a config::Config
}

pub fn new(config: &config::Config) -> Formatter {
    Formatter { config }
}

impl Formatter<'_> {
    pub fn format(&self, execution_results: Vec<ExecutionResult>) -> Vec<String> {
        let mut output = vec!();

        output.push("Bit | color=orange".to_string());

        for plugin in &self.config.plugins {
            let plugin_execution_result = execution_results
                .iter()
                .find(|r| r.plugin == plugin.display_name)
                .unwrap();

            let formatted = match &plugin_execution_result.result {
                Ok(output) => format!("---
{} | color=green
---
{}{}", &plugin_execution_result.plugin, output.stdout, output.stderr),
                Err(e) => format!("---
{} | color=green
Error: {:?} | color=red", &plugin_execution_result.plugin, e)
            };
            output.push(formatted);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, PluginConfig};
    use crate::exec;

    #[test]
    fn format_returns_result_in_order_of_plugins_in_config() {
        let c = Config {
            plugins: vec![PluginConfig {
                display_name: "Plugin 1".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-1'".to_string()],
            }, PluginConfig {
                display_name: "Plugin 3".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-3'".to_string()],
            }, PluginConfig {
                display_name: "Plugin 2".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-2'".to_string()],
            }]
        };

        let e = new(&c);

        let execution_results = vec!(
            exec::new_execution_result_with_output("Plugin 3", "plugin-3", ""),
            exec::new_execution_result_with_output("Plugin 2", "plugin-2", ""),
            exec::new_execution_result_with_output("Plugin 1", "plugin-1", "")
        );

        let formatted = e.format(execution_results);

        let expected = "Bit | color=orange
---
Plugin 1 | color=green
---
plugin-1
---
Plugin 3 | color=green
---
plugin-3
---
Plugin 2 | color=green
---
plugin-2";

        assert_eq!(expected, formatted.join("\n"))
    }
}