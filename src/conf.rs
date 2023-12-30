use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub server: Server,
    pub auth: Auth,
}

pub fn init() -> Result<Conf, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let builder = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("letters"))
        .build()?;

    builder.try_deserialize()
}
