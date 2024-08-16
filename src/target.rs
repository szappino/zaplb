use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Target {
    pub address: String,
    pub port: u16,
}

impl Target {
    pub fn new(address: String, port: u16) -> Self {
        Target { address, port }
    }
}