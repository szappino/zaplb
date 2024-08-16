use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Target {
    pub address: String,
    pub port: u16,
    pub health_check_endpoint : String,
}

impl Target {
    pub fn new(address: String, port: u16, health_check_endpoint : String) -> Self {
        Target { address, port, health_check_endpoint }
    }
}