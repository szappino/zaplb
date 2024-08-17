use clap::Parser;
use std::fs::File;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use std::io::ErrorKind;
use simple_logger::SimpleLogger;
use crate::config::Config;
use crate::health_check::run_health_check;

pub mod config;
pub mod target;
pub mod zap_lb;
mod health_check;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the configuration file relative to the LB launch location
    #[arg(short, long, default_value = "config.json")]
    config_path: String,
}

pub async fn init_load_balancer() -> Result<()> {

    SimpleLogger::new().init().unwrap();

    let args = Args::parse();
    let config_path =  args.config_path;

    let config_file = match File::open(config_path) {
        Ok(file) => file,
        Err(e) => {
            return if e.kind() == ErrorKind::NotFound {
                eprintln!("Config file not found: {}", e);
                Err(anyhow::anyhow!("Config file not found"))
            } else {
                Err(e.into())
            }
        }
    };
    let config: Arc<Mutex<Config>> = Config::new(config_file)?;

    let cfg = Arc::clone(&config);

    // TODO: run this on it's own thread every n minutes(?)
    let _ = run_health_check(cfg.clone()).await;

    let zap_lb = zap_lb::new(cfg.clone());

    zap_lb.run().await
}