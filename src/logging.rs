extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate slog_scope;

use slog::Drain;
use anyhow::Result;

pub fn setup() -> Result<slog::Logger>{
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Ok(slog::Logger::root(drain, o!()))
}
