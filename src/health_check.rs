use reqwest::Client;
use std::sync::{Arc, Mutex};
use anyhow::{Result};

use crate::target::Target;
use crate::config::Config;

pub async fn run_health_check(config: Arc<Mutex<Config>>) -> Result<(), String> {

    log::info!("Running health check.");

    let mut cfg = config.lock().unwrap();
    let targets = &mut cfg.targets;

    if targets.is_empty() {

        return Err("No targets defined, please check configuration".to_owned());
    }

    let client = Client::new();

    // Cannot do this:
    // retain works in a synchronous context cannot await for health_check_target
    // targets.retain(|target| health_check_target(target, &client).await);

    // Also cannot do this:
    // because of double mutable borrow(?)
    // for (index, target) in targets.into_iter().enumerate() {
    //     match health_check_target(&target, &client).await {
    //         true => continue,
    //         false => targets.remove(index),
    //         _ => continue,
    //     };
    // }

    // So for now im doing this indecency:
    // I'm sure this is not how you do it !!GO INTO DETAIL!!
    let mut indices_to_remove = Vec::new();

    let mut checks = Vec::new();
    for (index, target) in targets.iter().enumerate() {
        let target_clone = target.clone();
        let client_clone = client.clone();

        let check = async move {
            let result = health_check_target(&target_clone, &client_clone).await;
            (index, result)
        };
        checks.push(check);
    }

    for check in checks {
        let (index, should_remove) = check.await;

        if !should_remove {
            indices_to_remove.push(index);
        }
    }

    for index in indices_to_remove.into_iter().rev() {
        log::info!("Removing target {index}");
        targets.remove(index);
    }

    Ok(())
}

async fn health_check_target(target: &Target, client: &Client) -> bool {
    let url = format!("http://{}:{}{}", target.address, target.port, target.health_check_endpoint);

    log::info!("Checking url: {url}");

    match client.get(url).send().await {
        Ok(response) => {
            let status = response.status();

            match status {
                s if s.is_success() => {
                    let addr = &target.address;
                    log::info!("{addr} is ok");
                    true
                },
                s if s.is_client_error() => {
                    log::error!("Error during health check: Client Error {s}");
                    false
                }
                s if s.is_server_error() => {
                    log::error!("Error during health check: Server Error {s}");
                    false
                }
                _ => {
                    log::error!("Error during health check: Unhandled Error");
                    false
                }
            }
        }
        Err(err) => {
            log::error!("!Error! during health check: {err}");
            false
        }

    }
}

