use clap::Parser;
use std::fs::File;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use std::io::ErrorKind;

use crate::config::Config;

pub mod config;
pub mod target;
pub mod zap_lb;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the configuration file relative to the LB launch location
    #[arg(short, long, default_value = "config.json")]
    config_path: String,
}

pub async fn init_load_balancer() -> Result<()> {

    let args = Args::parse();
    let config_path =  args.config_path;

    let config_file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => {
            return if e.kind() == ErrorKind::NotFound {
                eprintln!("Config file not found: {}", e);
                Err(anyhow::anyhow!("Config file not found")) // Return an error
            } else {
                Err(e.into()) // Propagate other errors
            }
        }
    };

    let config: Arc<Mutex<Config>> = Config::new(config_file)?;

    let zap_lb = zap_lb::new(config);

    zap_lb.run().await
}