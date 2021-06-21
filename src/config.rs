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
        let mut ac = Config::default();
        ac.merge(config::File::with_name("Settings")).context("Unable to read `Settings.toml`").unwrap();
        ac.merge(Environment::with_prefix("APP"))?;

        ac.try_into()
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "AppConfig.name: {name}", name = self.name)
    }
}
