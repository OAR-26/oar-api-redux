use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub database: DatabaseConfig,
}

impl ApplicationConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = Config::builder();

        // Start off by merging in the "default" configuration file
        cfg = cfg.add_source(File::with_name("config/default"));

        // Add in the current environment file
        // Default to 'development' env
        let env = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        cfg = cfg.add_source(File::with_name(&format!("config/{}", env)).required(false));

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        cfg = cfg.add_source(File::with_name("config/local").required(false));

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        cfg = cfg.add_source(Environment::with_prefix("app"));

        let config = cfg.build()?;
        config.try_deserialize()
    }
}
