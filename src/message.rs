use clap::Subcommand;
use serde::{Deserialize, Serialize};
#[derive(Subcommand, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Gpu,
    #[command(subcommand)]
    Network(NetworkDirection),
}

#[derive(clap::Subcommand, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkDirection {
    Up,
    Down,
}
