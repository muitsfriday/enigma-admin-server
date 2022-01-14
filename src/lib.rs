use actix_web::{web, App, HttpServer};

mod handler;
mod mongo;
mod service;

pub use service::repo::{ExperimentMongoDocument, ExperimentMongoRepo, ExperimentRepo};

pub async fn run<T: ExperimentRepo + Send + Sync + Clone + 'static>(
    url: &str,
    experiment_repo: T,
) -> std::io::Result<()> {
    // init http server
    HttpServer::new(move || {
        App::new()
            .app_data(experiment_repo.clone())
            .configure(register_handler::<ExperimentMongoRepo>)
    })
    .bind(&url)?
    .run()
    .await
}

pub fn register_handler<T: ExperimentRepo + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/experiment").route(web::post().to(handler::experiment_create::handle::<T>)),
    );
}
