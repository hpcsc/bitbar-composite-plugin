use bitbar_composite_plugin::{config, exec};

fn main() {
    let results = config::from_file("./testdata/config.yaml")
        .map(exec::new)
        .map(|e| e.execute());

    match results {
        Ok(execution_results) => {
            for r in execution_results {
                match r {
                    Ok(out) => println!("{}: {:?}", out.plugin, out.output),
                    Err(e) => println!("failed to execute with error {:?}", e)
                }
            }
        }
        Err(e) => println!("failed with error {:?}", e)
    }
}
