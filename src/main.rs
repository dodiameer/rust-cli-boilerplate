#[macro_use]
extern crate slog;
extern crate slog_scope;

mod logging;
mod args;
mod config;
use crate::config::AppConfig;
use args::Args;
use structopt::StructOpt;
use anyhow::{Context, Result, Error};

fn cause_error(reason: &'static str) -> Result<()> {
    Err(Error::msg(reason))
}

fn main() -> Result<()> {
    // Setup logging
    let global_logger = logging::setup().context("Unable to setup logging")?;
    let _logger_guard = slog_scope::set_global_logger(global_logger);
    
    // Setup args and config
    let args = Args::from_args(); 
    debug!(slog_scope::logger(), "{}", args);
    let config = AppConfig::new().context("Can't read config")?;
    debug!(slog_scope::logger(), "{}", config);
    

    // Test error logging
    cause_error("Error caused to check out error handling").context("`cause_error` called in main function")?;
    Ok(())
}
