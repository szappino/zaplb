use anyhow::Result;
use zapLB::init_load_balancer;

#[tokio::main]
async fn main() -> Result<(), String> {

     match init_load_balancer().await {
          Ok(()) => Ok(()),
          Err(e) => Err(e.to_string()),
     }
}
