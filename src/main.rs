use charuster::config;
use charuster::generator;

fn main() {
    let config = config::parse_local_config();
    generator::generate_charusters(Some(config));
    println!("*** Charusters generated!!!")
}
