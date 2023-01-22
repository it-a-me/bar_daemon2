#[path = "../message.rs"]
mod message;
#[path = "../network.rs"]
mod network;
mod parse_input;
use network::LOCALHOST;

use clap::Parser;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
fn main() -> std::io::Result<()> {
    let args = parse_input::Args::parse();
    let mut socket = TcpStream::connect(format!("{LOCALHOST}:{}", args.port))?;
    let serialized = bincode::serialize(&args.query).unwrap();
    socket.write_all(&serialized)?;
    socket.shutdown(Shutdown::Write)?;

    let mut result = String::new();
    socket.read_to_string(&mut result)?;
    println!("{}", result);
    Ok(())
}
