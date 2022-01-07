use actix_web::web;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use std::error::Error;

mod handler;
mod service;

pub struct MongoDB {
    url: String,
    dbname: String,
    db_instance: Option<Database>,
}

impl MongoDB {
    pub fn new(url: &str, dbname: &str) -> Self {
        MongoDB {
            url: url.to_owned(),
            dbname: dbname.to_owned(),
            db_instance: None,
        }
    }

    async fn connect(&mut self) -> Result<&Database, Box<dyn Error>> {
        if let Some(ref db) = self.db_instance {
            return Result::Ok(db);
        }

        let opts = ClientOptions::parse(&self.url).await?;
        let client = Client::with_options(opts)?;
        self.db_instance = Some(client.database(&self.dbname));

        match self.db_instance {
            Some(ref db) => Result::Ok(db),
            None => Result::Err("cannot connect database".into()),
        }
    }

    pub async fn collection<T>(&mut self, colname: &str) -> Result<Collection<T>, Box<dyn Error>> {
        let db = self.connect().await?;
        let col = db.collection::<T>(&colname);

        Result::Ok(col)
    }
}

pub fn register_handler<'a>(cfg: &'a mut web::ServiceConfig) -> impl FnMut() + 'a {
    move || {
        cfg.service(
            web::resource("/experiment").route(web::post().to(handler::experiment_create::handle)),
        );
    }
}
