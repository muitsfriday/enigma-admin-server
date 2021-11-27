use actix_web::web;

pub mod experiment_create;

/// register is a function to register all handler in the services to actix http.
pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/experiment").route(web::post().to(experiment_create::handler)));
}
