use bitbar_composite_plugin::{config, exec};

fn main() {
    let result = config::from_file("./testdata/config.yaml")
        .map(exec::new)
        .and_then(|e| e.execute());

    match result {
        Ok(out) => println!("{}", out),
        Err(e) => println!("failed with error {:?}", e)
    }
}
