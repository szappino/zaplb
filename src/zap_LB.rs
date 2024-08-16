use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use anyhow::{Result};
use simple_logger::SimpleLogger;

use crate::config::Config;
use crate::target::Target;

#[derive(Clone)]
pub struct ZapLB {
    pub config: Arc<Mutex<Config>>,
}

impl ZapLB {
    pub async fn run(&self) -> Result<()> {
        let config = self.config.clone();

        let (addr, port) = {
            let config_lock = config.lock().unwrap();
            (config_lock.address.clone(), config_lock.port)
        };

        HttpServer::new(move || {
            let client = Client::new();
            let config_data = config.clone();

            App::new()
                .app_data(web::Data::new(client))
                .app_data(web::Data::new(config_data))
                .default_service(web::route().to(handle_request))
        })
        .workers(4)
        .bind((addr, port))?
        .run()
        .await?;

        Ok(())
    }
}
async fn handle_request(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
    config: web::Data<Arc<Mutex<Config>>>,
) -> HttpResponse {

    let backend = {
        let mut config_lock = config.lock().unwrap();
        choose_backend(&mut config_lock.targets)
    };

    let url = format!("http://{}:{}{}", backend.address, backend.port, req.uri());
    log::info!("Forwarded url: {url}");

    let reqwest_method = convert_method(req.method());

    let forwarded_req = client
        .request(reqwest_method, &url)
        .body(body)
        .send()
        .await;

    match forwarded_req {

        Ok(response) => {
            let status_code = actix_web::http::StatusCode::from_u16(response.status().as_u16()).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
            let mut actix_response = HttpResponse::build(status_code);

            for (key, value) in response.headers().iter() {
                if let (Some(actix_key), Ok(actix_value)) = (
                    actix_web::http::header::HeaderName::from_bytes(key.as_str().as_bytes()).ok(),
                    actix_web::http::header::HeaderValue::from_bytes(value.as_bytes()),
                ) {
                    actix_response.insert_header((actix_key, actix_value));
                }
            }

            let body = response.bytes().await.unwrap_or_default();
            actix_response.body(body)
        }
        Err(e) => {
            log::error!("error: {e}");
            HttpResponse::InternalServerError().finish()
        },
    }
}

fn convert_method(method: &actix_web::http::Method) -> reqwest::Method {
    match method {
        &actix_web::http::Method::GET => reqwest::Method::GET,
        &actix_web::http::Method::POST => reqwest::Method::POST,
        &actix_web::http::Method::PUT => reqwest::Method::PUT,
        &actix_web::http::Method::DELETE => reqwest::Method::DELETE,
        &actix_web::http::Method::PATCH => reqwest::Method::PATCH,
        &actix_web::http::Method::HEAD => reqwest::Method::HEAD,
        &actix_web::http::Method::OPTIONS => reqwest::Method::OPTIONS,
        _ => reqwest::Method::GET, // Fallback to GET if unsupported method is found
    }
}
fn choose_backend(targets: &mut Vec<Target>) -> Target {
    let backend = targets.remove(0);
    targets.push(backend.clone());
    backend
}
pub fn new(config: Arc<Mutex<Config>>) -> ZapLB {

    SimpleLogger::new().init().unwrap();

    ZapLB { config }
}
