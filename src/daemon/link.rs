const MINIMUMLENGTH: usize = 8;
pub fn set_min_len(len: usize, input: &str) -> String {
    let mut input = String::from(input);
    while input.len() < len {
        input.insert(0, ' ');
    }
    input
}

use crate::command::Command;
use crate::error::{self, Error};
use crate::message::NetworkDirection;
use async_trait::async_trait;
use log::warn;
use tokio::{io, time};
pub struct Link {
    network_interface: String,
    network_direction: NetworkDirection,
    last_checked: time::Instant,
    up_path: std::path::PathBuf,
    last_bytes: Option<usize>,
    bytes_path: std::path::PathBuf,
}
impl Link {
    pub fn new(network_interface: &str, network_direction: NetworkDirection) -> Self {
        use std::path::PathBuf;
        let bytes_path = PathBuf::from(format!("/sys/class/net/{network_interface}/statistics")).join(
            match network_direction {
                NetworkDirection::Up => "rx_bytes",
                NetworkDirection::Down => "tx_bytes",
            },
        );
        let up_path = PathBuf::from(format!("/sys/class/net/{network_interface}/operstate"));
        Self {
            network_interface: network_interface.to_string(),
            network_direction,
            bytes_path,
            last_checked: time::Instant::now(),
            up_path,
            last_bytes: None,
        }
    }
    async fn get_bytes(&self) -> Result<usize, Error> {
        let bytes = tokio::fs::read_to_string(&self.bytes_path)
            .await
            .map_err(|e| error::Fs::Read(self.bytes_path.clone(), e))?;
        bytes.trim()
            .parse::<usize>()
            .map_err(|_| Error::ParseNumber(bytes))
    }
    async fn get_bps(&self) -> Result<String, Error> {
        if let Some(last_bytes) = self.last_bytes {
            let bytes = self.get_bytes().await?;
            let bps = (bytes - last_bytes) * 1000
                / (self.last_checked.elapsed().as_millis() + 1) as usize;
            Ok(Self::humanify(bps))
        } else {
            Ok(Self::humanify(0))
        }
    }

    fn humanify(bytes: usize) -> String {
        match bytes {
            b @ 0..=2000 => set_min_len(MINIMUMLENGTH, &format!("{b}B/s")),
            k @ 2001..=2_000_000 => set_min_len(MINIMUMLENGTH, &format!("{}KiB/s", k / 1000)),
            m => set_min_len(MINIMUMLENGTH, &format!("{}MiB/s", m / 1_000_000)),
        }
    }
}
#[derive(PartialEq)]
enum LinkState {
    Up,
    Down,
}
impl LinkState {
    fn from_str(s: &str) -> Option<LinkState> {
        match s.to_lowercase().trim() {
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            _ => None,
        }
    }
}

#[async_trait]
impl crate::Monitor for Link {
    async fn query(&mut self) -> Result<String, Error> {
        let operstate = LinkState::from_str(
            &tokio::fs::read_to_string(&self.up_path)
                .await
                .map_err(|e| error::Fs::Read(self.up_path.clone(), e))?,
        );
        let result = match operstate {
            Some(LinkState::Up) => self.get_bps().await,
            Some(LinkState::Down) => return Ok(set_min_len(MINIMUMLENGTH, "Down")),
            None => {
                warn!("invalid operstate, acting as if interface is UP");
                self.get_bps().await
            }
        };
        self.update().await?;
        result
    }
    async fn update(&mut self) -> Result<(), Error> {
        let bytes = self.get_bytes().await?;
        self.last_bytes = Some(bytes);
        self.last_checked = time::Instant::now();
        Ok(())
    }
}
