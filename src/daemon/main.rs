#![warn(
    clippy::perf,
    clippy::pedantic,
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity
)]
#![allow(unused_imports, dead_code)]
mod error;
mod gpu;
mod link;
#[path = "../message.rs"]
mod message;
#[path = "../network.rs"]
mod network;
mod parse_input;
mod start_daemon;
use clap::Parser;
pub use error::Error;
use log::{debug, error, info, log_enabled, Level};
pub use start_daemon::Monitor;
mod command;

fn main() -> Result<(), Error> {
    let args = parse_input::Args::parse();
    if init_logger(args.log_level).is_none() {
        eprintln!("failed to initalize logger");
    }
    if let Some(network) = args.network.clone() {
        info!("tracking {network}");
    }
    if let Some(gpu) = args.gpu.clone() {
        info!("tracking {gpu}");
    }
    start_daemon::start_daemon(args)
}

fn init_logger(log_level: simplelog::LevelFilter) -> Option<()> {
    use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};
    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or_else(|_| SimpleLogger::init(log_level, Config::default()))
    .ok()
}
