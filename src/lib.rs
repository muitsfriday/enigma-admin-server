use actix_web::{web, App, HttpServer};

pub mod auth;
mod handler;
mod middleware;
pub mod mongo;
pub mod service;

use crate::middleware::Jwt;
use crate::service::repo;

#[derive(Clone)]
pub struct ServerConfig {
    pub url: String,
    pub mongo_url: String,
    pub mongo_dbname: String,
    pub mongo_expr_collname: String,
    pub jwt_secret: String,
}

pub async fn run<T: repo::ExperimentRepo + Send + Sync + Clone + 'static>(
    config: ServerConfig,
    experiment_repo: T,
) -> std::io::Result<()> {
    let experiment_repo = web::Data::new(experiment_repo);

    // init http server
    HttpServer::new(move || {
        App::new()
            .wrap(Jwt::new(&config.jwt_secret.clone()))
            .app_data(experiment_repo.clone())
            .configure(register_handler::<T>)
    })
    .bind(&config.url)?
    .run()
    .await
}

pub fn register_handler<T: repo::ExperimentRepo + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/experiment").route(web::post().to(handler::experiment_create::handle::<T>)),
    )
    .service(
        web::resource("/experiment/{id}")
            .route(web::get().to(handler::experiment_get::handle::<T>)),
    );
}
