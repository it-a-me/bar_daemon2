use crate::command::Command;
use crate::Error;
use async_trait::async_trait;
use tokio::io;
pub struct RocmSmi {
    gpu_id: String,
    fetch_cmd: Command,
}
impl RocmSmi {
    pub fn new(gpu_id: &str) -> Self {
        Self {
            gpu_id: gpu_id.to_string(),
            fetch_cmd: Command::new("rocm-smi", vec!["-a", "--json"]),
        }
    }
}

#[async_trait]
impl crate::Monitor for RocmSmi {
    async fn query(&mut self) -> Result<String, Error> {
        let output = self.fetch_cmd.call().await?;
        if let Ok(smi_json) = json::parse(output.trim()) {
            let usage_percent = smi_json["card0"]["GPU use (%)"]
                .as_u32()
                .ok_or(Error::RocmSmi)?;
            Ok(format!("GPU   {usage_percent}"))
        } else {
            Err(Error::RocmSmi)
        }
    }
    async fn update(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
