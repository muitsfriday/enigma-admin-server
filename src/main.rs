use anyhow::Result;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use std::env;

use enigma_server::repository::experiment as experiment_repo;
use enigma_server::service::experiment as experiment_service;
use enigma_server::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("APP_PORT")
        .expect("SERVER_URL is not found in env")
        .parse::<u16>()
        .unwrap();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not found in env");

    let experiment_coll = init_mongo_coll::<experiment_repo::Document>(
        &env::var("MONGO_URL").expect("MONGO_URL is not found in env"),
        &env::var("MONGO_DBNAME").expect("MONGO_DBNAME is not found in env"),
        &env::var("MONGO_COLLECTION_EXPERIMENT")
            .expect("MONGO_COLLECTION_EXPERIMENT is not found in env"),
    )
    .await
    .unwrap();

    let experiment_repo = init_experiment_repository(experiment_coll);

    init_server(
        port,
        ServerConfig { jwt_secret },
        Dependency { experiment_repo },
    )
    .await
}

async fn init_mongo_coll<T>(url: &str, dbname: &str, collection: &str) -> Result<Collection<T>> {
    let opts = ClientOptions::parse(url).await?;
    let client = Client::with_options(opts)?;
    let db_instance = client.database(dbname);
    let coll = db_instance.collection::<T>(collection);

    client
        .database(dbname)
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    return Ok(coll);
}

fn init_experiment_repository(
    coll: Collection<experiment_repo::Document>,
) -> impl experiment_service::Store {
    let repo = experiment_repo::Repo::new(coll);

    return repo;
}
