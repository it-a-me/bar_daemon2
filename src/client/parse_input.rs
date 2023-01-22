use clap::Parser;

/// Client to bar_daemon2
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from(crate::network::DEFAULTPORT))]
    pub port: String,
    #[command(subcommand)]
    pub query: crate::message::Message,
}
