extern crate config;
use anyhow::{Result, Context};
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    name: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let logger = slog_scope::logger().new(o!("mod" => "config.rs", "loc" => "AppConfig::new()"));
        debug!(logger, "Creating config");
        let mut ac = Config::default();
        debug!(logger, "Merging `Settings.toml`");
        ac.merge(config::File::with_name("Settings")).context("Unable to read `Settings.toml`").unwrap();
        debug!(logger, "Merging `APP_*` environment vars");
        ac.merge(Environment::with_prefix("APP"))?;

        ac.try_into()
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "AppConfig.name: {name}", name = self.name)
    }
}
