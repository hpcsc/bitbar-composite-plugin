use bitbar_composite_plugin::{config, exec, output};
use std::error::Error;
use std::{env, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_path = config_file_path().expect("failed to get config file path");
    let config = config::from_file(config_path.as_str())
        .unwrap_or_else(|_| panic!("failed to read config file {}", config_path));

    let executor = exec::new(&config);
    let results = executor.execute().await;

    let formatter = output::new(&config);
    let formatted = formatter.format(results);
    println!("{}", formatted.join("\n"));

    Ok(())
}

fn config_file_path() -> Result<String, io::Error> {
    let mut config_path = env::current_exe()?;
    config_path.pop();
    config_path.push(".bitbar-composite-plugin.yaml");
    Ok(config_path.as_path().display().to_string())
}
