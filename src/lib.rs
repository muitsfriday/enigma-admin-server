extern crate env_logger;

use log::info;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

pub mod auth;
mod handler;
mod middleware;
pub mod mongo;
pub mod service;

use crate::middleware::Jwt;
use crate::service::experiment::Repo as ExperimentRepo;

#[derive(Clone)]
pub struct Config {
    pub url: String,

    pub mongo_url: String,
    pub mongo_dbname: String,
    pub mongo_expr_collname: String,

    pub jwt_secret: String,

    pub experiment_item_per_page: usize,
}

#[derive(Clone)]
pub struct AppData {
    pub experiment_item_per_page: usize,
}

pub struct Dependency<R: ExperimentRepo + Send + Sync + Clone + 'static> {
    pub experiment_repo: R,
}

/// Function `run` is use to start a http server.
pub async fn run<R: ExperimentRepo + Send + Sync + Clone + 'static>(
    config: Config,
    dep: Dependency<R>,
) -> std::io::Result<()> {
    let experiment_repo = web::Data::new(dep.experiment_repo);
    let jwt_secret = config.jwt_secret;
    let app_data = web::Data::new(AppData {
        experiment_item_per_page: config.experiment_item_per_page,
    });

    info!("Starting the HTTP server");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Jwt::new(jwt_secret.clone()))
            .app_data(experiment_repo.clone())
            .app_data(app_data.clone())
            .configure(register_handler::<R>)
    })
    .bind(&config.url)?
    .run()
    .await
}

/// Register route
pub fn register_handler<R: ExperimentRepo + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/experiment").route(web::post().to(handler::experiment_create::handle::<R>)),
    )
    .service(
        web::resource("/experiment/{id}")
            .route(web::get().to(handler::experiment_get::handle::<R>)),
    )
    .service(
        web::resource("/experiments").route(web::get().to(handler::experiment_list::handle::<R>)),
    );
}
