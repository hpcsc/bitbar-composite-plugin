use bitbar_composite_plugin::config;
use bitbar_composite_plugin::config::{Config, PluginConfig};

#[test]
fn from_file_returns_deserialized_configs_when_path_is_valid() {
    let result = config::from_file("./.bitbar-composite-plugin.yaml");

    assert!(result.is_ok());
    assert_eq!(Config {
        plugins: vec![PluginConfig {
            display_name: "Plugin 1".to_string(),
            command: "bash".to_string(),
            args: vec!["-c".to_string(), "echo -n plugin-1".to_string()]
        }, PluginConfig {
            display_name: "Plugin 2".to_string(),
            command: "bash".to_string(),
            args: vec!["-c".to_string(), "echo -n plugin-2".to_string()]
        }]
    }, result.unwrap())
}

#[test]
fn from_file_returns_error_when_path_not_exists() {
    let result = config::from_file("not-existing-file");

    assert!(result.is_err());
}
