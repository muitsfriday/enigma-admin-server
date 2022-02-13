use actix_web::{web, App, HttpServer};

mod handler;
pub mod mongo;
pub mod service;

use crate::service::repo;

pub async fn run<T: repo::ExperimentRepo + Send + Sync + Clone + 'static>(
    url: &str,
    experiment_repo: T,
) -> std::io::Result<()> {
    let experiment_repo = web::Data::new(experiment_repo);

    // init http server
    HttpServer::new(move || {
        App::new()
            .app_data(experiment_repo.clone())
            .configure(register_handler::<T>)
    })
    .bind(&url)?
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
