mod app;
mod config;

use actix_web::{middleware, web, App, HttpServer};
use app::root;
use config::env;
use std::{io, process};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = match env::Env::from_env() {
        Ok(config) => {
            println!("Configuration file loaded successfully");
            config
        }
        Err(err) => {
            println!("Error when loading configuration file: {}", err);
            process::exit(1);
        }
    };

    println!(
        "Development server running at: {}:{}",
        config.host_url, config.host_port
    );

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(web::scope("/auth").route("", web::get().to(root::get)))
    })
    .bind((config.host_url, config.host_port))?
    .run()
    .await
}
