use async_trait::async_trait;
use bson::Bson;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::super::experiment::Experiment;
use super::ExperimentRepo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    id: Option<String>,
    title: String,
    description: String,
}

impl From<Experiment> for Document {
    fn from(data: Experiment) -> Self {
        Document {
            id: data.id,
            title: data.title,
            description: data.description,
        }
    }
}

pub struct ExperimentMongoRepo {
    coll: Collection<Document>,
}

impl ExperimentMongoRepo {
    pub fn new(coll: Collection<Document>) -> Self {
        ExperimentMongoRepo { coll }
    }
}

#[async_trait]
impl ExperimentRepo for ExperimentMongoRepo {
    async fn save(&self, data: Experiment) -> Result<Experiment, Box<dyn Error>> {
        let mut r = data.clone();
        let insert_result = self.coll.insert_one(Document::from(data), None).await?;

        if let Bson::ObjectId(ref id) = insert_result.inserted_id {
            r.id = Some(id.to_hex());
        } else {
            r.id = None;
        }

        Ok(r)
    }
}

impl Clone for ExperimentMongoRepo {
    fn clone(&self) -> Self {
        Self {
            coll: self.coll.clone(),
        }
    }
}
