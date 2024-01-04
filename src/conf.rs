use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Server {
    pub port: u16,
    pub log_level: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Auth {
    pub secret: String,
    pub timeout_seconds: i64,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ConfInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Conf {
    #[serde[default]]
    pub server: Server,
    #[serde[default]]
    pub database: Database,
    #[serde[default]]
    pub auth: Auth,
    #[serde[default]]
    pub info: ConfInfo,
}

impl Conf {
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        //     let builder = Config::builder()
        //         .add_source(File::with_name("config/default"))
        //         .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
        //         .add_source(File::with_name("config/local").required(false))
        //         .add_source(Environment::with_prefix("letters"))
        //         .build()?;

        let builder = Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("__")
                    .prefix("__"),
            )
            .set_override("info.location", location)?
            .set_override("info.env_prefix", env_prefix)?
            .build()?;

        Ok(builder.try_deserialize()?)
    }
}
