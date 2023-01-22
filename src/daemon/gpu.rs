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
            fetch_cmd: Command::new("rocm-smi", &["-a", "--json"]),
        }
    }
}

#[async_trait]
impl crate::Monitor for RocmSmi {
    async fn query(&mut self) -> Result<String, Error> {
        let smi_json = self.fetch_cmd.call_as_json().await?;
        let usage_percent = smi_json[&self.gpu_id]["GPU use (%)"]
            .as_str()
            .ok_or(Error::RocmSmi)?;
        Ok(format!("GPU   {usage_percent}"))
    }
    async fn update(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
