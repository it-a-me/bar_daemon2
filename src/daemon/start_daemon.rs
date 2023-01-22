use crate::error::{self, Error};
use crate::message::Message;
use crate::parse_input::Args;
use log::{info, trace, warn};
use std::collections::HashMap;
use tokio::sync::Mutex;
pub fn start_daemon(args: Args) -> Result<(), Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .enable_all()
        .build()
        .map_err(error::Network::Runtime)?
        .block_on(daemon(args))?;
    Ok(())
}

pub async fn daemon(args: Args) -> Result<(), Error> {
    use crate::network::LOCALHOST;
    trace!("runtime initalized");
    let hashmap = init_monitor_map(&args);
    let host = format!("{LOCALHOST}:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .map_err(|e| error::Network::Bind(host.clone(), e))?;
    trace!("daemon bount to host '{}'", &host);
    loop {
        if let Ok(connection) =
            tokio::time::timeout(std::time::Duration::from_secs(5), listener.accept()).await
        {
            use serde::Deserialize;
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            trace!("connection requested");
            let (mut stream, addr) = connection.map_err(error::Network::Accept)?;
            trace!("request accepted from {addr}");
            let mut buffer = Vec::new();
            let bytes_recived = stream
                .read_to_end(&mut buffer)
                .await
                .map_err(error::Network::Read)?;
            trace!("read {bytes_recived} from {addr}");
            if let Ok(message) = bincode::deserialize::<crate::message::Message>(&buffer) {
                trace!("message deserialized.  Connection is requesting {message:?}");
                match hashmap.get(&message) {
                    Some(monitor) => {
                        let mut lock = monitor.lock().await;
                        trace!("mutex unlocked");
                        match lock.query().await {
                            Ok(v) => {
                                stream
                                    .write(v.as_bytes())
                                    .await
                                    .map_err(error::Network::Write)?;
                            }
                            Err(e) => {
                                eprintln!("{e}");
                                warn!("query failed");
                            }
                        };
                    }
                    None => {
                        info!("no {message:?} configured for daemon");
                    }
                }
            } else {
                info!("invalid date format recived, ignoring");
            }
        }
        trace!("loop");
    }
}

fn init_monitor_map(args: &Args) -> HashMap<Message, Box<Mutex<dyn Monitor>>> {
    use crate::message;
    let mut hashmap: HashMap<Message, Box<Mutex<dyn Monitor>>> = HashMap::new();
    if let Some(gpu) = args.gpu.clone() {
        hashmap.insert(
            Message::Gpu,
            Box::new(Mutex::new(crate::gpu::RocmSmi::new(&gpu))),
        );
    }
    if let Some(network) = args.network.clone() {
        hashmap.insert(
            Message::Network(message::NetworkDirection::Up),
            Box::new(Mutex::new(crate::link::Link::new(&network, message::NetworkDirection::Up))),
        );
        hashmap.insert(
            Message::Network(message::NetworkDirection::Down),
            Box::new(Mutex::new(crate::link::Link::new(&network, message::NetworkDirection::Down))),
        );
    }
    hashmap
}

#[async_trait::async_trait]
pub trait Monitor {
    async fn query(&mut self) -> Result<String, Error>;
    async fn update(&mut self) -> Result<(), Error>;
}
