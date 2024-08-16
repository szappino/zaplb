use std::fs::File;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::target::Target;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub targets: Vec<Target>,
}

impl Config {
    pub fn new(config_file: File) -> Result<Arc<Mutex<Config>>> {
        let conf : Config = serde_json::from_reader(config_file)?;
        Ok(Arc::new(Mutex::new(conf)))
    }
}
