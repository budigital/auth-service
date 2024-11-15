use env_logger::{init_from_env, Env};
use log::{error, info, warn};

#[allow(dead_code)]
pub enum LoggerLevel {
    Info,
    Warn,
    Error,
}

pub fn init_logger() {
    init_from_env(Env::new().default_filter_or("info"));
}

pub fn log(level: LoggerLevel, message: &str) {
    let msg = message.to_owned();

    match level {
        LoggerLevel::Info => info!("{}", msg),
        LoggerLevel::Warn => warn!("{}", msg),
        LoggerLevel::Error => error!("{}", msg),
    }
}
