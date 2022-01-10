use mongodb::{options::ClientOptions, Client, Collection, Database};
use std::error::Error;

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

    pub async fn connect(&mut self) -> Result<&Database, Box<dyn Error>> {
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

impl Clone for MongoDB {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            dbname: self.dbname.clone(),
            db_instance: self.db_instance.clone(),
        }
    }
}
