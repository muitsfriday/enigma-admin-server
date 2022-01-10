use actix_web::{web, App, HttpServer};

mod handler;
mod mongo;
mod service;

use service::repo::{ExperimentMongoDocument, ExperimentMongoRepo, ExperimentRepo};

pub struct ServerConfig {
    pub url: String,
    pub mongo_url: String,
    pub mongo_dbname: String,
    pub mongo_expr_collname: String,
}

pub struct Server<'a> {
    conf: &'a ServerConfig,
}

impl<'a> Server<'a> {
    pub fn new(conf: &'a ServerConfig) -> Self {
        Server { conf }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        // init mongo db
        let mut mongodb = mongo::MongoDB::new(&self.conf.mongo_url, &self.conf.mongo_dbname);
        let _ = mongodb.connect().await;

        let experiment_coll = mongodb
            .collection::<ExperimentMongoDocument>(&self.conf.mongo_expr_collname)
            .await
            .unwrap_or_else(|err| {
                panic!(
                    "{} {}",
                    "Cannot initiate experiment repo (mongo)",
                    err.to_string()
                );
            });

        let experiment_repo = ExperimentMongoRepo::new(experiment_coll);

        println!("{}", &self.conf.mongo_url);

        // init http server
        HttpServer::new(move || {
            App::new()
                .app_data(experiment_repo.clone())
                .configure(register_handler::<ExperimentMongoRepo>)
        })
        .bind(&self.conf.url)?
        .run()
        .await
    }
}

pub fn register_handler<T: ExperimentRepo + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/experiment").route(web::post().to(handler::experiment_create::handle::<T>)),
    );
}
