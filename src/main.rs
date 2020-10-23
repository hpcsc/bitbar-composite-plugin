use bitbar_composite_plugin::{config, exec};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_path = "./.bitbar-composite-plugin.yaml";
    let config = config::from_file(config_path)
            .expect(format!("failed to read config file {}", config_path)
            .as_str());

    let executor = exec::new(config);
    let results = executor.execute().await;
    for r in results {
        match r {
            Ok(out) => println!("{}: {:?}", out.plugin, out.output),
            Err(e) => println!("failed to execute with error {:?}", e)
        }
    }

    Ok(())
}
