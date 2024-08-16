use std::fs::File;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use crate::config::Config;

// TODO: move to another file
pub mod config;
pub mod target;
pub mod zap_lb;


#[tokio::main]
async fn main() -> Result<()> {

    let config_file: File = File::open("config.json")?;
    let config: Arc<Mutex<Config>> = Config::new(config_file)?;

    let zap_lb = zap_lb::new(config);
    zap_lb.run().await
}
