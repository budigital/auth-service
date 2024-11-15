mod app;
mod config;
mod utils;

use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpServer,
};
use app::{healthcheck, root};
use config::env;
use std::{io, process};
use utils::logger;

#[actix_web::main]
async fn main() -> io::Result<()> {
    logger::init_logger();

    let config = match env::Env::from_env() {
        Ok(config) => {
            logger::log(
                logger::LoggerLevel::Info,
                "Configuration file loaded successfully",
            );
            config
        }
        Err(err) => {
            let err_message = format!("Error when loading configuration file: {}", err);

            logger::log(logger::LoggerLevel::Error, &err_message);
            process::exit(1);
        }
    };

    println!(
        "Development server running at: {}:{}",
        config.host_url, config.host_port
    );

    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .service(
                web::scope("/auth")
                    .route("", web::get().to(root::get))
                    .route("/healthcheck", web::get().to(healthcheck::get)),
            )
    })
    .bind((config.host_url, config.host_port))?
    .run()
    .await
}
