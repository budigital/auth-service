use derive_more::Display;
use dotenvy::dotenv;
use std::env;

pub struct Env {
    pub host_url: String,
    pub host_port: u128,
}

#[derive(Display)]
pub enum EnvError {
    #[display(".env file not found")]
    NoEnvFile,
    #[display("variable for {} not found", _0)]
    NoEnvVar(String),
    #[display("invalid variable type for {}", _0)]
    InvalidEnvVarType(String),
}

impl Env {
    pub fn from_env() -> Result<Self, EnvError> {
        if dotenv().is_err() {
            return Err(EnvError::NoEnvFile);
        }

        let host_url =
            env::var("HOST_URL").map_err(|_| EnvError::NoEnvVar("HOST_URL".to_string()))?;
        let host_port = env::var("HOST_PORT")
            .map_err(|_| EnvError::NoEnvVar("HOST_PORT".to_string()))?
            .parse::<u128>()
            .map_err(|_| EnvError::InvalidEnvVarType("HOST_PORT".to_string()))?;

        Ok(Env {
            host_url,
            host_port,
        })
    }
}
