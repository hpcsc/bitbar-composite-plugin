use crate::config;
use crate::exec::ExecutionResult;
use crate::config::PluginConfig;

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

        for plugin_config in &self.config.plugins {
            let plugin_execution_result = execution_results
                .iter()
                .find(|r| r.plugin == plugin_config.display_name)
                .unwrap();

            let formatted = match &plugin_execution_result.result {
                Ok(output) => format!("---
{} | color=green
{}{}{}",
                                      &plugin_execution_result.plugin,
                                      if plugin_config.show_in_sub_menu { "" } else { "---\n" },
                                      Formatter::show_in_sub_menu_if_needed(&output.stdout, plugin_config),
                                      Formatter::show_in_sub_menu_if_needed(&output.stderr, plugin_config)),
                Err(e) => format!("---
{} | color=green
{}Error: {:?} | color=red",
                                  &plugin_execution_result.plugin,
                                  if plugin_config.show_in_sub_menu { "" } else { "---\n" },
                                  e)
            };
            output.push(formatted);
        }

        output
    }

    fn show_in_sub_menu_if_needed(input: &str, plugin_config: &PluginConfig) -> String {
        if input.is_empty() {
            return String::new()
        }

        if !plugin_config.show_in_sub_menu {
            return input.to_string()
        }

        input.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| format!("--{}", s))
            .collect::<Vec<String>>().join("\n")
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
                show_in_sub_menu: false
            }, PluginConfig {
                display_name: "Plugin 3".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-3'".to_string()],
                show_in_sub_menu: false
            }, PluginConfig {
                display_name: "Plugin 2".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-2'".to_string()],
                show_in_sub_menu: false
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

    #[test]
    fn format_returns_plugin_output_in_sub_memu_format_when_config_is_true() {
        let c = Config {
            plugins: vec![PluginConfig {
                display_name: "Plugin 1".to_string(),
                command: "bash".to_string(),
                args: vec!["-c".to_string(), "echo -n 'plugin-1'".to_string()],
                show_in_sub_menu: true
            }]
        };

        let e = new(&c);

        let execution_results = vec!(
            exec::new_execution_result_with_output("Plugin 1", "line 1\nline 2\nline 3\n", ""),
        );

        let formatted = e.format(execution_results);

        let expected = "Bit | color=orange
---
Plugin 1 | color=green
--line 1
--line 2
--line 3";

        assert_eq!(expected, formatted.join("\n"))
    }
}