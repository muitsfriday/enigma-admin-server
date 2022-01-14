use dotenv::dotenv;
use std::env;

mod mongo;
mod service;

use enigma_server::*;

pub struct ServerConfig {
    pub url: String,
    pub mongo_url: String,
    pub mongo_dbname: String,
    pub mongo_expr_collname: String,
}

fn init_server_config() -> ServerConfig {
    ServerConfig {
        url: env::var("SERVER_URL").expect("SERVER_URL is not found in env"),
        mongo_url: env::var("MONGO_URL").expect("MONGO_URL is not found in env"),
        mongo_dbname: env::var("MONGO_DBNAME").expect("MONGO_DBNAME is not found in env"),
        mongo_expr_collname: env::var("MONGO_COLLECTION_EXPERIMENT")
            .expect("DATABASE_URL is not found in env"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init dotenv
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server_config = init_server_config();
    // init mongo db
    let mut mongodb = mongo::MongoDB::new(&server_config.mongo_url, &server_config.mongo_dbname);
    let _ = mongodb.connect().await;

    let experiment_coll = mongodb
        .collection::<ExperimentMongoDocument>(&server_config.mongo_expr_collname)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "{} {}",
                "Cannot initiate experiment repo (mongo)",
                err.to_string()
            );
        });

    let experiment_repo = ExperimentMongoRepo::new(experiment_coll);
    run(&server_config.url, experiment_repo).await
}
