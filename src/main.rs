use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

use enigma_server::*;
mod handler;   handler.rs , /handler/mod.rs
mod service;

type Experiment = service::Experiment;

fn init_server_config() -> ServerConfig {
    ServerConfig {
        url: env::var("SERVER_URL").expect("SERVER_URL is not found in env"),
        mongo_url: env::var("MONGO_URL").expect("MONGO_URL is not found in env"),
        mongo_dbname: env::var("MONGO_DBNAME").expect("MONGO_DBNAME is not found in env"),
        mongo_expr_collname: env::var("MONGO_COLLECTION_EXPERIMENT")
            .expect("DATABASE_URL is not found in env"),
    }
}

pub struct ServerConfig {
    pub url: String,
    pub mongo_url: String,
    pub mongo_dbname: String,
    pub mongo_expr_collname: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init dotenv
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server_config = init_server_config();

    // init server
    HttpServer::new(move || {
        App::new()
            .configure(handler::register)
            .wrap(middleware::Logger::default())
    })
    .bind(server_config.url)?
    .run()
    .await
}
