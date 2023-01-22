use clap::Parser;

/// Daemon to that allows easy querying for common requests
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///the verbosity of the log {Off, Error, Warn, Info, Debug, Trace}
    #[arg(short, long, default_value_t = simplelog::LevelFilter::Error)]
    pub log_level: simplelog::LevelFilter,
    ///the port to bind the daemon to
    #[arg(short, long, default_value_t = String::from(crate::network::DEFAULTPORT))]
    pub port: String,
    /// name of network interface to be monitored
    #[arg(short, long)]
    pub network: Option<String>,
    /// name of gpu interface to be monitored
    #[arg(short, long)]
    pub gpu: Option<String>,
}
