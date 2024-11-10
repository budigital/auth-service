mod config;

use config::env;
use std::process;

fn main() {
    let config = match env::Env::from_env() {
        Ok(config) => {
            println!("Configuration file loaded successfully");
            config
        }
        Err(err) => {
            println!("Error when loading configuration file: {}", err);
            process::exit(1)
        }
    };

    println!(
        "Development server running at: {}:{}",
        config.host_url, config.host_port
    );

    println!("Hello, world!");
}
