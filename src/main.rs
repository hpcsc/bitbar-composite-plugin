use bitbar_composite_plugin::config;

fn main() {
    let c = config::from_file("./testdata/config.yaml");
    println!("{:?}", c);
}
