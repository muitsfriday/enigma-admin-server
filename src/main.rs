use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{error::Error as MongoError, options::ClientOptions, Client, Collection, Database};
use std::env;
use std::result::Result;

mod handler;
mod service;

type Experiment = service::Experiment;

/// Init mongo db from env string.
/// Required env: MONGO_URL string use to connect to mongo database.
async fn init_mongo() -> Result<Database, MongoError> {
    // get env.
    let url = env::var("MONGO_URL").expect("MONGO_URL is not found in env");
    let db_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME is not found in env");

    // init mongo db
    let opts = ClientOptions::parse(url).await?;
    let client = Client::with_options(opts)?;
    let db = client.database(db_name.as_str());

    Result::Ok(db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init dotenv
    dotenv().ok();

    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // init server
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not found in env");
    let mongo_coll_experiment =
        env::var("MONGO_COLLECTION_EXPERIMENT").expect("DATABASE_URL is not found in env");

    let db = init_mongo()
        .await
        .expect("unable to initiate mongo database instance");
    let experiment_coll = db.collection::<Experiment>(&mongo_coll_experiment);

    // Register a server.
    HttpServer::new(move || {
        App::new()
            .app_data(experiment_coll.clone())
            .configure(handler::register)
            .wrap(middleware::Logger::default())
    })
    .bind(server_url)?
    .run()
    .await
}
