use dotenv::dotenv;
use mongodb::Collection;
use std::env;

use enigma_server::mongo::MongoDB;
use enigma_server::run;
use enigma_server::service::repo::mongo::Document as ExperimentMongoDocument;
use enigma_server::service::repo::mongo::ExperimentMongoRepo;
use enigma_server::ServerConfig;

fn init_server_config() -> ServerConfig {
    ServerConfig {
        url: env::var("SERVER_URL").expect("SERVER_URL is not found in env"),
        mongo_url: env::var("MONGO_URL").expect("MONGO_URL is not found in env"),
        mongo_dbname: env::var("MONGO_DBNAME").expect("MONGO_DBNAME is not found in env"),
        mongo_expr_collname: env::var("MONGO_COLLECTION_EXPERIMENT")
            .expect("DATABASE_URL is not found in env"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is not found in env"),
    }
}

async fn init_mongo(server_config: &ServerConfig) -> Collection<ExperimentMongoDocument> {
    let mut mongodb = MongoDB::new(&server_config.mongo_url, &server_config.mongo_dbname);
    let _ = mongodb.connect().await;
    mongodb
        .collection::<ExperimentMongoDocument>(&server_config.mongo_expr_collname)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "{} {}",
                "Cannot initiate experiment repo (mongo)",
                err.to_string()
            );
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init dotenv
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server_config = init_server_config();
    let experiment_collection = init_mongo(&server_config).await;
    let experiment_repo = ExperimentMongoRepo::new(experiment_collection);

    run(server_config, experiment_repo).await
}
