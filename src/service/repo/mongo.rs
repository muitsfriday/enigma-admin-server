use async_trait::async_trait;
use bson::Bson;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use super::super::experiment as ExperimentService;
use super::ExperimentRepo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub active_interval: Option<Interval>,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub deleted_at: Option<DateTime<Utc>>,

    pub variations: Vec<Varience>,
    pub group_assign: GroupAssignment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

impl From<ExperimentService::Interval> for Interval {
    fn from(interval: ExperimentService::Interval) -> Self {
        Interval(interval.0, interval.1)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Varience {
    pub group: String,
    pub description: String,
    pub indicator: String,
    pub weight: i32,
    pub values: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupAssignment {
    pub strategy: String,
    pub persistent: String,
}

impl From<ExperimentService::Experiment> for Document {
    fn from(data: ExperimentService::Experiment) -> Self {
        let active_interval = match data.active_interval {
            Some(act) => Some(Interval::from(act)),
            None => None,
        };

        let variations = data
            .variations
            .iter()
            .map(|v| Varience {
                group: v.group.to_owned(),
                description: v.description.to_owned(),
                indicator: v.indicator.to_owned(),
                weight: v.weight.to_owned(),
                values: v.values.to_owned(),
            })
            .collect();

        Document {
            id: data.id,
            title: data.title,
            description: data.description,
            active_interval: active_interval,
            created_at: data.created_at,
            updated_at: data.updated_at,
            deleted_at: data.deleted_at,
            variations: variations,
            group_assign: GroupAssignment {
                strategy: data.group_assign.strategy,
                persistent: data.group_assign.persistent,
            },
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
    async fn save(
        &self,
        data: ExperimentService::Experiment,
    ) -> Result<ExperimentService::Experiment, Box<dyn Error>> {
        let now = Utc::now();
        let mut r = data.clone();
        let mut document = Document::from(data);

        // update the document timestamp
        document.updated_at = Some(now);
        if let None = document.created_at {
            document.created_at = Some(now);
        }

        let insert_result = self.coll.insert_one(document, None).await?;

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
