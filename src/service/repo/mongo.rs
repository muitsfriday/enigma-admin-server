use async_trait::async_trait;
use bson::Bson;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use mongodb::{bson::doc, bson::oid, Collection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

use super::super::experiment as ExperimentService;
use super::ExperimentRepo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub _id: Option<oid::ObjectId>,
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

    pub owner: Option<User>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub alias: String,
}

impl From<ExperimentService::Experiment> for Document {
    fn from(data: ExperimentService::Experiment) -> Self {
        println!("{:?}", data);
        let _id = data.id.map(|id| oid::ObjectId::parse_str(id).unwrap());
        let active_interval = data.active_interval.map(|act| Interval(act.0, act.1));
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
            _id: None,
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
            owner: data.owner.map(|u| User {
                id: u.id,
                username: u.username,
                alias: u.alias,
            }),
        }
    }
}

impl From<Document> for ExperimentService::Experiment {
    fn from(val: Document) -> Self {
        let id = match val._id {
            Some(oid) => Some(oid.to_string()),
            None => None,
        };

        let active_interval = match val.active_interval {
            Some(act) => Some(ExperimentService::Interval(act.0, act.1)),
            None => None,
        };

        let variations = val
            .variations
            .into_iter()
            .map(|v| ExperimentService::Varience {
                group: v.group,
                description: v.description,
                indicator: v.indicator,
                weight: v.weight,
                values: v.values,
            })
            .collect();

        ExperimentService::Experiment {
            id,
            title: val.title.clone(),
            description: val.description.clone(),
            active_interval,
            created_at: val.created_at.clone(),
            updated_at: val.updated_at.clone(),
            deleted_at: val.deleted_at.clone(),
            variations,
            group_assign: ExperimentService::GroupAssignment {
                strategy: val.group_assign.strategy.clone(),
                persistent: val.group_assign.persistent.clone(),
            },
            owner: val.owner.map(|u| ExperimentService::User {
                id: u.id,
                username: u.username,
                alias: u.alias,
            }),
        }
    }
}

/// ExperimentMongoRepo
/// is a struct managing about how to save and get the given experiment document to mongo.
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
    /// get method is for retrieving a specific document from mongodb.
    async fn get(&self, id: &str) -> Result<ExperimentService::Experiment, Box<dyn Error>> {
        let object_id = oid::ObjectId::parse_str(id)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::NotFound, err.to_string()))?;
        let doc = self.coll.find_one(doc! { "_id": object_id }, None).await?;
        println!("{:?}", doc);

        match doc {
            Some(doc) => Ok(ExperimentService::Experiment::from(doc)),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "document not found",
            ))),
        }
    }

    /// save method is for save the given document to mongo db.
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

        if let None = document._id {
            document._id = Some(oid::ObjectId::new());
        }

        let insert_result = self.coll.insert_one(document, None).await?;
        println!(
            "insert_result.inserted_id == {:?}",
            insert_result.inserted_id
        );

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
