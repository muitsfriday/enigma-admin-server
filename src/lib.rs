use actix_web::{web, App, HttpServer};

mod handler;
mod middleware;
pub mod repository;
pub mod service;

use handler::Claims;
use middleware::auth as auth_middleware;
use service::experiment as experiment_service;

pub struct ServerConfig {
    pub jwt_secret: String,
}

pub struct Dependency<ExpStore>
where
    ExpStore: experiment_service::Store,
{
    pub experiment_repo: ExpStore,
}

pub async fn init_server<ExpStore>(
    port: u16,
    conf: ServerConfig,
    dep: Dependency<ExpStore>,
) -> std::io::Result<()>
where
    ExpStore: experiment_service::Store + Send + Sync + 'static,
{
    let dependency = web::Data::new(dep);

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/experiment")
                    .app_data(web::JsonConfig::default().error_handler(handler::handle_json_error))
                    .app_data(dependency.clone())
                    .wrap(auth_middleware::JwtExtractor::new(
                        conf.jwt_secret.clone(),
                        Claims::default(),
                    ))
                    .route(web::post().to(handler::experiment_create::handle::<ExpStore>)),
            )
            .service(
                web::resource("/experiments")
                    .app_data(web::JsonConfig::default().error_handler(handler::handle_json_error))
                    .app_data(dependency.clone())
                    .wrap(auth_middleware::JwtExtractor::new(
                        conf.jwt_secret.clone(),
                        Claims::default(),
                    ))
                    .route(web::get().to(handler::experiment_list::handle::<ExpStore>)),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
