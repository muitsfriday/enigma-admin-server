use dotenv::dotenv;
use std::env;

use enigma_server::*;

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
    let server = Server::new(&server_config);

    server.run().await
}
